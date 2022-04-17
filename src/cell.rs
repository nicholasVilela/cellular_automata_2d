use coffee::graphics::Point;
use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CellState {
    ALIVE,
    DEAD,
}

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub position: Point,
    pub state: CellState,
}

impl Cell {
    pub fn random<R: Rng>(position: Point, rng: &mut R) -> Cell {
        let state = if rng.gen_bool(0.5) { CellState::ALIVE } else { CellState::DEAD };

        return Cell {
            position,
            state,
        };
    }
}