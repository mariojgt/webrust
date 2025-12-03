import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "WebRust",
  description: "A Laravel-inspired Rust mini framework",
  base: '/webrust/', // Assuming repository name is 'webrust'. Change if different.
  lastUpdated: true,
  cleanUrls: false,
  sitemap: {
    hostname: 'https://mariojgt.github.io/webrust'
  },
  head: [
    ['link', { rel: 'preconnect', href: 'https://fonts.googleapis.com' }],
    ['link', { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' }],
    ['link', { href: 'https://fonts.googleapis.com/css2?family=Figtree:ital,wght@0,300..900;1,300..900&display=swap', rel: 'stylesheet' }],
    ['link', { rel: 'icon', href: '/webrust/favicon.ico' }],
    ['meta', { name: 'theme-color', content: '#22c55e' }],
    ['meta', { name: 'og:type', content: 'website' }],
    ['meta', { name: 'og:locale', content: 'en' }],
    ['meta', { name: 'og:site_name', content: 'WebRust' }],
    ['meta', { name: 'og:image', content: 'https://mariojgt.github.io/webrust/webrust-hero.png' }],
    ['meta', { name: 'twitter:card', content: 'summary_large_image' }],
    ['meta', { name: 'twitter:image', content: 'https://mariojgt.github.io/webrust/webrust-hero.png' }],
  ],
  transformHead({ pageData }) {
    const title = pageData.title ? `${pageData.title} | WebRust` : 'WebRust'
    const description = pageData.description || "A Laravel-inspired Rust mini framework"
    const url = `https://mariojgt.github.io/webrust/${pageData.relativePath.replace(/((^|\/)index)?\.md$/, '$2')}`

    return [
      ['meta', { property: 'og:title', content: title }],
      ['meta', { property: 'og:description', content: description }],
      ['meta', { property: 'og:url', content: url }],
      ['meta', { name: 'twitter:title', content: title }],
      ['meta', { name: 'twitter:description', content: description }],
    ]
  },
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/GUIDE' },
      { text: 'Orbit ORM', link: '/ORBIT' }
    ],

    sidebar: [
      {
        text: 'Getting Started',
        items: [
          { text: 'Introduction', link: '/GUIDE' },
          { text: 'Application Lifecycle', link: '/LIFECYCLE' },
          { text: 'Basics', link: '/BASICS' },
          { text: 'Routing', link: '/ROUTES' },
          { text: 'Migrations', link: '/MIGRATIONS' },
          { text: 'Deployment & Docker', link: '/DEPLOYMENT' },
        ]
      },
      {
        text: 'Core Concepts',
        items: [
          { text: 'Authentication', link: '/AUTH' },
          { text: 'Orbit ORM', link: '/ORBIT' },
          { text: 'Database Connections', link: '/DATABASE' },
          { text: 'Inertia.js', link: '/INERTIA' },
          { text: 'Caching', link: '/CACHE' },
          { text: 'Validation', link: '/VALIDATION' },
          { text: 'HTTP Client', link: '/HTTP' },
          { text: 'Logging', link: '/LOGGING' },
          { text: 'Storage', link: '/STORAGE' },
          { text: 'Packages', link: '/PACKAGES' },
          { text: 'Custom Commands', link: '/COMMANDS' },
        ]
      },
      {
        text: 'Deep Dive',
        items: [
          { text: 'Mail', link: '/MAIL' },
          { text: 'Queues', link: '/QUEUES' },
          { text: 'Scheduler', link: '/SCHEDULER' },
          { text: 'CSRF Protection', link: '/CSRF' },
          { text: 'Debugging', link: '/DEBUG_QUICK_REF' },
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/mariojgt/webrust' }
    ],

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2025 WebRust'
    },

    search: {
      provider: 'local'
    }
  }
})
