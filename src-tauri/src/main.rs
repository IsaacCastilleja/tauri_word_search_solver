// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use serde::ser::{Serialize, Serializer, SerializeStruct};

struct WordSearch{
    letter_grid: [[char; 3]; 3],
    word_bank: [&'static str; 2],
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
fn get_wordsearch() -> WordSearch {
    let word_search = WordSearch {
       letter_grid: [
            ['A', 'B', 'C'],
            ['D', 'E', 'F'],
            ['G', 'H', 'I']
        ],
        word_bank: ["cat", "dog"], 
    };
    word_search
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_wordsearch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
