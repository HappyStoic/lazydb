mod util {
    pub mod event;
}

mod app;
mod ui;
mod client;

use app::App;

use termion::event::Key;
use util::event::{Event, Events};
use std::error::Error;
use clap::{Clap, AppSettings};
use crate::ui::View;


const APP_TITLE: &str = "Lazydb";

#[derive(Clap)]
#[clap(version = "0.0.1", author = "Martin Řepa. <repa.martin@protonmail.ch>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct CliArgs {
    /// Remote host to connect to
    #[clap(short, long)]
    host: String,
    /// Port on which database is listening on
    #[clap(short, long)]
    port: u16,
    /// Optional username
    #[clap(long)]
    username: Option<String>,
    /// Optional password
    #[clap(long)]
    password: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: CliArgs = CliArgs::parse();

    let events = Events::new();
    let view = View::new();
    let mut app = App::new(APP_TITLE, view, args);
    app.init();

    loop {
        app.update_view();
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
        }
        if app.should_quit {
            break;
        }
        //TODO add a bit of sleep here?
    }
    Ok(())
}
