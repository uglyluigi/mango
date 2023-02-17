<template>
  <Suspense>
    <div v-for="[num, url] in this.images" :key="num"><img :src="url" /></div>
    <template #fallback> Loading... </template>
  </Suspense>
</template>

<script>
import { title } from "process";
import { get_resource_server_url } from "../../invoke";
export default {
  name: "ChapterView",
  data() {
    return {
      images: new Map(),
      chapter: this.$route.params.chapter,
      series: this.$route.params.series,
    };
  },
  created() {
    get_resource_server_url().then((url) => {
      fetch(`${url}image_count/${this.series}/${this.chapter}`).then((res) => {
        res.text().then((numImages) => {
          const n = parseInt(numImages);
          let promises = [];

          for (let i = 0; i < n; i++) {
            promises.push(
              fetch(
                `${url}chapter_image/${this.series}/${this.chapter}/${i}`
              ).then(async (res) => {
                this.images.set(i, URL.createObjectURL(await res.blob()));
              })
            );
          }

          Promise.all(promises).then(() =>
            console.log("chapter images loaded: " + this.images)
          );
        });
      });
    });
  },
};
</script>

<style></style>
