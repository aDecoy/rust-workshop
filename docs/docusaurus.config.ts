import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';
// This runs in Node.js - Don't use client-side code here (browser APIs, JSX.)

const config: Config = {
  title: "Rust for .NET Developers",
  tagline: "The #1 workshop for .NET developers learning Rust",
  favicon: "img/favicon.png",

  // Set the production url of your site here
  url: "https://rustfor.net",
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: "/",

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: "jeastham1993", // Usually your GitHub org/user name.
  projectName: "rust-for-dotnet-devs-workshop", // Usually your repo name.

  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",

  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },
  plugins: [
    [
      "@docusaurus/plugin-google-gtag",
      {
        trackingID: "G-YPEKK2EK7N",
        anonymizeIP: true,
      },
    ],
  ],

  presets: [
    [
      "classic",
      {
        docs: {
          sidebarPath: "./sidebars.ts",
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            "https://github.com/jeastham1993/rust-for-dotnet-devs-workshop",
        },
        theme: {
          customCss: "./src/css/custom.css",
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    // Replace with your project's social card
    //image: 'img/docusaurus-social-card.jpg',
    navbar: {
      title: "Rust for .NET",
      logo: {
        alt: "Rust for .NET",
        src: "img/logo.png",
      },
      items: [
        {
          type: "docSidebar",
          sidebarId: "tutorialSidebar",
          position: "left",
          label: "Workshop",
        },
        {
          href: "https://github.com/jeastham1993/rust-for-dotnet-devs-workshop",
          label: "GitHub",
          position: "right",
        },
      ],
    },
    footer: {
      style: "dark",
      links: [
        {
          title: "Docs",
          items: [
            {
              label: "Workshop",
              to: "/docs/intro",
            },
          ],
        },
        {
          title: "Community",
          items: [
            {
              label: "Bsky",
              href: "https://bsky.app/profile/jameseastham.co.uk",
            },
          ],
        },
        {
          title: "More",
          items: [
            {
              label: "Subscribe for Updates",
              href: "https://magic.beehiiv.com/v1/c0b70af5-bea3-4bc7-a1f9-ca9eaf4a56e9",
            },
            {
              label: "GitHub",
              href: "https://github.com/jeastham1993/rust-for-dotnet-devs-workshop",
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} James Eastham. Built with Docusaurus.`,
    },
    prism: {
      theme: prismThemes.oneLight,
      darkTheme: prismThemes.oneDark,
      additionalLanguages: ["rust", "csharp", "bash"],
    },
    algolia: {
      appId: "1ZNZB6R10K",

      // Public API key: it is safe to commit it
      apiKey: "c26f587b58b7983e04ab82b1c5e56c91",

      indexName: "rustfor",
    },
  } satisfies Preset.ThemeConfig,
};

export default config;