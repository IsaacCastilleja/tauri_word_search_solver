<script setup>
// In Letter.vue

const props = defineProps(['value']);
</script>

<template>
  <div class="wordbank-item">{{ value }}</div>
</template>

<style scoped>
    .wordbank-item {
        justify-content: top;
        align-items: top;
    }
</style>