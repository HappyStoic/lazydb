use tui::{layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style}, widgets::{
    Block, Borders, Cell, Row, Table,
}, Terminal};
use crate::app::StatefulTable;
use tui::backend::TermionBackend;
use std::io;
use termion::{input::MouseTerminal, raw::RawTerminal, raw::IntoRawMode, screen::AlternateScreen};

use std::io::Stdout;

pub struct View {
    terminal: Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>, //TODO redo this somehow to generics
}

impl View {
    pub fn new() -> View {
        // Terminal initialization
        let stdout = io::stdout().into_raw_mode().unwrap();
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();
        View {
            terminal
        }
    }

    pub fn draw(&mut self, table: &mut StatefulTable) {
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(80),
                    ]
                        .as_ref(),
                )
                .split(f.size());

            // Render left block
            let block = Block::default().title("Block").borders(Borders::ALL);
            f.render_widget(block, chunks[0]);

            // Render data block
            let table_block = gen_table(&table.items);
            f.render_stateful_widget(table_block, chunks[1], &mut table.state);
        });
    }
}

pub fn gen_table(data: &Vec<(String, String)>) -> Table {
    let mut odd = false;
    let items: Vec<Row> = data
        .iter()
        .map(|(k,v)|{
            let cells = vec![
                Cell::from(k.as_str()),
                Cell::from(v.as_str()),
            ];
            let style = if odd { Style::default() } else { Style::default().fg(Color::DarkGray) };
            odd = !odd;
            Row::new(cells).style(style)
        }).collect();

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let header_cells = ["Key", "Value"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells);

    Table::new(items)
        .header(header)
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .block(Block::default().title("Data").borders(Borders::ALL))
        .widths(&[
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
}

