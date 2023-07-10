use super::Snippet;

pub struct App<'a> {
    pub state: AppState,
    pub snippets: Vec<Snippet>,
    pub selected: Option<&'a Snippet>,
}

pub enum AppState {
    List,
    Search,
}

pub enum AppEvent<I> {
    Input(I),
    Tick
}