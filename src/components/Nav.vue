<template>
  <div class="nav"><img src="./../assets/mango.png" /></div>
</template>

<script>
export default {
  name: "App",
  components: {},
};
</script>

<style scoped>
.nav {
  display: flex;
  flex-direction: row;
  justify-content: flex-start;
  width: 100%;
  height: 50px;
  background-color: aqua;
}

img {
  height: 100%;
  width: 100%;
  object-fit: contain;
}
</style>
