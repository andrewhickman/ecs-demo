pub mod physics;

mod input;
mod print;

use specs::prelude::*;
use winit::VirtualKeyCode as KeyCode;

use self::input::*;
use self::physics::*;
use self::print::*;
use ARENA_SIZE;

pub fn init(world: &mut World, dispatcher: &mut DispatcherBuilder) {
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Ball>();
    world.register::<Paddle>();
    world.register::<Axis>();

    world
        .create_entity()
        .with(Position::new(0.0, ARENA_SIZE.height as f32 / 2.0))
        .with(Velocity::new(0.0, 0.0))
        .with(Paddle { length: 200.0 })
        .with(Axis::new(KeyCode::A, KeyCode::Q))
        .build();

    world
        .create_entity()
        .with(Position::new(
            ARENA_SIZE.width as f32,
            ARENA_SIZE.height as f32 / 2.0,
        ))
        .with(Velocity::new(0.0, 0.0))
        .with(Paddle { length: 200.0 })
        .with(Axis::new(KeyCode::L, KeyCode::O))
        .build();

    dispatcher.add(PrintSystem::new(), PrintSystem::NAME, &[]);
    dispatcher.add(InputSystem::new(world), InputSystem::NAME, &[]);
    dispatcher.add(PhysicsSystem, PhysicsSystem::NAME, &[InputSystem::NAME]);
    dispatcher.add(
        CollisionSystem,
        CollisionSystem::NAME,
        &[PhysicsSystem::NAME],
    );
    dispatcher.add(DeleteSystem, DeleteSystem::NAME, &[CollisionSystem::NAME]);
}
