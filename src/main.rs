use std::{io, thread, time::{Duration, Instant}, sync::mpsc};
use crossterm::{terminal::{disable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen, enable_raw_mode}, execute, event::{DisableMouseCapture, EnableMouseCapture, self, Event, KeyCode}};
use tui::{backend::{CrosstermBackend, Backend}, Terminal, widgets::{Borders, Block}, Frame, layout::{Layout, Constraint, Direction}};

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
        terminal.draw(|f| ui(f))?;

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


fn ui<B: Backend>(f: &mut Frame<B>) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(90),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .split(main_chunks[0]);

    let inspect_block = Block::default().title("Inspect").borders(Borders::ALL);
    f.render_widget(inspect_block, chunks[0]);

    let list_block = Block::default().title("List").borders(Borders::ALL);
    f.render_widget(list_block, chunks[1]);

    let search_block = Block::default().title("Search").borders(Borders::ALL);
    f.render_widget(search_block, main_chunks[1]);
}