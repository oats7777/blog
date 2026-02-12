// @ts-check
import { defineConfig } from 'astro/config';
import sitemap from '@astrojs/sitemap';
import remarkGfm from 'remark-gfm';
import wasm from 'vite-plugin-wasm';

// https://astro.build/config
export default defineConfig({
  markdown: {
    remarkPlugins: [remarkGfm],
  },
  site: 'https://www.fe-minkyu.dev',
  integrations: [
    sitemap({
      changefreq: 'weekly',
      priority: 0.7,
      lastmod: new Date(),
    }),
  ],
  vite: {
    plugins: [wasm()],
    resolve: {
      alias: {
        '@': '/src',
        '@toys': '/toys',
      },
    },
    optimizeDeps: {
      exclude: ['rust-canvas'],
    },
  },
});
