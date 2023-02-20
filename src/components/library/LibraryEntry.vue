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
      <img :src="imageUrl" />
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
    ordinal: {
      type: Number,
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

.image-container {
  width: 100%;
  height: 90%;
}

img {
  object-fit: contain;
  height: 100%;
}

.library-entry-title {
  font-family: "Varela Round";
  font-size: 3rem;
}

.ordinal {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
.ordinal-container {
  background-color: #2bc29f;
  width: 50%;
  aspect-ratio: 1/1;
  border-radius: 50%;
  font: "Varela Round";
  color: rgb(253, 254, 238);
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
