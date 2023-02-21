<template>
  <div class="nav">
    <div class="icon-container"><img src="./../assets/mango.png" /></div>
    <h1>Mango</h1>
  </div>
</template>

<script>
export default {
  name: "App",
};
</script>

<style scoped>
.nav {
  position: fixed;
  top: 0;
  display: flex;
  flex-direction: row;
  align-items: center;
  width: 100%;
  height: 50px;
  background-color: var(--accent-bg);
  box-shadow: 0px 3px 10px gray;
  margin-bottom: 1em;
  opacity: 99%;
}

.icon-container {
  width: 50px;
  height: 50px;
}

img {
  height: 100%;
  width: 100%;
  object-fit: contain;
}

h1 {
  padding: 0;
  margin: 0;
  color: var(--accent-font-color);
}
</style>
