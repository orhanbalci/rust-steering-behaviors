extern crate tui;
extern crate termion;

use std::io;
use std::thread;
use std::time;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, border, SelectableList, List};
use tui::widgets::canvas::{Canvas, Map, MapResolution, Line};
use tui::layout::{Group, Direction, Size};
use tui::style::{Style, Color, Modifier};

struct App<'a> {
    items: Vec<&'a str>,
    selected: usize,
    events: Vec<(&'a str, &'a str)>,
    info_style: Style,
    warning_style: Style,
    error_style: Style,
    critical_style: Style,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: vec!["Seek", "Flee", "Pursue"],
            selected: 0,
            events: vec![("Event1", "INFO"),
                         ("Event2", "INFO"),
                         ("Event3", "CRITICAL"),
                         ("Event4", "ERROR"),
                         ("Event5", "INFO"),
                         ("Event6", "INFO"),
                         ("Event7", "WARNING"),
                         ("Event8", "INFO"),
                         ("Event9", "INFO"),
                         ("Event10", "INFO"),
                         ("Event11", "CRITICAL"),
                         ("Event12", "INFO"),
                         ("Event13", "INFO"),
                         ("Event14", "INFO"),
                         ("Event15", "INFO"),
                         ("Event16", "INFO"),
                         ("Event17", "ERROR"),
                         ("Event18", "ERROR"),
                         ("Event19", "INFO"),
                         ("Event20", "INFO"),
                         ("Event21", "WARNING"),
                         ("Event22", "INFO"),
                         ("Event23", "INFO"),
                         ("Event24", "WARNING"),
                         ("Event25", "INFO"),
                         ("Event26", "INFO")],
            info_style: Style::default().fg(Color::White),
            warning_style: Style::default().fg(Color::Yellow),
            error_style: Style::default().fg(Color::Magenta),
            critical_style: Style::default().fg(Color::Red),
        }
    }

    fn advance(&mut self) {
        let event = self.events.pop().unwrap();
        self.events.insert(0, event);
    }
}

enum Event {
    Input(event::Key),
    Tick,
}

fn main() {
    // Terminal initialization
    let backend = TermionBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();

    // Channels
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();
    let clock_tx = tx.clone();

    // Input
    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    // Tick
    thread::spawn(move || {
        loop {
            clock_tx.send(Event::Tick).unwrap();
            thread::sleep(time::Duration::from_millis(500));
        }
    });

    // App
    let mut app = App::new();

    // First draw call
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    draw(&mut terminal, &app);

    loop {
        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => {
                match input {
                    event::Key::Char('q') => {
                        break;
                    }
                    event::Key::Down => {
                        app.selected += 1;
                        if app.selected > app.items.len() - 1 {
                            app.selected = 0;
                        }
                    }
                    event::Key::Up => {
                        if app.selected > 0 {
                            app.selected -= 1;
                        } else {
                            app.selected = app.items.len() - 1;
                        }
                    }
                    _ => {}
                }
            }
            Event::Tick => {
                app.advance();
            }
        }
        draw(&mut terminal, &app);
    }

    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<TermionBackend>, app: &App) {

    let size = t.size().unwrap();

    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(90), Size::Percent(10)])
        .render(t, &size, |t, chunks| {
            Group::default()
                .direction(Direction::Horizontal)
                .sizes(&[Size::Percent(15), Size::Percent(85)])
                .render(t, &chunks[0], |t, chunks|{
                    SelectableList::default()
                        .block(Block::default()
                               .borders(border::ALL)
                               .title("Steering Behaviors")
                              )
                        .items(&app.items)
                        .select(app.selected)
                        .highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold))
                        .highlight_symbol(">")
                        .render(t, &chunks[0]);
                    
                    Canvas::default()
                        .block(Block::default()
                               .borders(border::ALL)
                               .title("Steering Actors")
                              )
                        .paint(|ctx|{
                            ctx.draw(&Map{
                                color : Color::Red,
                                resolution : MapResolution::High,
                            });
                        })
                        .x_bounds([-180.0, 180.0])
                        .y_bounds([-90.0,  90.0])
                        .render(t,&chunks[1]);
                });
            
            Block::default()
                .borders(border::ALL)
                .title("Instructions")
                .title_style(
                    Style::default()
                    .fg(Color::White)
                    .bg(Color::Red)
                    .modifier(Modifier::Bold)
                            )
                .render(t, &chunks[1]);
        });

    t.draw().unwrap();
}