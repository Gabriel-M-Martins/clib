#![allow(unused)]

use std::{io, thread, time::{Duration, Instant}, sync::mpsc};
use crossterm::{terminal::{disable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen, enable_raw_mode}, execute, event::{DisableMouseCapture, EnableMouseCapture, self, Event, KeyCode}};
use ratatui::{backend::{CrosstermBackend, Backend}, Terminal, widgets::{Borders, Block, ListItem, List, Paragraph, Tabs}, Frame, layout::{Layout, Constraint, Direction}, text::{Text, Line, Span}, style::{Style, Modifier, Color}, symbols::line};

mod app;
use app::{App, AppState, AppEvent, AppInputMode, Commands};

mod data;
use data::Snippet;
use tui_input::{Input, backend::crossterm::EventHandler};

fn main() -> Result<(), io::Error>{
    // setup terminal ------------------------------------------------------------------------------------
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    // ---------------------------------------------------------------------------------------------------  

    let mut app = App::default();
    let snippets = vec![
        Snippet { command: "fooo".to_string(), description: "descricaozona po e tal".to_string(), category: None },
        Snippet { command: "baaar".to_string(), description: "outra descricaozona po e tal".to_string(), category: Some("teste".to_string()) },
    ];

    for snippet in snippets {
        app.add_snippet(snippet);
    }

    // input thread --------------------------------------------------------------------------------------
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let Event::Key(key) = event::read().expect("can read events") {
                    tx.send(AppEvent::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(AppEvent::Tick) {
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
                AppEvent::Input(event) => {
                    match app.input_mode {
                        AppInputMode::Normal => {
                            match event.code {
                                KeyCode::Char('q') | KeyCode::Char('Q') => {
                                    break;
                                }
                                
                                KeyCode::Char('s') | KeyCode::Char('S') => {
                                    app.input_mode = AppInputMode::Searching;
                                }
                                
                                _ => {}
                            }
                        },
                        AppInputMode::Searching => match event.code {
                            KeyCode::Enter => {
                                app.input_mode = AppInputMode::Normal;
                            }
                            
                            KeyCode::Esc => {
                                app.input_mode = AppInputMode::Normal;
                            }

                            _ => {
                                app.input.handle_event(&Event::Key(event));
                            }
                        }
                    }
                },
                AppEvent::Tick => {}
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
                Constraint::Min(3),
                Constraint::Percentage(100),
                Constraint::Min(3),
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


    // MARK: -  tabs ----------------------------------------------------------------------------------------------
    let commands = Commands::all_cases();
    let titles: Vec<Line> = commands
        .iter()
        .map(|command| {
            let (first_letter, rest) = command.stringfy().split_at(1);

            let first_letter = Span::styled(first_letter, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
            let rest = Span::raw(rest);

            let title = Line::from(vec![first_letter, rest]); 

            return title;
        })
        .collect();

    let tabs_block = Tabs::new(titles)
        .block(Block::default().title("Tabs").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .divider("|");

    f.render_widget(tabs_block, chunks[0]);
    // -----------------------------------------------------------------------------------------------------------

    // MARK: -  snippets list ------------------------------------------------------------------------------------
    let items: Vec<ListItem> = app.snippets
        .iter()
        .map(|s| { 
            let mut title = Text::styled(s.command.clone(), Style::default().add_modifier(Modifier::BOLD));
            
            // let sub_text = Text::styled(s.description.clone(), Style::default().add_modifier(Modifier::DIM));
            // title.extend(sub_text);

            let width = (middle_chunks[0].width - 2) as usize;
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
        .highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol(">> ");

    f.render_widget(snippets_list, middle_chunks[0]);
    // ---------------------------------------------------------------------------------------------------------------
    

    // MARK: - category list -----------------------------------------------------------------------------------------
    let items: Vec<ListItem> = app.categories
        .iter()
        .fold(vec![], |mut acc, s| {
            let item = ListItem::new(s.name.clone());
            acc.push(item);

            return acc;
        });

    let category_list = List::new(items)
        .block(Block::default().title("Categories").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol(">> ");
    
    f.render_widget(category_list, middle_chunks[1]);
    // ---------------------------------------------------------------------------------------------------------------

    // MARK: - input -------------------------------------------------------------------------------------------------
    let width = chunks[2].width.max(3) - 3;
    let scroll = app.input.visual_scroll(width as usize);

    let input = Paragraph::new(app.input.value())
        .style(match app.input_mode {
            AppInputMode::Normal => Style::default(),
            AppInputMode::Searching => Style::default().fg(Color::Yellow),
        })
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title("Search"));

    f.render_widget(input, chunks[2]);

    match app.input_mode {
        AppInputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        AppInputMode::Searching => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[2].x
                    + ((app.input.visual_cursor()).max(scroll) - scroll) as u16
                    + 1,
                // Move one line down, from the border to the input line
                chunks[2].y + 1,
            )
        }
    }
    // ---------------------------------------------------------------------------------------------------------------
}