import { defineConfig } from "vitepress";

export default defineConfig({
  title: "envlock",
  description: "Deterministic environment sessions from JSON profiles.",
  base: "/envlock/",
  cleanUrls: true,
  lastUpdated: true,
  themeConfig: {
    nav: [
      { text: "Tutorial", link: "/tutorials/quick-start" },
      { text: "How-to", link: "/how-to/install" },
      { text: "Reference", link: "/reference/cli" },
      { text: "Explanation", link: "/explanation/design-boundaries" },
      { text: "GitHub", link: "https://github.com/PerishCode/envlock" }
    ],
    sidebar: [
      {
        text: "Tutorial",
        items: [{ text: "Quick Start", link: "/tutorials/quick-start" }]
      },
      {
        text: "How-to",
        items: [
          { text: "Install", link: "/how-to/install" },
          { text: "Use Profiles", link: "/how-to/use-profiles" },
          { text: "Run Command Mode", link: "/how-to/command-mode" },
          { text: "Update and Uninstall", link: "/how-to/update-and-uninstall" }
        ]
      },
      {
        text: "Reference",
        items: [
          { text: "CLI", link: "/reference/cli" },
          { text: "Profile Format", link: "/reference/profile" },
          { text: "Environment Variables", link: "/reference/environment" },
          { text: "Release Pipeline", link: "/reference/release" }
        ]
      },
      {
        text: "Explanation",
        items: [
          { text: "Design Boundaries", link: "/explanation/design-boundaries" },
          { text: "Troubleshooting", link: "/explanation/troubleshooting" },
          { text: "Support Policy", link: "/explanation/support-policy" }
        ]
      }
    ],
    outline: {
      level: [2, 3],
      label: "On this page"
    },
    search: {
      provider: "local"
    },
    editLink: {
      pattern: "https://github.com/PerishCode/envlock/edit/main/docs/:path",
      text: "Edit this page on GitHub"
    },
    socialLinks: [{ icon: "github", link: "https://github.com/PerishCode/envlock" }],
    footer: {
      message: "Built with VitePress",
      copyright: "Copyright © 2026 PerishCode"
    }
  }
});
