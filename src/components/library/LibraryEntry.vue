<template>
  <div class="library-entry">
    <div class="library-entry-title">
      {{ title }}
    </div>
    <div class="library-entry-info">
      <div class="read-status">{{ readOrUnread }}</div>
      <div>Chapters: {{ numChapters }}</div>
      <div>Tags: Cool, Nice, Silly</div>
    </div>
    <div class="image-container">
      <ARImage :width="300" :src="imageUrl"></ARImage>
    </div>
  </div>
</template>

<script>
import ARImage from "@/components/etc/ARImage.vue";

export default {
  name: "LibraryEntry",
  components: { ARImage },
  props: {
    imageUrl: {
      type: String,
      required: true,
    },
    title: {
      type: String,
      required: true,
    },
    read: Boolean,
    numChapters: Number,
    tags: Array,
  },
  computed: {
    readOrUnread() {
      return this.read ? `Read` : `Unread`;
    },
  },
};
</script>

<style scoped>
.library-entry {
  display: grid;
  flex-direction: column;
  grid-template-columns: 0.5fr 0.5fr;
  justify-content: space-between;
  background-color: var(--secondary-bg);
  border-radius: 8px;
  margin-bottom: 3em;
  width: 80%;
  height: 80%;
  max-width: 800px;
  padding: 1em;
}

.library-entry-title {
  font-family: "Varela Round";
  font-size: 3rem;
  grid-column: 1 / 3;
  padding: 0.3em;
}

.library-entry-info {
  margin: auto;
  word-break: break-word;
  font-size: 1.2m;
  color: black;
  padding: 1em;
  border-radius: 8px;
}

.read-status {
  background-color: var(--primary-bg);
  border-radius: 8px;
  padding: 0.3em 0em 0.3em 0em;
}
</style>
