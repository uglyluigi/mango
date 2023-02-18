<template>
  <template v-if="currentImage !== null">
    <div class="content-container">
      <div class="page-status">
        {{ currentImageNum + 1 }} / {{ this.images.size }}
      </div>
      <PageButton
        class="left"
        @clicked="clickLeft()"
        direction="left"
      ></PageButton>
      <div class="image-container">
        <img :src="currentImage" />
      </div>
      <PageButton
        class="right"
        @clicked="clickRight()"
        direction="right"
      ></PageButton>
      <BackButton></BackButton>
    </div>
  </template>
</template>

<script>
import { get_resource_server_url } from "../../invoke";
import PageButton from "../PageButton.vue";
import BackButton from "../BackButton.vue";
export default {
  name: "ChapterView",
  data() {
    return {
      images: new Map(),
      chapter: this.$route.params.chapter,
      series: this.$route.params.series,
      currentImage: null,
      currentImageNum: 0,
    };
  },
  created() {
    get_resource_server_url().then((url) => {
      fetch(`${url}image_count/${this.series}/${this.chapter}`).then((res) => {
        res.text().then((numImages) => {
          const n = parseInt(numImages);
          this.numImages = n;
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
          Promise.all(promises).then(() => {
            this.currentImage = this.images.get(0);
          });
        });
      });
    });
  },
  methods: {
    clickLeft() {
      if (this.currentImageNum !== 0) {
        this.currentImageNum--;
        this.updateCurrentImage();
      }
    },
    clickRight() {
      if (this.currentImageNum < this.numImages - 1) {
        this.currentImageNum++;
        this.updateCurrentImage();
      }
    },
    updateCurrentImage() {
      this.currentImage = this.images.get(this.currentImageNum);
    },
  },
  computed: {},
  components: { PageButton, BackButton },
};
</script>

<style>
.content-container {
  display: flex;
  height: 100%;
  justify-content: space-between;
  background-color: black;
}

.image-container {
  width: 100%;
}

.image-container > img {
  height: 100%;
  width: 100%;
  object-fit: contain;
}

.page-status {
  position: fixed;
  bottom: 0;
  left: 0;
  margin: 1em;
  color: white;
}
</style>
