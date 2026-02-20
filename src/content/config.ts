import { defineCollection, z } from 'astro:content';

const postsCollection = defineCollection({
  type: 'content',
  schema: z.object({
    title: z.string(),
    description: z.string(),
    date: z.string(), // YYYY-MM-DD 형식의 문자열
    order: z.number().default(0), // 같은 날짜 내 정렬 순서 (낮을수록 먼저 표시)
    category: z.string().optional(),
    tags: z.array(z.string()).default([]),
    readTime: z.string().optional(),
    draft: z.boolean().default(false),
  }),
});

const toysCollection = defineCollection({
  type: 'content',
  schema: z.object({
    title: z.string(),
    description: z.string(),
    date: z.string(),
    embedUrl: z.string(),
    techStack: z.string(), // React, Svelte, Solid, Rust+WASM 등
    tags: z.array(z.string()).default([]),
    draft: z.boolean().default(false),
  }),
});

export const collections = {
  posts: postsCollection,
  toys: toysCollection,
};
