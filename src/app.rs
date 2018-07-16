use std::time::Duration;

use shrev::EventChannel;
use specs::prelude::*;
use winit::{ControlFlow, Event, EventsLoop, WindowBuilder, WindowEvent};
use {lime_render, main_loop};

use render::{DummySystem, RenderSystem};
use update;
use ARENA_SIZE;

struct App {
    update: Dispatcher<'static, 'static>,
    render: Dispatcher<'static, 'static>,
    world: World,
}

impl App {
    fn new(events_loop: &EventsLoop) -> Self {
        let mut world = World::new();

        let window = WindowBuilder::new()
            .with_dimensions(ARENA_SIZE)
            .with_resizable(false);
        let mut render = DispatcherBuilder::new()
            .with(DummySystem, DummySystem::NAME, &[])
            .with(RenderSystem, RenderSystem::NAME, &[]);
        lime_render::init(
            &mut world,
            &mut render,
            events_loop,
            window,
            DummySystem::NAME,
            RenderSystem::NAME,
        );
        let render = render.build();

        let mut update = DispatcherBuilder::new();
        update::init(&mut world, &mut update);
        let update = update.build();

        App {
            update,
            render,
            world,
        }
    }
}

impl main_loop::App for App {
    const UPDATES_PER_SECOND: u32 = 20;
    const RENDERS_PER_SECOND: u32 = 60;

    fn update(&mut self, dt: Duration) {
        self.update.dispatch(&self.world.res);
        println!("updating {:?}", dt);
    }

    fn render(&mut self, dt: Duration) {
        self.render.dispatch(&self.world.res);
        println!("rendering {:?}", dt);
    }

    fn event(&mut self, event: Event) -> ControlFlow {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => ControlFlow::Break,
            event => {
                self.world
                    .write_resource::<EventChannel<_>>()
                    .single_write(event);
                ControlFlow::Continue
            }
        }
    }
}

pub fn run() {
    main_loop::run(App::new)
}
