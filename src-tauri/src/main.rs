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


fn main() {
    let parsed_lettergrid = parse_wordsearch();
    let parsed_wordbank = parse_wordbank();
    let parsed_wordsearch = WordSearch {
        letter_grid: parsed_lettergrid,
        word_bank: parsed_wordbank,
    };
    tauri::Builder::default()
        .manage(Mutex::new(parsed_wordsearch))
        .invoke_handler(tauri::generate_handler![get_wordsearch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
