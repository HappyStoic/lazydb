use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Terminal, Frame,
    text::{Span, Spans},
};
use crate::app::{StatefulTable, App};
use tui::backend::{TermionBackend, Backend};
use std::io;
use termion::{input::MouseTerminal, raw::RawTerminal, raw::IntoRawMode, screen::AlternateScreen};

use std::io::Stdout;
use tui::layout::Rect;
use crate::CliArgs;
use tui::widgets::{Paragraph, Wrap, BorderType};

// TODO add cache for generated widgets so they are not recomputed every frame?
// TODO Does it TUI library itself or not?
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

    pub fn draw(&mut self, table: &mut StatefulTable, args: &CliArgs) {
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

            draw_left_tab(f, args, chunks[0]);
            draw_table(f, table, chunks[1]);
        });
    }
}

fn draw_left_tab<B>(f: &mut Frame<B>, args: &CliArgs, area: Rect)
    where
        B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ]
                .as_ref(),
        )
        .split(area);

    draw_information(f, args, chunks[0]);
    draw_controls(f, chunks[1]);
}

fn draw_controls<B>(f: &mut Frame<B>, area: Rect)
    where
        B: Backend,
{
    let text = vec![
        Spans::from(vec![
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" -> "),
            Span::styled("quit", Style::default().add_modifier(Modifier::ITALIC)),
        ]),
    ];
    let block = new_block("Controls");
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}


fn draw_information<B>(f: &mut Frame<B>, args: &CliArgs, area: Rect)
    where
        B: Backend,
{
    let default = String::from("not-provided"); //TODO make default view with distinguishable style
    let text = vec![
        Spans::from(vec![
            Span::styled("Host", Style::default().fg(Color::Blue)),
            Span::raw(": "),
            Span::raw(&args.host),
        ]),
        Spans::from(vec![
            Span::styled("Port", Style::default().fg(Color::Blue)),
            Span::raw(": "),
            Span::raw((&args.port).to_string()),
        ]),
        Spans::from(vec![
            Span::styled("Username", Style::default().fg(Color::Blue)),
            Span::raw(": "),
            Span::raw((args.username.as_ref()).unwrap_or(&default)),
        ]),
        Spans::from(vec![
            Span::styled("Password", Style::default().fg(Color::Blue)),
            Span::raw(": "),
            Span::raw((args.password.as_ref()).unwrap_or(&default)),
        ]),
    ];
    let block = new_block("Information");
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_table<B>(f: &mut Frame<B>, table: &mut StatefulTable, area: Rect)
    where
        B: Backend,
{
    let mut odd = false;
    let items: Vec<Row> = table
        .items
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

    let table_block = Table::new(items)
        .header(header)
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .block(new_block("Data"))
        .widths(&[
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ]);
    f.render_stateful_widget(table_block, area, &mut table.state);
}

fn new_block(title: &str) -> Block {
    Block::default().borders(Borders::ALL).title(Span::styled(
        title,
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))
}
