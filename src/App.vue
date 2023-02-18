<template>
  <Nav />
  <router-view></router-view>
</template>

<script>
import Nav from "./components/Nav.vue";

export default {
  name: "App",
  components: { Nav },
};
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  width: 100%;
}

body {
  margin: 0;
  max-height: 100vh;
  width: 100%;
  display: flex;
}

html {
  min-height: 100vh;
  display: flex;
}
</style>
