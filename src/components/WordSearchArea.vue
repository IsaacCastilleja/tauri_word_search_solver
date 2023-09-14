<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import Letter from "./Letter.vue";

const grid = ref([['E', 'R', 'R', 'O', 'R']]); // Initial value
const wordBank = ref([]);
async function fetchWordSearch() {
    try {
        const data = await invoke("get_wordsearch");
        grid.value = data.letter_grid;
        console.log(data);
    } catch (err) {
        console.error("Error fetching word search:", err);
        grid.value = [['E', 'R', 'R', 'O', 'R']];
    }
}

onMounted(fetchWordSearch);

const solvedWords = ref([]);

async function checkForSolved() {

}

</script>


<template>
  <div class="wordsearch-area">
    <div v-for="(row, rowIndex) in grid" :key="rowIndex" class="wordsearch-row">
      <Letter v-for="(letter, colIndex) in row" :key="colIndex" :value="letter" :id="`letter-${rowIndex}-${colIndex}`"/>
    </div>
  </div>
</template>

<style scoped>
.wordsearch-row {
  display: flex;
}

.wordsearch-area {
  border: solid 3px white;
  border-radius: 5px;
  min-width: fit-content;
  display: flex;
  flex-direction: column;
  aspect-ratio: 1/1;
}

/* Add more styles as necessary */
</style>
