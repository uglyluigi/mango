<template>
  <Nav></Nav>
  <LibraryView></LibraryView>
</template>

<script>
import LibraryView from "./components/library/LibraryView.vue";
import Nav from "./Nav.vue";

export default {
  name: "App",
  components: { LibraryView, Nav },
};
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

body {
  margin: 0;
}
</style>
