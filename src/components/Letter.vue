<script setup>
// In Letter.vue

const props = defineProps(['value']);
</script>

<template>
  <div class="letter">{{ value }}</div>
</template>

<style scoped>
    .letter {
        display: flex;
        flex: 1;
        aspect-ratio: 1/1;
        justify-content: center;
        align-items: center;
        min-width: 25px;
    }
</style>