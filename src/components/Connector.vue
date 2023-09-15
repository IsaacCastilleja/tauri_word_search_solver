<script setup>

const props = defineProps(['x1', 'y1', 'x2', 'y2']);

</script>

<template>
    <svg class="connectorSVG" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 250 250">
        <line :x1="x1" :y1="y1" :x2="x2" :y2="y2" stroke="red" stroke-width="1px"/>
    </svg>
</template>
  
  
<style scoped>
.connectorSVG {
    z-index: 1000;
    position: absolute;
    width: 100%;
    height: 100%;
}
</style>
