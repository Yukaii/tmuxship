import { defineConfig } from 'vitepress';

export default defineConfig({
  title: 'tmuxship',
  description: 'A Starship-to-tmux adapter with legendary built-in themes',
  base: '/tmuxship/',
  head: [
    ['link', { rel: 'icon', href: 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><text y=".9em" font-size="90">🚀</text></svg>' }],
    ['link', { rel: 'preconnect', href: 'https://cdn.jsdelivr.net' }]
  ],
  themeConfig: {
    logo: '🚀',
    siteTitle: 'tmuxship',
    nav: [
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'Themes Previewer', link: '/themes/' },
      { text: 'Configuration', link: '/guide/configuration' },
      { text: 'CLI', link: '/guide/cli' },
    ],
    sidebar: [
      {
        text: 'Getting Started',
        items: [
          { text: 'Introduction', link: '/guide/getting-started' },
          { text: 'How It Works', link: '/guide/how-it-works' },
          { text: 'Configuration', link: '/guide/configuration' },
          { text: 'CLI Reference', link: '/guide/cli' },
          { text: 'tmux Variables', link: '/guide/variables' },
        ]
      },
      {
        text: 'Themes',
        items: [
          { text: 'Theme Previewer', link: '/themes/' },
          { text: 'Contributing a Theme', link: '/themes/contributing' },
        ]
      }
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/Yukaii/tmuxship' }
    ],
    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2024-present Yukai Huang & Contributors'
    },
    search: {
      provider: 'local'
    }
  }
});
