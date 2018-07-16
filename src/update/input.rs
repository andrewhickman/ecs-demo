use shrev::{EventChannel, ReaderId};
use specs::prelude::*;

use update::physics::{Paddle, Velocity};
use winit::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

pub struct Binding {
    pub state: ElementState,
    pub key: VirtualKeyCode,
}

#[derive(Component)]
pub struct Axis {
    pub pos: Binding,
    pub neg: Binding,
}

impl Axis {
    pub fn new(pos: VirtualKeyCode, neg: VirtualKeyCode) -> Self {
        Axis {
            pos: Binding {
                key: pos,
                state: ElementState::Released,
            },
            neg: Binding {
                key: neg,
                state: ElementState::Released,
            },
        }
    }
}

impl Axis {
    fn update(&mut self, input: KeyboardInput) -> Option<f32> {
        if let Some(key) = input.virtual_keycode {
            if key == self.pos.key {
                self.pos.state = input.state;
            } else if key == self.neg.key {
                self.neg.state = input.state;
            } else {
                return None;
            }
        }

        Some(match (self.pos.state, self.neg.state) {
            (ElementState::Released, ElementState::Released) => 0.0,
            (ElementState::Pressed, ElementState::Released) => 1.0,
            (ElementState::Released, ElementState::Pressed) => -1.0,
            (ElementState::Pressed, ElementState::Pressed) => 0.0,
        })
    }
}

pub struct InputSystem {
    pub event_rx: ReaderId<Event>,
}

impl InputSystem {
    pub const NAME: &'static str = "Input";
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Axis>,
        ReadExpect<'a, EventChannel<Event>>,
    );

    fn run(&mut self, (mut velocities, mut axis, event_tx): Self::SystemData) {
        for event in event_tx.read(&mut self.event_rx) {
            let input = match event {
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { input, .. },
                    ..
                } => input,
                _ => continue,
            };

            for (vel, ax) in (&mut velocities, &mut axis).join() {
                if let Some(sign) = ax.update(*input) {
                    vel.set_y(sign * Paddle::SPEED);
                }
            }
        }
    }
}
