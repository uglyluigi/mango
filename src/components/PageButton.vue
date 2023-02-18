<template>
  <div class="pageButton" @mousedown="click">
    <div v-if="direction === 'left'">&lt;</div>
    <div v-else>&gt;</div>
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
	height: 100%;
	background-color: black;
	width: 50px;
}
</style>
