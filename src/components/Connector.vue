<script setup>

const props = defineProps(['x1', 'y1', 'x2', 'y2', 'viewBoxDimension']);

</script>

<template>
    <svg class="connectorSVG" xmlns="http://www.w3.org/2000/svg" :viewBox="viewBoxDimension">
        <line class="path" :x1="x1" :y1="y1" :x2="x2" :y2="y2" stroke="white" stroke-width="0.15"/>
    </svg>
</template>
  
  
<style scoped>
.connectorSVG {
    z-index: 1000;
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
}

.path {
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