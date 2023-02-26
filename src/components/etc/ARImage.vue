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
    height: String,
    width: String,
    src: {
      type: String,
      required: true,
    },
  },
  computed: {
    heightProp() {
      return this.height ? `${this.height}` : "auto";
    },
    widthProp() {
      return this.width ? `${this.width}` : "auto";
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
  height: v-bind(heightProp);
  width: v-bind(widthProp);
  margin: auto;
}

.centerer {
  margin: 0 auto 0 auto;
}

img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}
</style>
