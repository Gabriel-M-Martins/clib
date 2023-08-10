extern crate serde;

use super::Snippet;
use serde::{Deserialize, Serialize};
use tui_input::Input;

#[derive(Deserialize, Serialize)]
pub struct App {
    pub state: State,
    
    pub snippets: Vec<Snippet>,
    pub categories: Vec<Category>,
    
    pub input: Input,
    pub input_mode: AppInputMode,
}

impl Default for App {
    fn default() -> Self {
        App {
            state: State::Main,
            snippets: Vec::new(),
            categories: vec![ Category {name: "No category".to_string(), indices: Vec::new()} ],
            input: Input::default(),
            input_mode: AppInputMode::Normal,
        }
    }
}

impl App {
    pub fn add_snippet(&mut self, snippet: Snippet) {
        match &snippet.category  {
            Some(category) => match self.categories.iter().position(|c| c.name == *category) {
                Some(idx) => self.categories[idx].indices.push(self.snippets.len()),
                
                None => self.categories.push(Category {
                    name: category.clone(),
                    indices: vec![self.snippets.len()],
                }),
            }

            None => {
                self.categories[0].indices.push(self.snippets.len());
            }
        }

        self.snippets.push(snippet);
    }

    pub fn remove_snippet(&mut self, idx: usize) {
        let snippet = self.snippets.remove(idx);

        if let Some(category) = snippet.category {
            if let Some(idx) = self.categories.iter().position(|c| c.name == *category) {
                self.categories.remove(idx);
            }
        }
    }

    pub fn save(self) {

    }
}

#[derive(Deserialize, Serialize)]
pub enum AppInputMode {
    Normal,
    Searching,
}

#[derive(Deserialize, Serialize)]
pub enum State {
    Main,
    NewSnippet,
    Settings
}

pub enum Event<I> {
    Input(I),
    Tick
}

pub enum Commands {
    Quit,
    Search,
    Select,
    Delete,
    Edit,
    New,
    Help,
    None,
}

impl Commands {
    pub fn stringfy(&self) -> &str {
        match self {
            Commands::Quit => "Quit",
            Commands::Search => "Search",
            Commands::Select => "Select",
            Commands::Delete => "Delete",
            Commands::Edit => "Edit",
            Commands::New => "New",
            Commands::Help => "Help",
            Commands::None => "None",
        }
    }

    pub fn all_cases() -> Vec<Commands> {
        vec![
            Commands::Search,
            Commands::New,
            Commands::Edit,
            Commands::Delete,
            Commands::Quit,
        ]
    }
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct Category {
    pub name: String,
    pub indices: Vec<usize>,
}