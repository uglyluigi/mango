<template>
  <div class="container">
    <div class="centerer">
      <img :src="src" />
    </div>
  </div>
</template>

<script>
export default {
  name: "ARImage",
  props: {
    height: Number,
    width: Number,
    src: {
      type: String,
      required: true,
    },
  },
  computed: {
    heightProp() {
      return this.height ? `${this.height}px` : "auto";
    },
    widthProp() {
      return this.width ? `${this.width}px` : "auto";
    },
  },
};
</script>

<style scoped>
.container {
  display: flex;
  justify-content: space-between;
  max-height: v-bind(heightProp);
  max-width: v-bind(widthProp);
  margin: auto;
}

img {
  width: 100%;
  height: 100%;
  object-fit: fill;
}
</style>
