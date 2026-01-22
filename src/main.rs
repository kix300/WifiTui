use std::error::Error;
use ratatui::prelude::{CrosstermBackend, Terminal, Backend};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, EnableMouseCapture, DisableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use std::io;

use crate::app::CurrentScreen;

#[cfg(test)]
pub mod test;

pub mod app;

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut app::App) -> io::Result<bool>{
    loop{
        // cette fonciton sert a update lui en fonction de ce quil ce passe 
        // pas encore creeer ui 
        // terminal.draw(|f| ui(f,app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('L') => {
                        app.current_screen = CurrentScreen::Wifi;
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Info;
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                CurrentScreen::Wifi => match key.code {
                },
            }
        }
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = app::App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
