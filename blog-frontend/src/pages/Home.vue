<template>
  <n-space vertical>
    <n-card
      v-for="(post, index) in posts"
      :key="index"
      :title="post.title"
      hoverable
      :segmented="{
        content: true,
        footer: 'soft',
      }"
    >
      <n-space>
        <span>点击数：{{ post.hit }}</span>
        <span
          >发布时间：<n-time
            :time="post.publish_time * 1000"
            time-zone="Asia/Shanghai"
          ></n-time
        ></span>
        <span>字符长度：{{ post.words_len }}</span>
      </n-space>
      <template #footer>
        {{ post.summary }}
      </template>
      <template #action>
        <n-space>
          Tags：
          <n-tag
            v-for="(tag, index) in post.tag_id"
            :key="index"
            :bordered="false"
            type="success"
          >
            {{ store.tags[tag].name }}
          </n-tag></n-space
        ></template
      >
    </n-card>
  </n-space>
</template>

<script setup lang="ts">
import axios from "axios";
import { useMessage } from "naive-ui";
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import type { Post } from "../entity.ts";
import { store } from "../store.ts";

const message = useMessage();
const route = useRoute();
const page = ref<number>(1);
const current_page = ref<number>(0);
const page_total = ref<number>(0);

const posts = ref<Array<Post>>([]);

onMounted(() => {
  let p = route.query.page as unknown as number;
  if (p !== 0) {
    page.value = p;
  }
  fetchPosts();
});

const fetchPosts = async () => {
  await axios
    .get("/api/post/search_posts", {
      params: {
        page: page.value,
      },
    })
    .then((data) => {
      posts.value = data.data.posts;
      current_page.value = data.data.page;
      page_total.value = data.data.page_total;
    })
    .catch((err) => {
      message.error(err.message);
    });
};
</script>

<style scoped></style>
