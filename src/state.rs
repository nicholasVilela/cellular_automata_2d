use crate::{Cell, CellState, config::Config};
use rayon::prelude::*;
use coffee::{Game, input::{KeyboardAndMouse, mouse, keyboard}, graphics::{Batch, Window, Frame, Image, Color, Rectangle, Sprite, Point}, load::{Task, Join}, Timer};

pub struct State {
    cells: Vec<Cell>,
    batch: Batch,
    config: Config,
    scale: f32,
    paused: bool,
}

impl State {
    fn generate(config: Config) -> Task<Vec<Cell>> {
        return Task::succeed(move || {
            let rng = &mut rand::thread_rng();
            let mut cells: Vec<Cell> = Vec::new();

            for y in 0..config.grid_size.1 {
                for x in 0..config.grid_size.0 {
                    let cell = Cell::random(Point::new(x as f32, y as f32), rng);
                    cells.push(cell);
                }
            }

            return cells;
        });
    }

    fn load_palette() -> Task<Image> {
        return Task::using_gpu(|gpu| Image::from_colors(gpu, &[Color::GREEN]))
    }
}

impl Game for State {
    type Input = KeyboardAndMouse;
    type LoadingScreen = ();

    const TICKS_PER_SECOND: u16 = 144;

    fn load(_window: &Window,) -> Task<State> {
        let config = Config::load();

        return (
            Task::stage("Generating Grid...", Self::generate(config.clone())),
            Task::stage("Loading assets...", Self::load_palette())
        )
            .join()
            .map(|(cells, palette)| State {
                cells,
                batch: Batch::new(palette),
                scale: config.clone().size.0 as f32 / config.clone().grid_size.0 as f32,
                config,
                paused: false,
            });
    }

    fn interact(&mut self, input: &mut KeyboardAndMouse, _window: &mut Window) {
        let mouse = input.mouse();
        let keyboard = input.keyboard();

        if mouse.is_button_pressed(mouse::Button::Left) {
            let mut target_point = mouse.cursor_position() / self.scale;
            target_point.x = target_point.x.floor();
            target_point.y = target_point.y.floor();

            let index = (target_point.x + (target_point.y * self.config.grid_size.0 as f32)) as usize;

            if index < self.cells.len() {
                self.cells[index].state = CellState::ALIVE;
            }
        }

        if mouse.is_button_pressed(mouse::Button::Right) {
            let mut target_point = mouse.cursor_position() / self.scale;
            target_point.x = target_point.x.floor();
            target_point.y = target_point.y.floor();

            let index = (target_point.x + (target_point.y * self.config.grid_size.0 as f32)) as usize;

            if index < self.cells.len() {
                self.cells[index].state = CellState::DEAD;
            }
        }

        if keyboard.was_key_released(keyboard::KeyCode::P) {
            self.paused = !self.paused;
        }
    }

    fn update(&mut self, _window: &Window) {
        if !self.paused {
            let cloned_cells = self.cells.clone();
            let config = &self.config;

            self.cells.par_iter_mut().for_each(move |cell| {
                let mut count = 0;

                for &x_off in [-1, 0, 1].iter() {
                    for &y_off in [-1, 0, 1].iter() {
                        if x_off == 0 && y_off == 0 {
                            continue;
                        }
                    
                        let neighbor_position = Point::new(cell.position.x + x_off as f32, cell.position.y + y_off as f32);
                        if neighbor_position.x < 0.0
                            || neighbor_position.x > config.grid_size.0 as f32 - 1.0
                            || neighbor_position.y < 0.0
                            || neighbor_position.y > config.grid_size.1 as f32  -1.0 {
                            continue;
                        }
                    
                        let neighbor_index = (neighbor_position.x + (neighbor_position.y * config.grid_size.0 as f32)) as usize;
                    
                        if cloned_cells[neighbor_index].state == CellState::ALIVE {
                            count += 1;
                        }
                    }
                }

                match cell.state {
                    CellState::ALIVE => if count < 2 || count > 3 { 
                        cell.state = CellState::DEAD;
                    },
                    CellState::DEAD => if count == 3 { 
                        cell.state = CellState::ALIVE;
                     },
                }
            });
        }
    }

    fn draw(&mut self, frame: &mut Frame, timer: &Timer) {
        frame.clear(Color::BLACK);

        let sprites = self.cells
            .par_iter()
            .filter(|cell| cell.state == CellState::ALIVE)
            .map(|cell| {
                let position = cell.position * self.scale;

                return Sprite {
                    source: Rectangle {
                        x: 0,
                        y: 0,
                        width: 1,
                        height:1,
                    },
                    position: Point::new(position.x , position.y),
                    scale: (self.scale, self.scale),
                } 
        });

        self.batch.clear();
        self.batch.par_extend(sprites);
        self.batch.draw(&mut frame.as_target());
    }
}