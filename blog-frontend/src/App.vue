<template>
  <n-message-provider>
    <n-layout>
      <n-layout-header><Menu></Menu></n-layout-header>
      <n-layout-content content-style="padding: 24px;">
        <RouterView />
      </n-layout-content>
      <n-layout-footer>This is footer.</n-layout-footer>
    </n-layout>
    <n-back-top :right="100" />
  </n-message-provider>
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import Menu from "./components/Menu.vue";
import axios from "axios";
import { store } from "./store.ts";
import { Tag } from "./entity.ts";

onMounted(() => {
  axios
    .get("/api/tag/search_tags")
    .then((data) => {
      store.tags = data.data.reduce((acc: { [x: string]: any; }, tag: Tag) => {
          acc[tag.id] = tag;
          return acc;
        }, {});
      // console.log(store.tags);
    })
    .catch((err) => {
      console.log(err);
    });
});
</script>

<style scoped></style>
