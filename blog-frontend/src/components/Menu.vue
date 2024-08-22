<template>
  <n-split :default-size="1">
    <template #1>
      <n-menu mode="horizontal" :options="menuOptions" responsive />
    </template>
  </n-split>
</template>
<script setup lang="ts">
import { h } from "vue";
import type { MenuOption } from "naive-ui";
import { RouterLink } from "vue-router";

const menuOptions: MenuOption[] = [
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            path: "/",
            query: {
                page: 1,
            }
          },
        },
        { default: () => "主页" }
      ),
    key: "home",
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            path: "/about",
          },
        },
        { default: () => "关于" }
      ),
    key: "about",
  },
];
</script>
