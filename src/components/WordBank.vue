<script setup>
// In Letter.vue

const props = defineProps(['value']);
</script>

<template>
  <div class="wordbank-item">{{ value }}</div>
</template>

<style scoped>
.wordbank-item {
  break-inside: avoid;  /* Prevent items from breaking across columns */
  align-self: flex-start;
  width: 100%;  /* Ensure the item takes up the full column width */
}
</style>