export interface Tag {
  id: number;
  name: string;
}

export interface Post {
  id: number;
  hit: number;
  publish_time: number;
  update_time: number;
  tag_id: Array<number>;
  words_len: number;
  summary: string;
  title: string;
}
