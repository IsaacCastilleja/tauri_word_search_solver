// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::Mutex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_wordsearch() -> Vec<Vec<char>> {
    let mut word_search: Vec<Vec<char>> = Vec::new();
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./src/word_search.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut word_search_row: Vec<char> = Vec::new();
                for letter in ip.chars() {
                    word_search_row.push(letter);
                }
                word_search.push(word_search_row);
            }
        }
    }
    else{
        word_search = vec![vec!['E', 'R', 'R', 'O', 'R'], vec!['P', 'A', 'R', 'S', 'E']];
    }
    word_search
}

fn parse_wordbank() -> Vec<String> {
    let mut word_bank: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("./src/word_bank.txt") {
        for line in lines {
            if let Ok(ip) = line {
                word_bank.push(ip);
            }
        }
    }
    else {
        word_bank = vec!["Oops".to_string(), "Word".to_string(), "Bank".to_string(), "Parsed".to_string(), "Wrong".to_string()];
    }
    word_bank
}

#[derive(Clone)]
struct WordSearch{
    letter_grid: Vec<Vec<char>>,
    word_bank: Vec<String>,
}

impl Serialize for WordSearch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("WordSearch", 2)?;
        state.serialize_field("letter_grid", &self.letter_grid)?;
        state.serialize_field("word_bank", &self.word_bank)?;
        state.end()
    }
}

#[tauri::command]
fn get_wordsearch(word_search: tauri::State<Mutex<WordSearch>>) -> WordSearch {
    word_search.lock().unwrap().clone()
}

#[derive(Clone)]
struct SolvedWord {
    start_letter: [usize; 2],
    end_letter: [usize; 2],
    word: String,
}

impl Serialize for SolvedWord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        let mut state = serializer.serialize_struct("SolvedWord", 3)?;
        state.serialize_field("start_letter", &self.start_letter)?;
        state.serialize_field("end_letter", &self.end_letter)?;
        state.serialize_field("word", &self.word)?;
        state.end()
    }
}
#[tauri::command]
fn get_solved(solved_words: tauri::State<Mutex<Vec<SolvedWord>>>) -> Result<SolvedWord, String> {
    let mut locked_words = solved_words.lock().map_err(|e| format!("Failed to lock solved_words: {}", e))?;
    if let Some(word) = locked_words.pop() {
        Ok(word)
    } else {
        Err("No words available.".into())
    }
}


fn solve_wordsearch(wordsearch: WordSearch) -> Vec<SolvedWord>{
    fn parse_word_from_row(row_string: &str, word_match: &str, is_reverse: bool) -> [usize; 2] {
        let mut count = 0;
        let word_chars: Vec<char> = word_match.chars().collect();  // Convert word_match into a Vec<char>

        for (index, letter) in row_string.chars().enumerate() {
            if count == word_match.len() {
                let start = if is_reverse {row_string.len() - index - word_match.len()} else {index - word_match.len()};
                let end = if is_reverse {row_string.len() - index - 1} else {index - 1};
                return [start, end];
            }

            if letter == word_chars[count] {
                count += 1;
                if count == word_match.len() {
                    let start = if is_reverse {row_string.len() - index - 1} else {index + 1 - word_match.len()};
                    let end = if is_reverse {row_string.len() - index - 1 + word_match.len() - 1} else {index};
                    return [start, end];
                }
            } else {
                count = 0;  // Reset the count if there's a mismatch
            }
            
        }

        [0, 0]  // Return [0, 0] if the word_match is not found in row_string (Shouldn't happen)
    }
    
    let grid = wordsearch.letter_grid;
    let bank = wordsearch.word_bank;
    let mut solved_words: Vec<SolvedWord> = Vec::new();
    for (row_index, row) in grid.iter().enumerate() {
        let row_string: String = row.iter().collect();
        for word in &bank {
            // left-right case
            if row_string.contains(word){
                let col_indices = parse_word_from_row(&row_string, &word, false);
                let grid_indices = [[row_index, col_indices[0]], [row_index, col_indices[1]]];
                solved_words.push(SolvedWord { start_letter: grid_indices[0], end_letter: grid_indices[1], word: word.clone()})
            }
            // right-left case
            let reverse_row_string = row_string.chars().rev().collect::<String>();
            if reverse_row_string.contains(word) {
                let col_indices = parse_word_from_row(&reverse_row_string, &word, true);
                let grid_indices = [[row_index, col_indices[0]], [row_index, col_indices[1]]];
                solved_words.push(SolvedWord { start_letter: grid_indices[0], end_letter: grid_indices[1], word: word.clone()})
            }
        }
    }
    solved_words
}



fn main() {
    let parsed_lettergrid = parse_wordsearch();
    let parsed_wordbank = parse_wordbank();
    let parsed_wordsearch = WordSearch {
        letter_grid: parsed_lettergrid,
        word_bank: parsed_wordbank,
    };

    let solved_words: Vec<SolvedWord>;
    solved_words = solve_wordsearch(parsed_wordsearch.clone());

    tauri::Builder::default()
        .manage(Mutex::new(parsed_wordsearch))
        .manage(Mutex::new(solved_words))
        .invoke_handler(tauri::generate_handler![get_solved, get_wordsearch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
