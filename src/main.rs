mod util {
    pub mod event;
}

mod app;
mod ui;
mod client;

use app::App;

use std::io;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders}; 
use tui::Terminal;
use util::event::{Event, Events};
use std::error::Error;
use crate::ui::View;

fn main() -> Result<(), Box<dyn Error>> {

    let events = Events::new();
    let mut view = View::new();
    let mut app = App::new("Lazydb", view);
    app.init();

    loop {
        if let Event::Input(input) = events.next()? {
            match input {
                Key::Char(c) => {
                    app.on_key(c);
                }
                Key::Up => {
                    app.on_up();
                }
                Key::Down => {
                    app.on_down();
                }
                _ => {}
            }
            app.update_view();
        }
        if app.should_quit {
            break;
        }

    }
    Ok(())
}
