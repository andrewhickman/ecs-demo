use std::f32::consts::PI;
use std::mem;

use cgmath::{Point2, Vector2};
use lime_render::{d2, d2::Point, Color};
use specs::prelude::*;

use update::physics::{Ball, Paddle, Position};

pub struct DummySystem;

impl DummySystem {
    pub const NAME: &'static str = "Dummy";
}

impl<'a> System<'a> for DummySystem {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {}
}

pub struct RenderSystem;

impl RenderSystem {
    pub const NAME: &'static str = "Render";
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        WriteExpect<'a, d2::Renderer>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Paddle>,
        ReadStorage<'a, Ball>,
    );

    fn run(&mut self, (mut renderer, positions, paddles, balls): Self::SystemData) {
        for (pos, pad) in (&positions, &paddles).join() {
            renderer.queue_tri(&rect(pos.0, Paddle::WIDTH, pad.length), Color::GREEN);
        }

        for (pos, ball) in (&positions, &balls).join() {
            for tri in circle(pos.0, ball.radius) {
                renderer.queue_tri(&tri, Color::GREEN);
            }
        }
    }
}

pub fn rect(centre: Point2<f32>, width: f32, height: f32) -> [Point; 6] {
    [
        Point(centre.x - width / 2.0, centre.y - height / 2.0),
        Point(centre.x + width / 2.0, centre.y - height / 2.0),
        Point(centre.x - width / 2.0, centre.y + height / 2.0),
        Point(centre.x - width / 2.0, centre.y + height / 2.0),
        Point(centre.x + width / 2.0, centre.y - height / 2.0),
        Point(centre.x + width / 2.0, centre.y + height / 2.0),
    ]
}

pub fn circle(centre: Point2<f32>, radius: f32) -> impl IntoIterator<Item = [Point; 3]> {
    const N: usize = 12;

    (1..=N)
        .map(|n| 2.0 * n as f32 * PI / N as f32)
        .map(f32::sin_cos)
        .map(move |(sin, cos)| centre + Vector2::<f32>::new(cos, sin) * radius)
        .map(|vec| Point(vec.x, vec.y))
        .scan(Point(centre.x + radius, centre.y), move |prev, cur| {
            Some([Point(centre.x, centre.y), mem::replace(prev, cur), cur])
        })
}
