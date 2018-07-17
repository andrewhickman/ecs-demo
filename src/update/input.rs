use rand::distributions::Poisson;
use rand::{self, Rng};
use shrev::{EventChannel, ReaderId};
use specs::prelude::*;

use update::physics::{Ball, Paddle, Position, Velocity};
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
    event_rx: ReaderId<Event>,
    dist: Poisson,
}

impl InputSystem {
    pub const NAME: &'static str = "Input";

    pub fn new(world: &mut World) -> Self {
        InputSystem {
            event_rx: world
                .write_resource::<EventChannel<Event>>()
                .register_reader(),
            dist: Poisson::new(4.0),
        }
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Ball>,
        WriteStorage<'a, Axis>,
        ReadExpect<'a, EventChannel<Event>>,
    );

    fn run(
        &mut self,
        (entities, mut positions, mut velocities, mut balls, mut axis, event_tx): Self::SystemData,
    ) {
        for event in event_tx.read(&mut self.event_rx) {
            let input = match event {
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { input, .. },
                    ..
                } => input,
                _ => continue,
            };

            if input.virtual_keycode == Some(VirtualKeyCode::Space)
                && input.state == ElementState::Released
            {
                let radius = rand::thread_rng().sample(&self.dist) as f32;
                entities
                    .build_entity()
                    .with(Position::centre(), &mut positions)
                    .with(Velocity::random(), &mut velocities)
                    .with(Ball { radius }, &mut balls)
                    .build();
            } else {
                for (vel, ax) in (&mut velocities, &mut axis).join() {
                    if let Some(sign) = ax.update(*input) {
                        vel.set_y(sign * Paddle::SPEED);
                    }
                }
            }
        }
    }
}
