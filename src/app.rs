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