<template>
  <div class="back-button" @mousedown="clicked">
    <XIcon />
  </div>
</template>

<script>
import XIcon from "./icon/XIcon.vue";

export default {
  name: "BackButton",
  components: { XIcon },
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
  background-color: black;
  border-radius: 8px;
  width: 30px;
  height: 30px;
  padding: 0.5em;
  display: flex;
  justify-content: center;
  box-shadow: 0px 2px 8px gray;
  cursor: pointer;
  transition: background-color 0.15s ease-in-out;
}

:deep(path) {
  transition: fill 0.15s ease-in-out;
}

:deep(.x-icon > path) {
  fill: white;
}

.back-button:hover :deep(.x-icon > path) {
  fill: black;
}

.back-button:hover {
  background-color: white;
}

.back-button > img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}
</style>
