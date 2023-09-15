<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import Letter from "./Letter.vue";
import WordBank from "./WordBank.vue";
import Connector from "./Connector.vue";

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
  const data = await invoke("get_solved").catch((e) => {
    console.log(e);
    return;
  });
  if(data){
    return data;
  }
}

let solvedInterval = setInterval(async () => {
  const solved = await checkForSolved();
  if(solved){
    drawConnector(solved.start_letter, solved.end_letter, solved.line_type);
    document.getElementById(`wordbank-${solved.word}`).style.setProperty("text-decoration", "line-through");
  }
}, 3000);

function drawConnector(letter1, letter2, lineType) {
  const let1 = document.getElementById(`letter-${letter1[0]}-${letter1[1]}`);
  const let2 = document.getElementById(`letter-${letter2[0]}-${letter2[1]}`);
  const rect1 = let1.getBoundingClientRect();
  const rect2 = let2.getBoundingClientRect();

  const letterGridArea = document.querySelector('.lettergrid-area');
  const letterGridAreaRect = letterGridArea.getBoundingClientRect();

  let x1_offset = 0;
  let x2_offset = 0;
  let y1_offset = 0;
  let y2_offset = 0;
  if (lineType === 'r') {
    x1_offset = - 12;
    x2_offset = 5;
    y1_offset = -3;
    y2_offset = y1_offset;
  } else if (lineType === 'c') {
    x1_offset = -3;
    x2_offset = x1_offset;
    y1_offset = -8;
    y2_offset = 4;
  }
  const x1 = (rect1.left + rect1.right) / 2.0 - letterGridAreaRect.left + x1_offset;
  const x2 = (rect2.left + rect2.right) / 2.0 - letterGridAreaRect.left + x2_offset;
  const y1 = (rect1.top + rect1.bottom) / 2.0 - letterGridAreaRect.top + y1_offset;
  const y2 = (rect2.top + rect2.bottom) / 2.0 - letterGridAreaRect.top + y2_offset;

  solvedWords.value.push({ x1, x2, y1, y2 });
}



</script>


<template>
  <div class=wordsearch>
    <div class="lettergrid-area">
      <Connector v-for="solvedWord in solvedWords" :x1="solvedWord.x1" :x2="solvedWord.x2" :y1="solvedWord.y1" :y2="solvedWord.y2"/>
      <div v-for="(row, rowIndex) in grid" :key="rowIndex" class="lettergrid-row">
      <Letter v-for="(letter, colIndex) in row" :key="colIndex" :value="letter" :id="`letter-${rowIndex}-${colIndex}`"/>
    </div>
    </div>
    <div class="wordbank-area">
      <WordBank v-for="word, wordIndex in wordBank" :key="wordIndex" :value="word" :id="`wordbank-${word}`"/>
    </div>
  </div>
  
  
</template>

<style scoped>
.lettergrid-row {
  display: flex;
}

.lettergrid-area {
  position:relative;
  border: solid 3px white;
  border-radius: 5px;
  display: flex;
  flex-direction: column;
  height: min-content;
  flex: 1 1 auto;
  /* aspect-ratio: 1/1; */
}

.wordbank-area {
  border: solid 3px white;
  border-radius: 5px;
  padding: 5%;
  display: flex;
  flex-direction: column;
  flex-wrap: wrap;
  flex: none;
  height: min-content;
  width: min-content;
  gap: 5%;
}

.wordsearch {
  display: flex;
  flex-direction: row;
  /* aspect-ratio: 1/1; */
  gap: 5%;
  align-items: start;
}

/* Add more styles as necessary */
</style>
