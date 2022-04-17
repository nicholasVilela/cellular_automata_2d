use coffee::{
    graphics::WindowSettings, 
    Game,
    Result,
};

mod config;
use config::*;

mod state;
use state::*;

mod cell;
use cell::*;

fn main() -> Result<()> {
    let config = Config::load();

    return State::run(WindowSettings {
        title: config.title,
        size: config.size,
        resizable: config.resizable,
        fullscreen: config.fullscreen,
        maximized: config.maximized,
    });
}
