#!/usr/bin/env python3
import argparse
import datetime as dt
import json
import os
import sys
from typing import Any, Dict, List, Optional, Tuple
from urllib import error, request

API_BASE = "https://api.github.com"


def now_utc() -> dt.datetime:
    return dt.datetime.now(dt.timezone.utc)


def parse_ts(value: str) -> dt.datetime:
    return dt.datetime.fromisoformat(value.replace("Z", "+00:00"))


def clamp(value: float, low: float, high: float) -> float:
    return max(low, min(high, value))


class GitHubClient:
    def __init__(self, token: str):
        self.token = token.strip()

    def get(
        self,
        path: str,
        accept: str = "application/vnd.github+json",
        use_token: bool = True,
    ) -> Tuple[int, Dict[str, str], str]:
        url = f"{API_BASE}{path}"
        headers = {
            "Accept": accept,
            "X-GitHub-Api-Version": "2022-11-28",
            "User-Agent": "envlock-first-star-observer",
        }
        if use_token and self.token:
            headers["Authorization"] = f"Bearer {self.token}"

        req = request.Request(url, headers=headers, method="GET")
        try:
            with request.urlopen(req, timeout=20) as resp:
                body = resp.read().decode("utf-8")
                return resp.status, dict(resp.headers.items()), body
        except error.HTTPError as exc:
            body = exc.read().decode("utf-8", errors="replace")
            return exc.code, dict(exc.headers.items()), body


def sum_recent(daily_items: List[Dict[str, Any]], days: int) -> Tuple[int, int]:
    cutoff = now_utc() - dt.timedelta(days=days)
    total_count = 0
    total_uniques = 0
    for item in daily_items:
        ts = item.get("timestamp")
        if not ts:
            continue
        try:
            item_time = parse_ts(ts)
        except ValueError:
            continue
        if item_time >= cutoff:
            total_count += int(item.get("count", 0))
            total_uniques += int(item.get("uniques", 0))
    return total_count, total_uniques


def to_int(value: Any, default: int = 0) -> int:
    try:
        return int(value)
    except (TypeError, ValueError):
        return default


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Observe low-sample first-star funnel from GitHub API."
    )
    parser.add_argument(
        "--repo",
        default=os.getenv("GITHUB_REPOSITORY", ""),
        help="owner/repo, default from GITHUB_REPOSITORY",
    )
    parser.add_argument(
        "--days",
        type=int,
        default=7,
        help="window days (1-14 recommended), default 7",
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="output JSON only",
    )
    args = parser.parse_args()

    repo = args.repo.strip()
    if not repo or "/" not in repo:
        print(
            "ERROR: missing --repo (owner/repo) and GITHUB_REPOSITORY is not set.",
            file=sys.stderr,
        )
        return 2

    days = max(1, min(args.days, 14))
    token = os.getenv("GITHUB_TOKEN", "").strip()
    client = GitHubClient(token=token)

    warnings: List[str] = []
    traffic_available = bool(token)

    stars_total: Optional[int] = None
    status, _, body = client.get(f"/repos/{repo}", use_token=bool(token))
    if status == 200:
        try:
            repo_data = json.loads(body)
            stars_total = to_int(repo_data.get("stargazers_count"), 0)
        except json.JSONDecodeError:
            warnings.append("repo metadata parse failed")
    else:
        warnings.append(f"repo metadata unavailable (HTTP {status})")

    stars_new_7d: Optional[int] = None
    status, _, body = client.get(
        f"/repos/{repo}/stargazers?per_page=100",
        accept="application/vnd.github.star+json",
        use_token=bool(token),
    )
    if status == 200:
        try:
            stars = json.loads(body)
            cutoff = now_utc() - dt.timedelta(days=days)
            stars_new_7d = 0
            for item in stars:
                starred_at = item.get("starred_at")
                if not starred_at:
                    continue
                try:
                    if parse_ts(starred_at) >= cutoff:
                        stars_new_7d += 1
                except ValueError:
                    continue
        except json.JSONDecodeError:
            warnings.append("stargazers parse failed")
    else:
        warnings.append(f"stargazers unavailable (HTTP {status})")

    views_7d_count: Optional[int] = None
    views_7d_uniques_proxy: Optional[int] = None
    clones_7d_count: Optional[int] = None
    clones_7d_uniques_proxy: Optional[int] = None

    if traffic_available:
        status, _, body = client.get(f"/repos/{repo}/traffic/views?per=day", use_token=True)
        if status == 200:
            try:
                views_data = json.loads(body)
                views_items = views_data.get("views", [])
                views_7d_count, views_7d_uniques_proxy = sum_recent(views_items, days)
            except json.JSONDecodeError:
                warnings.append("traffic views parse failed")
        else:
            warnings.append(f"traffic views unavailable (HTTP {status})")
            traffic_available = False

        status, _, body = client.get(f"/repos/{repo}/traffic/clones?per=day", use_token=True)
        if status == 200:
            try:
                clones_data = json.loads(body)
                clones_items = clones_data.get("clones", [])
                clones_7d_count, clones_7d_uniques_proxy = sum_recent(clones_items, days)
            except json.JSONDecodeError:
                warnings.append("traffic clones parse failed")
        else:
            warnings.append(f"traffic clones unavailable (HTTP {status})")
            traffic_available = False
    else:
        warnings.append("GITHUB_TOKEN missing; traffic metrics downgraded")

    components: Dict[str, Dict[str, Any]] = {
        "exposure": {"max": 20, "score": None, "available": views_7d_count is not None},
        "reach": {"max": 20, "score": None, "available": views_7d_uniques_proxy is not None},
        "understand": {
            "max": 20,
            "score": None,
            "available": (views_7d_count is not None and views_7d_uniques_proxy is not None),
        },
        "try": {"max": 20, "score": None, "available": clones_7d_uniques_proxy is not None},
        "approve": {"max": 10, "score": None, "available": stars_new_7d is not None},
        "star": {"max": 10, "score": None, "available": stars_total is not None},
    }

    if views_7d_count is not None:
        components["exposure"]["score"] = round(clamp((views_7d_count / 30.0) * 20.0, 0, 20), 1)

    if views_7d_uniques_proxy is not None:
        components["reach"]["score"] = round(
            clamp((views_7d_uniques_proxy / 5.0) * 20.0, 0, 20), 1
        )

    if views_7d_count is not None and views_7d_uniques_proxy is not None:
        depth = views_7d_count / max(views_7d_uniques_proxy, 1)
        components["understand"]["score"] = round(
            clamp(((depth - 1.0) / 0.8) * 20.0, 0, 20), 1
        )
    else:
        depth = None

    if clones_7d_uniques_proxy is not None:
        components["try"]["score"] = round(
            clamp((clones_7d_uniques_proxy / 4.0) * 20.0, 0, 20), 1
        )

    if stars_new_7d is not None:
        components["approve"]["score"] = round(clamp(stars_new_7d * 10.0, 0, 10), 1)

    if stars_total is not None:
        components["star"]["score"] = 10.0 if stars_total >= 1 else 0.0

    available_max = 0.0
    raw_score = 0.0
    for comp in components.values():
        if comp["available"] and comp["score"] is not None:
            available_max += float(comp["max"])
            raw_score += float(comp["score"])

    weekly_score = round((raw_score / available_max) * 100.0) if available_max > 0 else 0
    mode = "full" if available_max >= 100 else "degraded"
    confidence = "high" if mode == "full" else "low"

    report = {
        "repo": repo,
        "window_days": days,
        "token_present": bool(token),
        "mode": mode,
        "confidence": confidence,
        "metrics": {
            "views_count_7d": views_7d_count,
            "visitors_7d_proxy": views_7d_uniques_proxy,
            "clones_count_7d": clones_7d_count,
            "clones_uniques_7d_proxy": clones_7d_uniques_proxy,
            "stars_total": stars_total,
            "stars_new_7d": stars_new_7d,
            "depth_proxy": round(depth, 3) if depth is not None else None,
        },
        "funnel_scores": {
            k: {"score": v["score"], "max": v["max"], "available": v["available"]}
            for k, v in components.items()
        },
        "weekly_score_0_100": weekly_score,
        "warnings": warnings,
    }

    if args.json:
        print(json.dumps(report, ensure_ascii=False, indent=2))
        return 0

    def fmt(value: Any) -> str:
        return "n/a" if value is None else str(value)

    print(
        f"FIRST_STAR_OBSERVER repo={repo} window={days}d mode={mode} token={'yes' if token else 'no'}"
    )
    for warning in warnings:
        print(f"WARN {warning}")

    print(
        "METRICS "
        f"views={fmt(views_7d_count)} "
        f"visitors={fmt(views_7d_uniques_proxy)} "
        f"clones={fmt(clones_7d_uniques_proxy)} "
        f"stars_total={fmt(stars_total)} "
        f"stars_new={fmt(stars_new_7d)} "
        f"depth={fmt(round(depth, 3) if depth is not None else None)}"
    )

    print(
        "FUNNEL "
        f"exposure={fmt(components['exposure']['score'])}/20 "
        f"reach={fmt(components['reach']['score'])}/20 "
        f"understand={fmt(components['understand']['score'])}/20 "
        f"try={fmt(components['try']['score'])}/20 "
        f"approve={fmt(components['approve']['score'])}/10 "
        f"star={fmt(components['star']['score'])}/10"
    )

    print(f"SCORE weekly={weekly_score}/100 confidence={confidence}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
