use crate::{Cell, CellState, config::Config};
use rayon::prelude::*;
use coffee::{Game, input::{KeyboardAndMouse, mouse, keyboard}, graphics::{Batch, Window, Frame, Image, Color, Rectangle, Sprite, Point}, load::{Task, Join}, Timer};

const NEIGHBORS: [(f32, f32); 8] = [
    (-1.0, -1.0),
    (-1.0,  0.0),
    (-1.0,  1.0),
    ( 0.0, -1.0),
    ( 0.0,  1.0),
    ( 1.0, -1.0),
    ( 1.0,  0.0),
    ( 1.0,  1.0),
];

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
        return Task::using_gpu(|gpu| Image::from_colors(gpu, &[
            Color::RED,
            Color::BLUE,
        ]))
    }
}

impl Game for State {
    type Input = KeyboardAndMouse;
    type LoadingScreen = ();

    const TICKS_PER_SECOND: u16 = 20;

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
                let count = NEIGHBORS.iter().fold(0, |acc, neighbor| {
                    let target_point = Point::new(cell.position.x + neighbor.0, cell.position.y + neighbor.1);

                    if (target_point.x >= 0.0 && target_point.x < config.grid_size.0 as f32) && (target_point.y >= 0.0 && target_point.y < config.grid_size.1 as f32) {
                        let target_cell: CellState = cloned_cells.iter().filter(|c| c.position == target_point).cloned().collect::<Vec<Cell>>()[0].state;

                        if target_cell == CellState::ALIVE {
                            return acc + 1;
                        }
                    }

                    return acc;
                });

                match cell.state {
                    CellState::ALIVE => if count < 2 || count > 3 { 
                        cell.state = CellState::DEAD;
                    },
                    CellState::DEAD => if count == 3 { 
                        cell.state = CellState::ALIVE;
                     },
                    _ => (),
                }
            });
        }
    }

    fn draw(&mut self, frame: &mut Frame, timer: &Timer) {
        frame.clear(Color::BLACK);

        let sprites = self.cells.par_iter().map(|cell| {
            let position = cell.position * self.scale;

           return Sprite {
               source: Rectangle {
                   x: if cell.state == CellState::ALIVE { 1 } else { 0 },
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