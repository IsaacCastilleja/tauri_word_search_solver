<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import Letter from "./Letter.vue";
import WordBank from "./WordBank.vue";
import Connector from "./Connector.vue";

const grid = ref([['E', 'R', 'R', 'O', 'R']]); // Initial value
const wordBank = ref([]);
const grid_size = ref([]);
const wordBank_size = ref();
async function fetchWordSearch() {
  try {
      const data = await invoke("get_wordsearch");
      grid.value = data.letter_grid;
      grid_size.value = data.letter_grid_size;
      wordBank.value = data.word_bank;
      wordBank_size.value = data.word_bank_size;

      getGridArea();

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
    drawConnector(solved.start_letter, solved.end_letter, solved.line_type, solved.word);
    document.getElementById(`wordbank-${solved.word}`).style.setProperty("text-decoration", "line-through");
    // console.log(solved.word);
  }
}, 250);

// function drawConnector(letter1, letter2, lineType, words) {
//   try {
//     const let1 = document.getElementById(`letter-${letter1[0]}-${letter1[1]}`);
//     const let2 = document.getElementById(`letter-${letter2[0]}-${letter2[1]}`);
//     const rect1 = let1.getBoundingClientRect();
//     const rect2 = let2.getBoundingClientRect();

//     const letterGridArea = document.querySelector('.lettergrid-area');
//     const letterGridAreaRect = letterGridArea.getBoundingClientRect();
//     let x1_offset = 0;
//     let x2_offset = 0;
//     let y1_offset = 0;
//     let y2_offset = 0;
//     if (lineType === 'r') {
//       x1_offset = - 12;
//       x2_offset = 5;
//       y1_offset = -3;
//       y2_offset = y1_offset;
//     } else if (lineType === 'c') {
//       x1_offset = -3;
//       x2_offset = x1_offset;
//       y1_offset = -8;
//       y2_offset = 4;
//     } else if (lineType === '+') {
//       x1_offset = -4;
//       x2_offset = 3;
//       y1_offset = -3;
//       y2_offset = -6;
//     } else if (lineType === '-') {
//       x1_offset = 4;
//       x2_offset = -3;
//       y1_offset = 3;
//       y2_offset = -6;
//     }
//     const x1 = (rect1.left + rect1.right) / 2.0 - letterGridAreaRect.left + x1_offset;
//     const x2 = (rect2.left + rect2.right) / 2.0 - letterGridAreaRect.left + x2_offset;
//     const y1 = (rect1.top + rect1.bottom) / 2.0 - letterGridAreaRect.top + y1_offset;
//     const y2 = (rect2.top + rect2.bottom) / 2.0 - letterGridAreaRect.top + y2_offset;

//     solvedWords.value.push({ x1, x2, y1, y2 });

//   } catch (error) {
//     console.log(error);
//     console.log(words);
//     console.log(letter1, letter2);
//   }
  

// }

// function getGridArea() {
//   const let1 = document.getElementById(`letter-0-0`); // Top left corner
//   const let2 = document.getElementById(`letter-${grid_size.value[0] - 1}-${grid_size.value[1] - 1}`); // Bottom right corner
//   const rect1 = let1.getBoundingClientRect();
//   const rect2 = let2.getBoundingClientRect();
  
//   const absoluteWidth = rect2.right - rect1.left;
//   const absoluteHeight = rect2.bottom - rect1.top;
//   const localWidth = 100 / absoluteWidth;
//   const localHeight = 100 / absoluteHeight;

//   return [localWidth, localHeight]; 
// }

// let viewBoxWidth = 0;
// let viewBoxHeight = 0;
// document.addEventListener("DOMContentLoaded", () => {
//   viewBoxWidth, viewBoxHeight = getGridArea();
// });

// function drawConnector(letter1, letter2, lineType, words) {
//   try {
    
//     // Converted to viewbox dimensions
//     console.log(grid_size.value);
//     const localX = 100/grid_size.value[1];
//     const localY = 100/grid_size.value[0];

    

//   } catch (error) {
//     console.log(error);
//     console.log(words);
//     console.log(letter1, letter2);
//   }
  

// }

let viewBoxWidth = 0;
let viewBoxHeight = 0;
async function getGridArea() {
  setTimeout(() => {
    console.log(grid_size.value[0] - 1, grid_size.value[1] - 1);
    const let1 = document.getElementById(`letter-0-0`); // Top left corner
    const let2 = document.getElementById(`letter-${grid_size.value[0] - 1}-${grid_size.value[1] - 1}`); // Bottom right corner
    const rect1 = let1.getBoundingClientRect();
    const rect2 = let2.getBoundingClientRect();

    const absoluteWidth = rect2.right - rect1.left;
    const absoluteHeight = rect2.bottom - rect1.top;
    const localWidth = 100 / absoluteWidth;
    const localHeight = 100 / absoluteHeight;

    viewBoxWidth = localWidth;
    viewBoxHeight = localHeight;
    console.log(absoluteWidth, absoluteHeight)
    console.log(viewBoxWidth, viewBoxHeight);
  }, 100);
}


// function drawConnector(letter1, letter2, lineType, words) {
//   try {
//     const localX = viewBoxWidth;
//     const localY = viewBoxHeight;

//     let x1_offset = 0;
//     let x2_offset = 0;
//     let y1_offset = 0;
//     let y2_offset = 0;
//     // if (lineType === 'r') {
//     //   x1_offset = 0;
//     //   x2_offset = 0;
//     //   y1_offset = 0;
//     //   y2_offset = y1_offset;
//     // } else if (lineType === 'c') {
//     //   x1_offset = 1;
//     //   x2_offset = x1_offset;
//     //   y1_offset = -0.1;
//     //   y2_offset = 0.1;
//     // } else if (lineType === '+') {
//     //   x1_offset = -0.4;
//     //   x2_offset = 0.3;
//     //   y1_offset = -0.3;
//     //   y2_offset = -6;
//     // } else if (lineType === '-') {
//     //   x1_offset = 0.4;
//     //   x2_offset = -0.3;
//     //   y1_offset = 0.3;
//     //   y2_offset = -6;
//     // }

//     const x1 = letter1[1] * localX + localX/2 + x1_offset;
//     const y1 = letter1[0] * localY + localY/2 + y1_offset;

//     const x2 = letter2[1] * localX + localX/2 + x2_offset;
//     const y2 = letter2[0] * localY + localY/2 + y2_offset;

//     solvedWords.value.push({ x1, x2, y1, y2 });

//   } catch (error) {
//     console.log(error);
//     console.log(words);
//     console.log(letter1, letter2);
//   }
// }

function drawConnector(letter1, letter2, lineType) {
  try {
    const localX = 100 / grid_size.value[1];
    const localY = 100 / grid_size.value[0];

    // ... (your offset logic) ...
    let x1_offset = 0;
    let x2_offset = 0;
    let y1_offset = 0;
    let y2_offset = 0;
    if (lineType === 'r') {
      x1_offset = 0;
      x2_offset = 0;
      y1_offset = 0;
      y2_offset = y1_offset;
    } else if (lineType === 'c') {
      x1_offset = 1;
      x2_offset = x1_offset;
      y1_offset = -0.1;
      y2_offset = 0.1;
    } else if (lineType === '+') {
      x1_offset = -0.4;
      x2_offset = 0.3;
      y1_offset = -0.3;
      y2_offset = -6;
    } else if (lineType === '-') {
      x1_offset = 0.4;
      x2_offset = -0.3;
      y1_offset = 0.3;
      y2_offset = -6;
    }

    const x1 = letter1[1] * localX + localX / 2 + x1_offset * viewBoxWidth; // Multiplying offset by viewBoxWidth to scale it correctly
    const y1 = letter1[0] * localY + localY / 2 + y1_offset * viewBoxHeight;

    const x2 = letter2[1] * localX + localX / 2 + x2_offset * viewBoxWidth;
    const y2 = letter2[0] * localY + localY / 2 + y2_offset * viewBoxHeight;

    solvedWords.value.push({ x1, x2, y1, y2 });

  } catch (error) {
    console.log(error);
    console.log(letter1, letter2);
  }
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
  flex: 1 1 auto;
}

.lettergrid-area {
  position:relative;
  border: solid 3px white;
  border-radius: 5px;
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 70%;
  flex: 1 1 auto;
  /* aspect-ratio: 1/1; */
}

.wordbank-area {
  border: solid 3px white;
  border-radius: 5px;
  padding: 5%;
  column-width: 20%;  /* Set the minimum column width */
  column-gap: 5%;
  column-count: 2;
  max-height: 100vh;
  width: 25%;
}

.wordsearch {
  display: flex;
  flex-direction: row;
  /* aspect-ratio: 1/1; */
  gap: 5%;
  align-items: start;
  max-height: 100vh;
  max-width: 100vw;
}

/* Add more styles as necessary */
</style>
