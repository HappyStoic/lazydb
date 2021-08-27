use tui::widgets::TableState;
use crate::client::{Client};
use crate::ui::View;

pub struct StatefulTable {
    pub state: TableState,
    pub items: Vec<(String, String)>,
}

impl StatefulTable {
    fn new() -> StatefulTable {
        StatefulTable {
            state: TableState::default(),
            items: Vec::new(),
        }
    }
    fn data(mut self, items: Vec<(String, String)>) -> StatefulTable{
        self.items = items;
        self
    }

    pub fn next(&mut self) {
        if self.items.len() == 0 { return }
        let i = (self.state.selected().unwrap_or(0) + 1) % self.items.len();
        self.state.select(Some(i))
    }

    pub fn previous(&mut self) {
        if self.items.len() == 0 { return }
        // It's this stupid cuz of unsigned type
        let cur = self.state.selected().unwrap_or(0);
        let i;
        if cur == 0 {
            i = self.items.len() - 1;
        } else {
            i = cur - 1;
        }
        self.state.select(Some(i as usize))
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub table: StatefulTable,
    pub client: Client,
    pub view: View
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, view: View) -> App<'a> {
        App {
            title,
            should_quit: false,
            table: StatefulTable::new(),
            client: Client::new(),
            view,
        }
    }

    pub fn init(&mut self){
        let items = self.client.get_all();
        self.table.items = items;
        self.update_view();
    }

    pub fn update_view(&mut self){
        self.view.draw(&mut self.table);
    }

    pub fn on_up(&mut self) {
        self.table.previous();
    }

    pub fn on_down(&mut self) {
        self.table.next();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}
