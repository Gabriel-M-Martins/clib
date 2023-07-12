use super::Snippet;
use tui_input::Input;

pub struct App<'a> {
    pub state: AppState,
    
    pub snippets: Vec<Snippet>,
    pub selected: Option<&'a Snippet>,
    
    pub input: Input,
    pub input_mode: AppInputMode,
}

impl Default for App<'_> {
    fn default() -> Self {
        App {
            state: AppState::Snippets,
            snippets: Vec::new(),
            selected: None,
            input: Input::default(),
            input_mode: AppInputMode::Normal,
        }
    }
}

pub enum AppInputMode {
    Normal,
    Searching,
}

pub enum AppState {
    Snippets,
    Categories,
    Search,
}

pub enum AppEvent<I> {
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
            Commands::Quit,
            Commands::Search,
            Commands::Select,
            Commands::Delete,
            Commands::Edit,
            Commands::New,
            Commands::Help,
            Commands::None,
        ]
    }
}