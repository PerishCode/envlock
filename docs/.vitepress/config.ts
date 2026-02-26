import { defineConfig } from "vitepress";

export default defineConfig({
  title: "envlock",
  description: "Install and manage envlock with install.sh and self-update",
  base: "/envlock/",
  themeConfig: {
    nav: [
      { text: "Guide", link: "/install" },
      { text: "Release", link: "/release" }
    ],
    sidebar: [
      {
        text: "Guide",
        items: [
          { text: "Install", link: "/install" },
          { text: "Update", link: "/update" },
          { text: "Uninstall", link: "/uninstall" },
          { text: "Support Policy", link: "/support-policy" },
          { text: "Release", link: "/release" }
        ]
      }
    ]
  }
});
