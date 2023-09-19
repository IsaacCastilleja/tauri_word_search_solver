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

let viewBoxScaleX = 0;
let viewBoxScaleY = 0;
let letterScaleX = 0;
let letterScaleY = 0;
let viewBoxSpacingX = 0;
let viewBoxSpacingY = 0;
let viewBoxWidth = ref(0);
let viewBoxHeight = ref(0);
async function getGridArea() {
  setTimeout(() => {
    const let1 = document.getElementById(`letter-0-0`); // Top left corner
   
    const grid_area = document.querySelector(".lettergrid-area");

    const borderSpacingX = let1.offsetWidth/2;   // Space between outer letters and lettergrid-area border
    const borderSpacingY = let1.offsetHeight/2 - 3; // Should be equal to borderSpacingX since 1/1 aspect ratio
    
  
    viewBoxWidth.value = 100;
    viewBoxHeight.value = (grid_size.value[0] - 1) / (grid_size.value[1] - 1) * 100;

    viewBoxScaleX = 100 / grid_area.offsetWidth; // Scaling factor in X direction
    viewBoxScaleY = viewBoxHeight.value / grid_area.offsetHeight; // Scaling factor in Y direction
    viewBoxSpacingX = borderSpacingX * viewBoxScaleX;
    viewBoxSpacingY = borderSpacingY * viewBoxScaleY;

    letterScaleX = (100 - viewBoxSpacingX * 2.0) / (grid_size.value[1] - 1);
    letterScaleY = (viewBoxHeight.value - viewBoxSpacingY * 2.0) / (grid_size.value[0] - 1);

  }, 150);
}

function drawConnector(letter1, letter2, lineType) {
  try {
    let x1 = letter1[1] * letterScaleX + viewBoxSpacingX;
    let y1 = letter1[0] * letterScaleY + viewBoxSpacingY;

    let x2 = letter2[1] * letterScaleX + viewBoxSpacingX;
    let y2 = letter2[0] * letterScaleY + viewBoxSpacingY;

    // if(lineType == 'r'){
    //   y1 -= 4 * viewBoxScaleY;
    //   y2 -= 4 * viewBoxScaleY; 
    // }

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
      <svg class="connectorSVG" xmlns="http://www.w3.org/2000/svg" :viewBox="`0 0 ${viewBoxWidth} ${viewBoxHeight}`">
        <Connector v-for="solvedWord in solvedWords" :x1="solvedWord.x1" :x2="solvedWord.x2" :y1="solvedWord.y1" :y2="solvedWord.y2"/>
      </svg>
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
.connectorSVG {
  box-sizing: border-box;
  z-index: 1000;
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}
.lettergrid-row {
  box-sizing: border-box;
  display: flex;
  flex: 1 1 auto;
  /* align-items: center; */
}

.lettergrid-area {
  /* box-sizing: border-box; */
  position:relative;
  border: solid 3px white;
  border-radius: 5px;
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 70%;
  flex: 1 1 auto;
  /* align-items:stretch; */
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
  min-width: fit-content;
  min-height: fit-content;
  max-height: 100vh;
  max-width: 100vw;
}

/* Add more styles as necessary */
</style>