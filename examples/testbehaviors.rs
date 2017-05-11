extern crate tui;
extern crate termion;
extern crate nalgebra;
extern crate steering;
extern crate alga;

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

use nalgebra::Vector3;
use alga::general::AbstractModule;
use steering::Steerable;
use steering::SteeringAcceleration;
use steering::Seek;
use steering::Flee;
use steering::Pursue;
use steering::Arrive;
use steering::Evade;
use steering::SteeringAccelerationCalculator;
use steering::SteeringBehavior;

struct App<'a> {
    items: Vec<&'a str>,
    selected: usize,
    events: Vec<(&'a str, &'a str)>,
    info_style: Style,
    warning_style: Style,
    error_style: Style,
    critical_style: Style,
    v: Vehicle,
    behavior: Option<&'a SteeringAccelerationCalculator<f32>>,
    positions: Vec<String>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: vec!["Seek", "Flee", "Pursue", "Arrive", "Evade"],
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
            v: Vehicle::new(),
            behavior: None,
            positions: vec![],
        }
    }

    fn advance(&mut self) {
        let event = self.events.pop().unwrap();
        self.events.insert(0, event);
        match self.behavior {
            Some(ref a) => self.v.advance(*a, 500f32),
            None => (), 
        };
        self.positions.clear();
        self.positions
            .push(format!("{}:{}:{}",
                          self.v.get_position().as_slice()[0],
                          self.v.get_position().as_slice()[1],
                          self.v.get_position().as_slice()[2]));
        // self.target.advance_by_velocity();
    }
}

enum Event {
    Input(event::Key),
    Tick,
}

struct Vehicle {
    linear_velocity: Vector3<f32>,
    position: Vector3<f32>,
    angular_velocity: f32,
    bounding_radius: f32,
}

impl Steerable<f32> for Vehicle {
    fn get_linear_velocity(&self) -> &Vector3<f32> {
        &self.linear_velocity
    }

    fn get_angular_velocity(&self) -> f32 {
        self.angular_velocity
    }

    fn get_bounding_radius(&self) -> f32 {
        self.bounding_radius
    }

    fn get_position(&self) -> &Vector3<f32> {
        &self.position
    }
}

impl Vehicle {
    fn new() -> Self {
        Vehicle {
            linear_velocity: Vector3::new(1.0, 0.0, 0.0),
            position: Vector3::new(-50.0, 50.0, 0.0),
            angular_velocity: 0.0,
            bounding_radius: 2.0,
        }
    }

    fn advance(&mut self, calc: &SteeringAccelerationCalculator<f32>, milis: f32) {
        let mut sa = SteeringAcceleration::default();
        calc.calculate_steering(&mut sa, self);
        self.linear_velocity += sa.linear;
        self.angular_velocity += sa.angular;
        self.position = self.position + self.linear_velocity.multiply_by(milis / 1000.0);
    }

    fn advance_by_velocity(&mut self, milis: f32){
        self.position += self.linear_velocity.multiply_by(milis / 1000.0);
    }
}

fn main() {
    // Terminal initialization
    let backend = TermionBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
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
    thread::spawn(move || loop {
                      clock_tx.send(Event::Tick).unwrap();
                      thread::sleep(time::Duration::from_millis(500));
                  });

    // App
    let mut target = Vehicle {
        linear_velocity: Vector3::new(1.0, 0.0, 0.0),
        position: Vector3::new(0.0, 0.0, 0.0),
        angular_velocity: 0.0,
        bounding_radius: 2.0,
    };

    let pursue = Pursue {
        behavior: SteeringBehavior {
            enabled: true,
            limiter: None,
        },
        target: &target,
        max_prediction_time: 100.0f32,
    };

    let evade = Evade {
        behavior: SteeringBehavior {
            enabled: true,
            limiter: None,
        },
        target: &target,
        max_prediction_time: 100.0f32,
    };

    let seek = Seek {
        behavior: SteeringBehavior {
            enabled: true,
            limiter: None,
        },
        target: &target,
    };

    let flee = Flee {
        behavior: SteeringBehavior {
            enabled: true,
            limiter: None,
        },
        target: &target,
    };

    let arrive = Arrive {
        behavior: SteeringBehavior {
            enabled: true,
            limiter: None,
        },
        target: &target,
        time_to_target: 10f32,
        deceleration_radius: 20f32,
        tolerance: 5f32,
    };

    let mut behaviors: Vec<&SteeringAccelerationCalculator<f32>> = vec![];
    behaviors.push(&seek);
    behaviors.push(&flee);
    behaviors.push(&pursue);
    behaviors.push(&arrive);
    behaviors.push(&evade);

    let mut app = App::new();
    app.behavior = Some(behaviors[0]);
    // First draw call
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    draw(&mut terminal, &app, &target);

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
                // target.advance_by_velocity(100.0f32);
                app.advance();
            }
        }
        app.behavior = Some(behaviors[app.selected]);
        draw(&mut terminal, &app, &target);
    }

    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<TermionBackend>, app: &App, target: &Vehicle) {

    let size = t.size().unwrap();

    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(90), Size::Percent(10)])
        .render(t, &size, |t, chunks| {
            Group::default()
                .direction(Direction::Horizontal)
                .sizes(&[Size::Percent(15), Size::Percent(85)])
                .render(t, &chunks[0], |t, chunks| {
                    SelectableList::default()
                        .block(Block::default()
                                   .borders(border::ALL)
                                   .title("Steering Behaviors"))
                        .items(&app.items)
                        .select(app.selected)
                        .highlight_style(Style::default()
                                             .fg(Color::Yellow)
                                             .modifier(Modifier::Bold))
                        .highlight_symbol(">")
                        .render(t, &chunks[0]);

                    Canvas::default()
                        .block(Block::default()
                                   .borders(border::ALL)
                                   .title("Steering Actors"))
                        .paint(|ctx| {
                            /// draw steerable vehicle
                            ctx.draw(&Line {
                                         x1: (app.v.get_position().as_slice()[0] - 10.0f32) as f64,
                                         y1: (app.v.get_position().as_slice()[1] + 10.0f32) as f64,
                                         x2: (app.v.get_position().as_slice()[0] + 10.0f32) as f64,
                                         y2: (app.v.get_position().as_slice()[1] + 10.0f32) as f64,
                                         color: Color::Red,
                                     });
                            ctx.draw(&Line {
                                         x1: (app.v.get_position().as_slice()[0] + 10.0f32) as f64,
                                         y1: (app.v.get_position().as_slice()[1] - 10.0f32) as f64,
                                         x2: (app.v.get_position().as_slice()[0] + 10.0f32) as f64,
                                         y2: (app.v.get_position().as_slice()[1] + 10.0f32) as f64,
                                         color: Color::Red,
                                     });
                            ctx.draw(&Line {
                                         x1: (app.v.get_position().as_slice()[0] - 10.0f32) as f64,
                                         y1: (app.v.get_position().as_slice()[1] - 10.0f32) as f64,
                                         x2: (app.v.get_position().as_slice()[0] + 10.0f32) as f64,
                                         y2: (app.v.get_position().as_slice()[1] - 10.0f32) as f64,
                                         color: Color::Red,
                                     });
                            ctx.draw(&Line {
                                         x1: (app.v.get_position().as_slice()[0] - 10.0f32) as f64,
                                         y1: (app.v.get_position().as_slice()[1] - 10.0f32) as f64,
                                         x2: (app.v.get_position().as_slice()[0] - 10.0f32) as f64,
                                         y2: (app.v.get_position().as_slice()[1] + 10.0f32) as f64,
                                         color: Color::Red,
                                     });
                            /// draw target
                            ctx.draw(&Line {
                                         x1: (target.get_position().as_slice()[0] - 10.0f32) as f64,
                                         y1: (target.get_position().as_slice()[1] + 10.0f32) as f64,
                                         x2: (target.get_position().as_slice()[0] + 10.0f32) as f64,
                                         y2: (target.get_position().as_slice()[1] + 10.0f32) as f64,
                                         color: Color::Green,
                                     });
                            ctx.draw(&Line {
                                         x1: (target.get_position().as_slice()[0] + 10.0f32) as f64,
                                         y1: (target.get_position().as_slice()[1] - 10.0f32) as f64,
                                         x2: (target.get_position().as_slice()[0] + 10.0f32) as f64,
                                         y2: (target.get_position().as_slice()[1] + 10.0f32) as f64,
                                         color: Color::Green,
                                     });
                            ctx.draw(&Line {
                                         x1: (target.get_position().as_slice()[0] - 10.0f32) as f64,
                                         y1: (target.get_position().as_slice()[1] - 10.0f32) as f64,
                                         x2: (target.get_position().as_slice()[0] + 10.0f32) as f64,
                                         y2: (target.get_position().as_slice()[1] - 10.0f32) as f64,
                                         color: Color::Green,
                                     });
                            ctx.draw(&Line {
                                         x1: (target.get_position().as_slice()[0] - 10.0f32) as f64,
                                         y1: (target.get_position().as_slice()[1] - 10.0f32) as f64,
                                         x2: (target.get_position().as_slice()[0] - 10.0f32) as f64,
                                         y2: (target.get_position().as_slice()[1] + 10.0f32) as f64,
                                         color: Color::Green,
                                     });

                        })
                        .x_bounds([-180.0, 180.0])
                        .y_bounds([-180.0, 180.0])
                        .render(t, &chunks[1]);
                });

            List::default()
                .block(Block::default()
                           .borders(border::ALL)
                           .title("Instructions")
                           .title_style(Style::default()
                                            .fg(Color::White)
                                            .bg(Color::Red)
                                            .modifier(Modifier::Bold)))
                .items(&app.positions
                            .iter()
                            .map(|evt| (format!("{}", evt), &app.error_style))
                            .collect::<Vec<(String, &Style)>>())
                .render(t, &chunks[1]);
        });

    t.draw().unwrap();
}
