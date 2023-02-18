<template>
  <div class="back-button" @mousedown="clicked">X</div>
</template>

<script>
export default {
  name: "BackButton",
  methods: {
    clicked() {
      this.$emit("clicked");
      this.$router.go(-1);
    },
  },
};
</script>

<style scoped>
.back-button {
  position: absolute;
  color: white;
  background-color: blueviolet;
  border-radius: 8px;
  padding: 1em 0.3em 1em 0.3em;
}
</style>
