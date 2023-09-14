<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import Letter from "./Letter.vue";
import WordBank from "./WordBank.vue";

const grid = ref([['E', 'R', 'R', 'O', 'R']]); // Initial value
const wordBank = ref([]);
async function fetchWordSearch() {
    try {
        const data = await invoke("get_wordsearch");
        grid.value = data.letter_grid;
        wordBank.value = data.word_bank;
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
  <div class=wordsearch>
    <div class="lettergrid-area">
      <div v-for="(row, rowIndex) in grid" :key="rowIndex" class="lettergrid-row">
      <Letter v-for="(letter, colIndex) in row" :key="colIndex" :value="letter" :id="`letter-${rowIndex}-${colIndex}`"/>
    </div>
    </div>
    <div class="wordbank-area">
      <WordBank v-for="word, wordIndex in wordBank" :key="wordIndex" :value="word"/>
    </div>
  </div>
  
  
</template>

<style scoped>
.lettergrid-row {
  display: flex;
}

.lettergrid-area {
  border: solid 3px white;
  border-radius: 5px;
  min-width: fit-content;
  display: flex;
  flex-direction: column;
  flex: 1;
  aspect-ratio: 1/1;
}

.wordbank-area {
  border: solid 3px white;
  border-radius: 5px;
  padding: 5%;
  min-width: fit-content;
  display: flex;
  flex-direction: column;
  flex: 1;
  gap: 5%;
}

.wordsearch {
  display: flex;
  flex-direction: row;
  max-height: 90vh;
  aspect-ratio: 1/1;
  gap: 5%;
}

/* Add more styles as necessary */
</style>
