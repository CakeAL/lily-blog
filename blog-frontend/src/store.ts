import { reactive } from "vue";
import { Tag } from "./entity";

export const store = reactive({
  tags: [] as Array<Tag>,
});
