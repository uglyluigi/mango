<template>
  <div class="library-entry">
    <div class="library-entry-info">
      <div class="library-entry-title">
        {{ title }}
      </div>
      <div>Unread</div>
      <div>Chapters: 69</div>
      <div>Tags: Cool, Nice, Silly</div>
    </div>
    <div class="image-container">
      <ARImage :width="300" :src="imageUrl"></ARImage>
    </div>
  </div>
</template>

<script>
import ARImage from "../etc/ARImage.vue";

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
  },
};
</script>

<style scoped>
.library-entry {
  display: grid;
  flex-direction: column;
  grid-template-columns: 0.5fr 0.5fr;
  justify-content: space-between;
  background-color: rgb(253, 254, 238);
  border-radius: 8px;
  margin-bottom: 3em;
  max-height: 50%;
}

.library-entry-title {
  font-family: "Varela Round";
  font-size: 3rem;
}

.entry-info {
  display: flex;
  align-items: center;
}

.library-entry-info {
  margin: auto;
  word-break: break-word;
}
</style>
