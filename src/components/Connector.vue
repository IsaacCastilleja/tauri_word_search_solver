<script setup>

const props = defineProps(['x1', 'y1', 'x2', 'y2']);

</script>

<template>
  <line class="path" :x1="x1" :y1="y1" :x2="x2" :y2="y2" stroke="white" stroke-width="0.15"/>
</template>
  
  
<style scoped>

.path {
  box-sizing: border-box;
  stroke-dasharray: 100;
  stroke-dashoffset: 100;
  top: 0;
  left: 0;
  animation: dash 2s linear forwards;
}

@keyframes dash {
  to {
    stroke-dashoffset: 0;
  }
}
</style>