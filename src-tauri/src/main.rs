// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::Mutex;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_wordsearch(path: &str) -> Vec<Vec<char>> {
    let mut word_search: Vec<Vec<char>> = Vec::new();
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
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

fn parse_wordbank(path: &str) -> Vec<String> {
    let mut word_bank: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(path) {
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

#[derive(Clone, Hash, PartialEq, Eq)]
struct SolvedWord {
    start_letter: [usize; 2],
    end_letter: [usize; 2],
    word: String,
    line_type: char,
}

impl Serialize for SolvedWord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        let mut state = serializer.serialize_struct("SolvedWord", 4)?;
        state.serialize_field("start_letter", &self.start_letter)?;
        state.serialize_field("end_letter", &self.end_letter)?;
        state.serialize_field("word", &self.word)?;
        state.serialize_field("line_type", &self.line_type)?;
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


fn solve_wordsearch(wordsearch: WordSearch) -> Vec<SolvedWord> {
    let letter_grid = wordsearch.letter_grid;
    let word_bank = wordsearch.word_bank;
    let mut final_solved: Vec<SolvedWord> = Vec::new();

    fn parse_word_from_string(ref_string: &str, word_match: &str, is_reverse: bool) -> Option<[usize; 2]> {
        let mut ref_idx = 0;
        let word_chars: Vec<char> = word_match.chars().collect();
        
        // Brute force search to fix words not being found if the first letter of ref_string is duplicated in the word search e.g. "apple" wouldn't be found in "A A P P L E" the old way
        while ref_idx < ref_string.len() {
            let mut count = 0;
            let mut tmp_idx = ref_idx;
            
            while tmp_idx < ref_string.len() && count < word_match.len() {
                if ref_string.chars().nth(tmp_idx).unwrap() == word_chars[count] {
                    count += 1;
                    if count == word_match.len() {
                        let start = if is_reverse { ref_string.len() - tmp_idx - 1} else { tmp_idx + 1 - word_match.len() };
                        let end = if is_reverse {ref_string.len() - tmp_idx - 1 + word_match.len() - 1} else {tmp_idx};
                        return Some([start, end]);
                    }
                } else {
                    break;
                }
                tmp_idx += 1;
            }
            ref_idx += 1;
        }
    
        None
    }

    fn solve_rows(grid: &[Vec<char>], bank: &[String], solved_words: &mut Vec<SolvedWord>) {
        for (row_index, row) in grid.iter().enumerate() {
            let row_string: String = row.iter().collect();
            let reversed_row_string: String = row_string.chars().rev().collect();

            for word in bank {
                if let Some(col_indices) = parse_word_from_string(&row_string, word, false) {
                    let grid_indices = [
                        [row_index, col_indices[0]], 
                        [row_index, col_indices[1]]
                    ];
                    solved_words.push(SolvedWord { 
                        start_letter: grid_indices[0], 
                        end_letter: grid_indices[1], 
                        word: word.clone(), line_type: 'r'
                    });
                    
                }

                if let Some(col_indices) = parse_word_from_string(&reversed_row_string, word, true) {
                    let grid_indices = [
                        [row_index, col_indices[0]], 
                        [row_index, col_indices[1]]
                    ];
                    solved_words.push(SolvedWord { 
                        start_letter: grid_indices[0], 
                        end_letter: grid_indices[1], 
                        word: word.clone(), line_type: 'r'
                    });
                }
            }
        }
    }

    fn solve_columns(grid: &[Vec<char>], bank: &[String], solved_words: &mut Vec<SolvedWord>) {
        for column_index in 0..grid[0].len() {
            let mut col_string = String::new();
            for row in grid.iter() {
                col_string.push(row[column_index]);
            }
            let reversed_col_string : String = col_string.chars().rev().collect();
            for word in bank{
                if let Some(row_indices) = parse_word_from_string(&col_string, word, false){
                    let grid_indices = [
                        [row_indices[0], column_index],
                        [row_indices[1], column_index]
                    ];
                    solved_words.push(SolvedWord { 
                        start_letter: grid_indices[0], 
                        end_letter: grid_indices[1], 
                        word: word.clone(), 
                        line_type: 'c'
                    });
                }
                if let Some(row_indices) = parse_word_from_string(&reversed_col_string, word, true){
                    let grid_indices = [
                        [row_indices[0], column_index],
                        [row_indices[1], column_index]
                    ];
                    solved_words.push(SolvedWord { 
                        start_letter: grid_indices[0], 
                        end_letter: grid_indices[1], 
                        word: word.clone(), 
                        line_type: 'c'
                    });
                }
            }
            
        }
        
    }

    fn solve_positive_slope_diagonals(grid: &[Vec<char>], bank: &[String], solved_words: &mut Vec<SolvedWord>) {
        let rows = grid.len();
        let cols = grid[0].len();
        let diags = rows + cols - 1;
    
        for sum in 0..diags {
            let mut diagonal_string = String::new();
            
            let mut diagonal_coords = Vec::new();  // Store corresponding grid components for letters
            
            for row in (0..=std::cmp::min(sum, rows-1)).rev() {
                let col = sum - row;
                if col < cols {
                    diagonal_string.push(grid[row][col]);
                    diagonal_coords.push([row, col]);
                }
            }
            let reversed_diagonal_string: String = diagonal_string.chars().rev().collect();
            for word in bank {
                if let Some(diagonal_indices) = parse_word_from_string(&diagonal_string, word, false) {
                    let start_grid_coord = diagonal_coords[diagonal_indices[0]];
                    let end_grid_coord = diagonal_coords[diagonal_indices[1]];
                    solved_words.push(SolvedWord {
                        start_letter: start_grid_coord, 
                        end_letter: end_grid_coord, 
                        word: word.clone(), 
                        line_type: '+'
                    });
                }
                
                if let Some(diagonal_indices) = parse_word_from_string(&reversed_diagonal_string, word, true) {
                    let start_grid_coord = diagonal_coords[diagonal_indices[0]];
                    let end_grid_coord = diagonal_coords[diagonal_indices[1]];
                    solved_words.push(SolvedWord { 
                        start_letter: start_grid_coord, 
                        end_letter: end_grid_coord, 
                        word: word.clone(), 
                        line_type: '+'
                    });
                }
            }
        }
    }
    

    fn solve_negative_slope_diagonals(grid: &[Vec<char>], bank: &[String], solved_words: &mut Vec<SolvedWord>) {
        let rows = grid.len();
        let cols = grid[0].len();
        let diags = rows + cols - 1;
    
        for sum in 0..diags {
            let mut diagonal_string = String::new();
            let mut diagonal_coords = Vec::new();  // Store corresponding grid components for letters
            
            // Start from the last column and move towards the first one
            for col in 0..=(std::cmp::min(sum, cols-1)) {
                
                let row = sum - col;
                if row < rows {
                    // print!("[{}, {}]", row, cols - 1 - col);
                    diagonal_string.push(grid[row][cols - 1 - col]);
                    diagonal_coords.push([row, cols - 1 - col]);
                }
            }
            // println!("{}");
            
            let reverse_diagonal_string: String = diagonal_string.chars().rev().collect();
            for word in bank {
                if let Some(diagonal_indices) = parse_word_from_string(&diagonal_string, word, false) {
                    let start_grid_coord = diagonal_coords[diagonal_indices[0]];
                    let end_grid_coord = diagonal_coords[diagonal_indices[1]];
                    solved_words.push(SolvedWord {
                        start_letter: start_grid_coord, 
                        end_letter: end_grid_coord, 
                        word: word.clone(), 
                        line_type: '-'
                    });
                }
                
                // For reversed
                if let Some(diagonal_indices) = parse_word_from_string(&reverse_diagonal_string, word, true) {
                    let start_grid_coord = diagonal_coords[diagonal_indices[0]];
                    let end_grid_coord = diagonal_coords[diagonal_indices[1]];
                    solved_words.push(SolvedWord {
                        start_letter: start_grid_coord, 
                        end_letter: end_grid_coord, 
                        word: word.clone(), 
                        line_type: '-'
                    });
                }
            }
        }
    }
    

    solve_rows(&letter_grid, &word_bank, &mut final_solved);
    solve_columns(&letter_grid, &word_bank, &mut final_solved);
    solve_positive_slope_diagonals(&letter_grid, &word_bank, &mut final_solved);
    solve_negative_slope_diagonals(&letter_grid, &word_bank, &mut final_solved);
    final_solved
}


fn main() {
    let parsed_lettergrid = parse_wordsearch("./src/word_search3.txt");
    let parsed_wordbank = parse_wordbank("./src/word_bank3.txt");
    let parsed_wordsearch = WordSearch {
        letter_grid: parsed_lettergrid,
        word_bank: parsed_wordbank,
    };
    
    // Hash set to get rid of duplicate solutions (like if there is a palindrome)
    let no_duplicates: HashSet<SolvedWord> = solve_wordsearch(parsed_wordsearch.clone()).into_iter().collect();
    // for item in &no_duplicates{
    //     println!("{}", item.word);
    // }
    // Shuffle the solved words to make for a more visually interesting solve
    let mut solved_words: Vec<SolvedWord> = no_duplicates.into_iter().collect();
    solved_words.shuffle(&mut thread_rng());
    
    tauri::Builder::default()
        .manage(Mutex::new(parsed_wordsearch))
        .manage(Mutex::new(solved_words))
        .invoke_handler(tauri::generate_handler![get_solved, get_wordsearch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
