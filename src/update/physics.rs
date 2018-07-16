use std::f32::consts::PI;

use cgmath::prelude::*;
use cgmath::{Point2, Vector2};
use rand;
use specs::prelude::*;

use ARENA_SIZE;

#[derive(Component)]
pub struct Position(pub Point2<f32>);

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position(Point2::new(x, y))
    }

    pub fn centre() -> Self {
        Position::new(
            ARENA_SIZE.width as f32 / 2.0,
            ARENA_SIZE.height as f32 / 2.0,
        )
    }
}

#[derive(Component)]
pub struct Velocity(pub Vector2<f32>);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        let vec = Vector2::new(x, y);
        assert!(vec.magnitude() < Paddle::WIDTH);
        Velocity(vec)
    }

    pub fn set_y(&mut self, y: f32) {
        self.0.y = y;
        assert!(self.0.magnitude() < Paddle::WIDTH);
    }

    pub fn random() -> Self {
        let angle = 2.0 * rand::random::<f32>() * PI;
        let (sin, cos) = angle.sin_cos();
        Velocity(Vector2::new(cos, sin) * 15.0)
    }
}

#[derive(Component)]
pub struct Ball {
    pub radius: f32,
}

#[derive(Component)]
pub struct Paddle {
    pub length: f32,
}

impl Paddle {
    pub const SPEED: f32 = 10.0;
    pub const WIDTH: f32 = 20.0;

    fn contains(&self, pos: Point2<f32>, ball: Point2<f32>, radius: f32) -> bool {
        let relative_pos = ball - pos;
        let total_width = (Self::WIDTH / 2.0) + radius;
        let total_height = (self.length / 2.0) + radius;
        -total_width < relative_pos.x
            && relative_pos.x < total_width
            && -total_height < relative_pos.y
            && relative_pos.y < total_height
    }
}

pub struct PhysicsSystem;

impl PhysicsSystem {
    pub const NAME: &'static str = "Physics";
}

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut positions, velocities): Self::SystemData) {
        for (pos, vel) in (&mut positions, &velocities).join() {
            pos.0 += vel.0;
            pos.0.x = pos.0.x.max(0.0).min(ARENA_SIZE.width as f32);
            pos.0.y = pos.0.y.max(0.0).min(ARENA_SIZE.height as f32);
        }
    }
}

pub struct CollisionSystem;

impl CollisionSystem {
    pub const NAME: &'static str = "Collision";
}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Paddle>,
    );

    fn run(&mut self, (positions, mut velocities, balls, paddles): Self::SystemData) {
        for (ball_pos, vel, ball) in (&positions, &mut velocities, &balls).join() {
            for (pad_pos, paddle) in (&positions, &paddles).join() {
                if paddle.contains(pad_pos.0, ball_pos.0, ball.radius) {
                    vel.0.x = -vel.0.x;
                }
            }

            if ball_pos.0.y == 0.0 || ball_pos.0.y == ARENA_SIZE.height as f32 {
                vel.0.y = -vel.0.y;
            }
        }
    }
}

pub struct ScoreSystem;

impl ScoreSystem {
    pub const NAME: &'static str = "Score";
}

impl<'a> System<'a> for ScoreSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Ball>,
    );

    fn run(&mut self, (mut positions, mut velocities, balls): Self::SystemData) {
        for (pos, vel, _) in (&mut positions, &mut velocities, &balls).join() {
            if pos.0.x == 0.0 || pos.0.x == ARENA_SIZE.width as f32 {
                *pos = Position::centre();
                *vel = Velocity::random();
            }
        }
    }
}
