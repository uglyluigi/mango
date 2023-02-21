<template>
  <div
    v-for="chapter in this.chapterList"
    :key="chapter"
    @click="chapterView(chapter)"
  >
    Chapter {{ chapter }}
  </div>
  <BackButton></BackButton>
</template>

<script>
import { get_chapter_list } from "@/invoke";
import BackButton from "@/components/BackButton.vue";
export default {
  name: "ChapterView",
  created() {
    const chapterListPromise = get_chapter_list(this.title);
    chapterListPromise.then((list) => {
      for (const chap of list) {
        const [num, _] = chap;
        this.chapterList.push(num);
      }
      this.chapterList.sort((a, b) => {
        return a > b;
      });
    });
  },
  data() {
    return {
      chapterList: [],
      title: this.$route.params.series,
    };
  },
  methods: {
    get_chapter_list,
    chapterView(chapter) {
      this.$router.push({
        name: "view",
        params: { chapter, series: this.title },
      });
    },
  },
  components: { BackButton },
};
</script>

<style></style>
