extern crate alga;
extern crate nalgebra;
extern crate steering;
extern crate termion;
extern crate tui;

use std::io;
use std::thread;
use std::time;
use std::sync::mpsc;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::RefMut;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{border, Block, List, SelectableList, Widget};
use tui::widgets::canvas::{Canvas, Line, Map, MapResolution};
use tui::layout::{Direction, Group, Size};
use tui::style::{Color, Modifier, Style};

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
    v: Rc<RefCell<Vehicle>>,
    behavior: Option<Rc<RefCell<SteeringAccelerationCalculator<f32>>>>,
    positions: Vec<String>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: vec!["Seek", "Flee", "Pursue", "Arrive", "Evade"],
            selected: 0,
            events: vec![
                ("Event1", "INFO"),
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
                ("Event26", "INFO"),
            ],
            info_style: Style::default().fg(Color::White),
            warning_style: Style::default().fg(Color::Yellow),
            error_style: Style::default().fg(Color::Magenta),
            critical_style: Style::default().fg(Color::Red),
            v: Rc::new(RefCell::new(Vehicle::new())),
            behavior: None,
            positions: vec![],
        }
    }

    fn advance(&mut self) {
        let event = self.events.pop().unwrap();
        self.events.insert(0, event);
        match self.behavior {
            Some(ref mut a) => {
                let sa = self.v.borrow().calculate_steering(a.borrow_mut());
                self.v.borrow_mut().advance(sa, 500f32)
            }
            None => (),
        };
        self.positions.clear();
        self.positions.push(format!(
            "{}:{}:{}",
            self.v.borrow().get_position().as_slice()[0],
            self.v.borrow().get_position().as_slice()[1],
            self.v.borrow().get_position().as_slice()[2]
        ));
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

    fn get_orientation(&self) -> f32 {
        0.0f32
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

    fn advance(&mut self, sa: Rc<RefCell<SteeringAcceleration<f32>>>, milis: f32) {
        self.linear_velocity += sa.borrow().linear;
        self.angular_velocity += sa.borrow().angular;
        self.position = self.position + self.linear_velocity.multiply_by(milis / 1000.0);
    }

    fn calculate_steering(
        &self,
        mut calc: RefMut<SteeringAccelerationCalculator<f32>>,
    ) -> Rc<RefCell<SteeringAcceleration<f32>>> {
        let mut sa = Rc::new(RefCell::new(SteeringAcceleration::default()));
        sa = calc.calculate_steering(sa);
        sa
    }

    fn advance_by_velocity(&mut self, milis: f32) {
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

    let mut app = App::new();
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
    let target = Rc::new(RefCell::new(Vehicle {
        linear_velocity: Vector3::new(1.0, 0.0, 0.0),
        position: Vector3::new(0.0, 0.0, 0.0),
        angular_velocity: 0.0,
        bounding_radius: 2.0,
    }));

    let pursue = Pursue {
        behavior: RefCell::new(SteeringBehavior {
            enabled: true,
            limiter: None,
            target: target.clone(),
            owner: app.v.clone(),
        }),
        max_prediction_time: 100.0f32,
    };

    let evade = Evade {
        behavior: RefCell::new(SteeringBehavior {
            enabled: true,
            limiter: None,
            target: target.clone(),
            owner: app.v.clone(),
        }),
        max_prediction_time: 100.0f32,
    };

    let seek = Seek {
        behavior: RefCell::new(SteeringBehavior {
            enabled: true,
            limiter: None,
            target: target.clone(),
            owner: app.v.clone(),
        }),
    };

    let flee = Flee {
        behavior: RefCell::new(SteeringBehavior {
            enabled: true,
            limiter: None,
            target: target.clone(),
            owner: app.v.clone(),
        }),
    };

    let arrive = Arrive {
        behavior: RefCell::new(SteeringBehavior {
            enabled: true,
            limiter: None,
            target: target.clone(),
            owner: app.v.clone(),
        }),
        time_to_target: 10f32,
        deceleration_radius: 20f32,
        tolerance: 5f32,
    };

    let mut behaviors: Vec<Rc<RefCell<SteeringAccelerationCalculator<f32>>>> = vec![];
    behaviors.push(Rc::new(RefCell::new(seek)));
    behaviors.push(Rc::new(RefCell::new(flee)));
    behaviors.push(Rc::new(RefCell::new(pursue)));
    behaviors.push(Rc::new(RefCell::new(arrive)));
    behaviors.push(Rc::new(RefCell::new(evade)));

    app.behavior = Some(behaviors[0].clone());
    // First draw call
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    draw(&mut terminal, &app, target.clone());

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
                target.borrow_mut().advance_by_velocity(300.0f32);
                app.advance();
            }
        }
        app.behavior = Some(behaviors[app.selected].clone());
        draw(&mut terminal, &app, target.clone());
    }

    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<TermionBackend>, app: &App, target: Rc<RefCell<Vehicle>>) {
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
                        .block(Block::default().borders(border::ALL).title(
                            "Steering Behaviors",
                        ))
                        .items(&app.items)
                        .select(app.selected)
                        .highlight_style(
                            Style::default().fg(Color::Yellow).modifier(Modifier::Bold),
                        )
                        .highlight_symbol(">")
                        .render(t, &chunks[0]);

                    Canvas::default()
                        .block(Block::default().borders(border::ALL).title(
                            "Steering Actors",
                        ))
                        .paint(|ctx| {
                            /// draw steerable vehicle
                            ctx.draw(&Line {
                                x1: f64::from(
                                    app.v.borrow().get_position().as_slice()[0] - 10.0f32,
                                ),
                                y1: f64::from(
                                    app.v.borrow().get_position().as_slice()[1] + 10.0f32,
                                ),
                                x2: f64::from(
                                    app.v.borrow().get_position().as_slice()[0] + 10.0f32,
                                ),
                                y2: f64::from(
                                    app.v.borrow().get_position().as_slice()[1] + 10.0f32,
                                ),
                                color: Color::Red,
                            });
                            ctx.draw(&Line {
                                x1: f64::from(
                                    app.v.borrow().get_position().as_slice()[0] + 10.0f32,
                                ),
                                y1: f64::from(
                                    app.v.borrow().get_position().as_slice()[1] - 10.0f32,
                                ),
                                x2: f64::from(
                                    app.v.borrow().get_position().as_slice()[0] + 10.0f32,
                                ),
                                y2: f64::from(
                                    app.v.borrow().get_position().as_slice()[1] + 10.0f32,
                                ),
                                color: Color::Red,
                            });
                            ctx.draw(&Line {
                                x1: f64::from(
                                    app.v.borrow().get_position().as_slice()[0] - 10.0f32,
                                ),
                                y1: f64::from(
                                    app.v.borrow().get_position().as_slice()[1] - 10.0f32,
                                ),
                                x2: f64::from(
                                    app.v.borrow().get_position().as_slice()[0] + 10.0f32,
                                ),
                                y2: f64::from(
                                    app.v.borrow().get_position().as_slice()[1] - 10.0f32,
                                ),
                                color: Color::Red,
                            });
                            ctx.draw(&Line {
                                x1: f64::from(
                                    app.v.borrow().get_position().as_slice()[0] - 10.0f32,
                                ),
                                y1: f64::from(
                                    app.v.borrow().get_position().as_slice()[1] - 10.0f32,
                                ),
                                x2: f64::from(
                                    app.v.borrow().get_position().as_slice()[0] - 10.0f32,
                                ),
                                y2: f64::from(
                                    app.v.borrow().get_position().as_slice()[1] + 10.0f32,
                                ),
                                color: Color::Red,
                            });
                            /// draw target
                            ctx.draw(&Line {
                                x1: f64::from(
                                    target.borrow().get_position().as_slice()[0] - 10.0f32,
                                ),
                                y1: f64::from(
                                    target.borrow().get_position().as_slice()[1] + 10.0f32,
                                ),
                                x2: f64::from(
                                    target.borrow().get_position().as_slice()[0] + 10.0f32,
                                ),
                                y2: f64::from(
                                    target.borrow().get_position().as_slice()[1] + 10.0f32,
                                ),
                                color: Color::Green,
                            });
                            ctx.draw(&Line {
                                x1: f64::from(
                                    target.borrow().get_position().as_slice()[0] + 10.0f32,
                                ),
                                y1: f64::from(
                                    target.borrow().get_position().as_slice()[1] - 10.0f32,
                                ),
                                x2: f64::from(
                                    target.borrow().get_position().as_slice()[0] + 10.0f32,
                                ),
                                y2: f64::from(
                                    target.borrow().get_position().as_slice()[1] + 10.0f32,
                                ),
                                color: Color::Green,
                            });
                            ctx.draw(&Line {
                                x1: f64::from(
                                    target.borrow().get_position().as_slice()[0] - 10.0f32,
                                ),
                                y1: f64::from(
                                    target.borrow().get_position().as_slice()[1] - 10.0f32,
                                ),
                                x2: f64::from(
                                    target.borrow().get_position().as_slice()[0] + 10.0f32,
                                ),
                                y2: f64::from(
                                    target.borrow().get_position().as_slice()[1] - 10.0f32,
                                ),
                                color: Color::Green,
                            });
                            ctx.draw(&Line {
                                x1: f64::from(
                                    target.borrow().get_position().as_slice()[0] - 10.0f32,
                                ),
                                y1: f64::from(
                                    target.borrow().get_position().as_slice()[1] - 10.0f32,
                                ),
                                x2: f64::from(
                                    target.borrow().get_position().as_slice()[0] - 10.0f32,
                                ),
                                y2: f64::from(
                                    target.borrow().get_position().as_slice()[1] + 10.0f32,
                                ),
                                color: Color::Green,
                            });
                        })
                        .x_bounds([-180.0, 180.0])
                        .y_bounds([-180.0, 180.0])
                        .render(t, &chunks[1]);
                });

            List::default()
                .block(
                    Block::default()
                        .borders(border::ALL)
                        .title("Instructions")
                        .title_style(Style::default().fg(Color::White).bg(Color::Red).modifier(
                            Modifier::Bold,
                        )),
                )
                .items(&app.positions
                    .iter()
                    .map(|evt| (evt.to_owned(), &app.error_style))
                    .collect::<Vec<(String, &Style)>>())
                .render(t, &chunks[1]);
        });

    t.draw().unwrap();
}
