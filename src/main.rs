extern crate cgmath;
extern crate console;
extern crate env_logger;
extern crate lime_main_loop as main_loop;
extern crate lime_render;
extern crate lime_utils as utils;
extern crate shrev;
extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate rand;
extern crate winit;

mod app;
mod render;
mod update;

use winit::dpi::LogicalSize;

const ARENA_SIZE: LogicalSize = LogicalSize {
    width: 1024.0,
    height: 768.0,
};

fn main() {
    utils::set_panic_hook();
    env_logger::init();

    app::run();
}
