<script setup>

const props = defineProps(['x1', 'y1', 'x2', 'y2']);

</script>

<template>
    <svg class="connectorSVG" xmlns="http://www.w3.org/2000/svg">
        <line class="path" :x1="x1" :y1="y1" :x2="x2" :y2="y2" stroke="white" stroke-width="1.5px"/>
    </svg>
</template>
  
  
<style scoped>
.connectorSVG {
    z-index: 1000;
    position: absolute;
    width: 100%;
    height: 100%;
}

.path {
  stroke-dasharray: 1000;
  stroke-dashoffset: 1000;
  animation: dash 3s linear forwards;
}

@keyframes dash {
  to {
    stroke-dashoffset: 0;
  }
}
</style>