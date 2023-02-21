<template>
  <Nav />
  <router-view></router-view>
</template>

<script>
import Nav from "@/components/Nav.vue";

export default {
  name: "App",
  components: { Nav },
};
</script>

<style>
@import "./assets/styles/theme.css";

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: var(--font-color);
  width: 100%;
}

body {
  margin: 0;
  width: 100%;
  display: flex;
}

html {
  min-height: 100vh;
  display: flex;
}
</style>
