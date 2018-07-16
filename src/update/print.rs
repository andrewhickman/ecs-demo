use std::fmt::{self, Display};
use std::str;

use console::Term;
use specs::prelude::*;
use utils::throw;
use winit::{ElementState, VirtualKeyCode};

use super::*;

pub struct PrintSystem {
    term: Term,
    pos: usize,
}

impl PrintSystem {
    pub const NAME: &'static str = "Print";

    pub fn new() -> Self {
        PrintSystem {
            term: Term::stdout(),
            pos: 0,
        }
    }
}

impl<'a> System<'a> for PrintSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, Paddle>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Axis>,
    );

    fn run(&mut self, (entities, positions, velocities, paddles, balls, axis): Self::SystemData) {
        const COLUMN_0: usize = 6;
        const COLUMN_1: usize = 20;
        const COLUMN_2: usize = 20;
        const COLUMN_3: usize = 20;
        const COLUMN_4: usize = 20;
        const COLUMN_5: usize = 80;

        self.term.clear_last_lines(self.pos).unwrap_or_else(throw);
        self.pos = 0;

        println!(
            "{}┃{}│{}│{}│{}│{}",
            str::repeat(" ", COLUMN_0),
            fmt_header("Position", COLUMN_1),
            fmt_header("Velocity", COLUMN_2),
            fmt_header("Paddle", COLUMN_3),
            fmt_header("Ball", COLUMN_4),
            fmt_header("Keybind", COLUMN_5)
        );
        self.pos += 1;
        println!(
            "{}╋{}┿{}┿{}┿{}┿{}",
            str::repeat("━", COLUMN_0),
            str::repeat("━", COLUMN_1),
            str::repeat("━", COLUMN_2),
            str::repeat("━", COLUMN_3),
            str::repeat("━", COLUMN_4),
            str::repeat("━", COLUMN_5)
        );
        self.pos += 1;
        let mut first = true;
        for ent in (&*entities).join() {
            if !first {
                println!(
                    "{}╂{}┼{}┼{}┼{}┼{}",
                    str::repeat("─", COLUMN_0),
                    str::repeat("─", COLUMN_1),
                    str::repeat("─", COLUMN_2),
                    str::repeat("─", COLUMN_3),
                    str::repeat("─", COLUMN_4),
                    str::repeat("─", COLUMN_5),
                );
                self.pos += 1;
            }
            println!(
                "{}┃{}│{}│{}│{}│{}",
                fmt_value(Some(&ent.id()), COLUMN_0),
                fmt_value(positions.get(ent), COLUMN_1),
                fmt_value(velocities.get(ent), COLUMN_2),
                fmt_value(paddles.get(ent), COLUMN_3),
                fmt_value(balls.get(ent), COLUMN_4),
                fmt_value(axis.get(ent), COLUMN_5),
            );
            first = false;
            self.pos += 1;
        }
        println!();
        self.pos += 1;
    }
}

fn fmt_header(name: &'static str, column_width: usize) -> String {
    format!("{: ^1$.1$}", name, column_width)
}

fn fmt_value(obj: Option<&impl Display>, column_width: usize) -> String {
    match obj {
        Some(obj) => format!(" {: <1$.1$} ", obj.to_string(), column_width - 2),
        None => str::repeat(" ", column_width),
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.0.x, self.0.y)
    }
}

impl Display for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.0.x, self.0.y)
    }
}

impl Display for Paddle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ length = {} }}", self.length)
    }
}

impl Display for Ball {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ radius = {} }}", self.radius)
    }
}

impl Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ +ve = {{ key = {}, state = {} }}, -ve = {{ key = {}, state = {} }} }}",
            fmt_key(self.pos.key),
            fmt_state(self.pos.state),
            fmt_key(self.neg.key),
            fmt_state(self.pos.state)
        )
    }
}

fn fmt_key(key: VirtualKeyCode) -> &'static str {
    match key {
        VirtualKeyCode::Q => "q",
        VirtualKeyCode::A => "a",
        VirtualKeyCode::O => "o",
        VirtualKeyCode::L => "l",
        _ => unimplemented!(),
    }
}

fn fmt_state(state: ElementState) -> &'static str {
    match state {
        ElementState::Pressed => "pressed",
        ElementState::Released => "released",
    }
}
