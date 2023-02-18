<template>
  <svg
    viewBox="0 0 512 512"
    xmlns="http://www.w3.org/2000/svg"
    xmlns:xlink="http://www.w3.org/1999/xlink"
    class="x-icon"
  >
    <path
      :fill="this.color"
      fill-rule="evenodd"
      stroke="none"
      d="M 399.282959 467 L 255.5 323.215271 L 111.715462 467 L 45 400.284729 L 188.784943 256.5 L 45 112.715271 L 111.715462 46 L 255.5 189.784729 L 399.282959 46 L 465.993622 112.715271 L 322.215454 256.5 L 465.993622 400.284729 Z"
    />
  </svg>
</template>

<script>
export default {
  name: "XIcon",
  props: {
    color: { type: String, default: "#FFF" },
  },
};
</script>

<style scoped></style>
