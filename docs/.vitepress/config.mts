import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "WebRust",
  description: "A Laravel-inspired Rust mini framework",
  base: '/webrust/', // Assuming repository name is 'webrust'. Change if different.
  head: [
    ['link', { rel: 'preconnect', href: 'https://fonts.googleapis.com' }],
    ['link', { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' }],
    ['link', { href: 'https://fonts.googleapis.com/css2?family=Figtree:ital,wght@0,300..900;1,300..900&display=swap', rel: 'stylesheet' }]
  ],
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
          { text: 'Basics', link: '/BASICS' },
          { text: 'Routing', link: '/ROUTES' },
          { text: 'Migrations', link: '/MIGRATIONS' },
        ]
      },
      {
        text: 'Core Concepts',
        items: [
          { text: 'Authentication', link: '/AUTH' },
          { text: 'Orbit ORM', link: '/ORBIT' },
          { text: 'Inertia.js', link: '/INERTIA' },
          { text: 'Caching', link: '/CACHE' },
          { text: 'Validation', link: '/VALIDATION' },
          { text: 'HTTP Client', link: '/HTTP' },
          { text: 'Logging', link: '/LOGGING' },
          { text: 'Storage', link: '/STORAGE' },
          { text: 'Packages', link: '/PACKAGES' },
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
