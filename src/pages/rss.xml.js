import rss from '@astrojs/rss';
import { getCollection } from 'astro:content';

export async function GET(context) {
  const posts = await getCollection('posts', ({ data }) => !data.draft);

  // 날짜 기준 내림차순 정렬
  const sortedPosts = posts.sort((a, b) => {
    return new Date(b.data.date).getTime() - new Date(a.data.date).getTime();
  });

  return rss({
    title: "Minkyu의 기술 블로그",
    description: "프론트엔드 개발에 대한 인사이트와 경험을 공유합니다",
    site: context.site,
    items: sortedPosts.map((post) => ({
      title: post.data.title,
      pubDate: new Date(post.data.date),
      description: post.data.description,
      link: `/posts/${post.slug}/`,
      categories: post.data.tags || [],
      author: "한민규",
    })),
    customData: `<language>ko-KR</language>`,
  });
}
