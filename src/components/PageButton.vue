<template>
  <div class="pageButton" @mousedown="click">
    <template v-if="direction === 'left'">&lt;</template>
    <template v-else>&gt;</template>
  </div>
</template>

<script>
export default {
  name: "PageButton",
  props: {
    direction: String,
  },
  methods: {
    click() {
      this.$emit("clicked");
    },
  },
};
</script>

<style scoped>
.pageButton {
  background-color: grey;
  width: 50px;
  min-height: 10em;
  margin-top: auto;
  margin-bottom: auto;
  display: flex;
  flex-direction: column;
  justify-content: center;
}
</style>
