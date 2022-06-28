use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::*,
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();

    let tick_rate = Duration::from_millis(200);

    // "Move" moves the ownership to the thread.
    // This is listening for inputs in  a separate thread, not blocking the main rendering thread.
    thread::spawn(move || {
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();

    let crossterm_backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(crossterm_backend)?;

    terminal.clear()?;

    loop {
        terminal.draw(|rect| {
            let header = Block::default().title("Header").borders(Borders::ALL);
            let main = Block::default().title("Main").borders(Borders::ALL);

            let footer = Paragraph::new("Footer message")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Copyright")
                        .border_type(BorderType::Plain),
                );

            let main_left = Block::default().title("MainLeft").borders(Borders::ALL);
            let main_right = Block::default().title("MainRight").borders(Borders::ALL);

            let main_layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(rect.size());

            let pains_inside_main = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(main_layout[1]);

            rect.render_widget(header, main_layout[0]);
            rect.render_widget(main, main_layout[1]);
            rect.render_widget(footer, main_layout[2]);

            rect.render_widget(main_left, pains_inside_main[0]);
            rect.render_widget(main_right, pains_inside_main[1]);
        })?;

        match rx.recv()? {
            Event::Input(event) => {
                println!("Key pressed...");
                match event.code {
                    KeyCode::Char('q') => {
                        println!("Exiting program...");
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        break;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(())
}
