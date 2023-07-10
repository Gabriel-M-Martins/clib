#![allow(unused)]

use std::{io, thread, time::{Duration, Instant}, sync::mpsc};
use crossterm::{terminal::{disable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen, enable_raw_mode}, execute, event::{DisableMouseCapture, EnableMouseCapture, self, Event, KeyCode}};
use tui::{backend::{CrosstermBackend, Backend}, Terminal, widgets::{Borders, Block, ListItem, List}, Frame, layout::{Layout, Constraint, Direction}, text::Text, style::{Style, Modifier, Color}};

enum CEvent<I> {
    Input(I),
    Tick
}

fn main() -> Result<(), io::Error>{
    // setup terminal ------------------------------------------------------------------------------------
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    // ---------------------------------------------------------------------------------------------------  

    let mut app = App {
        state: AppState::List,
        snippets: vec![
            Snippet { command: "teste".to_string(), description: "desc".to_string(), category: None },
            Snippet { command: "teste2".to_string(), description: "descricao maior e tal e mais cheio de coisa e info e pah ne".to_string(), category: Some("dotnet".to_string())},
            Snippet { command: "teste3".to_string(), description: "descricao maior e tal e mais cheio de coisa e info e pah ne, agora com ainda mais texto e coisarada ne time vamo quebra a linha porra".to_string(), category: None },
        ],
        selected: None,
    };

    // input thread --------------------------------------------------------------------------------------
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll worls") {
                if let Event::Key(key) = event::read().expect("can read events") {
                    tx.send(CEvent::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(CEvent::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    // ---------------------------------------------------------------------------------------------------

    // main thread | draw loop ----------------------------------------------------------------------------
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        // handle input
        if let Ok(rx) = rx.recv() {
            match rx {
                CEvent::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                },
                CEvent::Tick => {}
            }
        }
    }
    // ---------------------------------------------------------------------------------------------------

    // restore terminal ----------------------------------------------------------------------------------
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;
    // ---------------------------------------------------------------------------------------------------

    Ok(())
}


fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());

    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(75),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let tabs_block = Block::default().title("Commands").borders(Borders::ALL);
    f.render_widget(tabs_block, chunks[0]);

    //  snippets list -------------------------------------------------------------------------------------------------
    let items: Vec<ListItem> = app.snippets
        .iter()
        .map(|s| { 
            let mut title = Text::styled(s.command.clone(), Style::default().add_modifier(Modifier::BOLD));
            
            // let sub_text = Text::styled(s.description.clone(), Style::default().add_modifier(Modifier::DIM));
            // title.extend(sub_text);

            let width: usize = (middle_chunks[0].width - 2).into();
            let wrapped_text = textwrap::wrap(s.description.as_str(), width);
            
            let sub_text: Vec<Text> = wrapped_text
                .iter()
                .map(|t| {
                    Text::styled(t.to_owned(), Style::default().add_modifier(Modifier::DIM)) 
                })
                .collect();
            
            sub_text.iter()
                .for_each(|t| {
                    title.extend(t.clone());
                });

            return ListItem::new(title);
        }).collect();
    
    let snippets_list = List::new(items)
        .block(Block::default().title("Snippets").borders(Borders::ALL))
        .highlight_style(tui::style::Style::default().fg(tui::style::Color::Yellow))
        .highlight_symbol(">> ");

    f.render_widget(snippets_list, middle_chunks[0]);
    // ---------------------------------------------------------------------------------------------------------------
    

    // category list -------------------------------------------------------------------------------------------------
    let items: Vec<ListItem> = app.snippets
        .iter()
        .fold(vec![], |mut acc, s| {
            match s.category {
                Some(ref c) => {
                    if acc.contains(c) {
                        return acc;
                    }
                    acc.push(c.clone());
                },
                None => {},
            }

            return acc
        })
        .iter()
        .map(|s| {
            ListItem::new(s.clone())
        })
        .collect();

    let category_list = List::new(items)
        .block(Block::default().title("Categories").borders(Borders::ALL))
        .highlight_style(tui::style::Style::default().fg(tui::style::Color::Yellow))
        .highlight_symbol(">> ");
    
    f.render_widget(category_list, middle_chunks[1]);
    // ---------------------------------------------------------------------------------------------------------------


    let search_block = Block::default().title("Search").borders(Borders::ALL);
    f.render_widget(search_block, chunks[2]);
}


struct App<'a> {
    state: AppState,
    snippets: Vec<Snippet>,
    selected: Option<&'a Snippet>,
}

enum AppState {
    List,
    Search,
}

struct Snippet {
    command: String,
    description: String,
    category: Option<String>,
}