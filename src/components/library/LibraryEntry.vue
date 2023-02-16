<template>
  <div class="library-entry">
    <div class="image-container">
      <img :src="imageUrl" />
    </div>
    <div class="library-entry-title">
      {{ title }}
    </div>
  </div>
</template>

<script>
export default {
  name: "LibraryEntry",
  components: {},
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
  display: flex;
  flex-direction: column;
  width: 100%;
  min-height: fit-content;
  justify-content: space-between;
}

.image-container {
  width: 100%;
  height: 90%;
}

img {
  object-fit: contain;
  width: 100%;
  height: 100%;
}
</style>
