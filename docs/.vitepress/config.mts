import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Stark demo",
  description: "Stark demo",
  base: '/stark-demo/',
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Giới thiệu', link: '/introduction' }
    ],

    sidebar: [
      {
        text: 'Docs',
        items: [
          { text: 'Giới thiệu', link: '/introduction' },
          { text: 'Đại số', link: '/algebra' },
          { text: 'Đa thức đơn biến', link: '/univariate' },
          { text: 'Đa thức đa biến', link: '/multivariate' },
          { text: 'Luồng chứng minh và Fiat-Shamir', link: '/ip' },
          { text: 'Cây Merkle và các Hoạt động', link: '/merkle' },
          { text: 'FRI', link: '/fri' },
          { text: 'Cây Merkle và các Hoạt động', link: '/merkle' }

        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/lddogs/stark-demo' }
    ]
  },

  // Add KaTeX support
  head: [
    ['link', { rel: 'stylesheet', href: 'https://cdn.jsdelivr.net/npm/katex@0.16.4/dist/katex.min.css' }],
    ['script', { src: 'https://cdn.jsdelivr.net/npm/katex@0.16.4/dist/katex.min.js' }],
  ],

  markdown: {
    math: true
  }
})
