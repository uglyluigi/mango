<template>
  <div class="library-view">
    <template v-for="([title, blobUrl], index) in imageBlobUrls" :key="title">
      <LibraryEntry
        @click="libraryEntryClicked(title)"
        :title="title"
        :imageUrl="blobUrl"
        :ordinal="index + 1"
      ></LibraryEntry>
    </template>
  </div>
</template>

<script>
import { get_resource_server_url, get_all_titles } from "@/invoke";
import LibraryEntry from "@/components/library/LibraryEntry.vue";

export default {
  name: "LibraryView",
  components: { LibraryEntry },
  data() {
    return {
      titles: [],
      resourceServerUrl: "",
      imageBlobUrls: [],
    };
  },
  created() {
    const titlesPromise = this.get_all_titles();
    const resourceServerUrlPromise = this.get_resource_server_url();
    Promise.all([titlesPromise, resourceServerUrlPromise]).then((vals) => {
      const [titles, resourceServerUrl] = vals;
      this.titles = titles;
      this.resourceServerUrl = resourceServerUrl;

      for (const title of titles) {
        this.urlFor(title);
      }
    });
  },
  methods: {
    get_all_titles,
    get_resource_server_url,
    urlFor(title) {
      fetch(`${this.resourceServerUrl}covers/${title}`).then((res) =>
        res.blob().then((blob) => {
          const url = URL.createObjectURL(blob);
          this.imageBlobUrls.push([title, url]);
        })
      );
    },
    libraryEntryClicked(title) {
      this.$router.push(`/chapters/${title}`);
    },
  },
};
</script>

<style scoped>
.library-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: var(--primary-bg);
  align-items: center;
}
</style>
