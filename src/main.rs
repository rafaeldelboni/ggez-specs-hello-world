extern crate ggez;
extern crate specs;

mod systems;
mod components;
mod entities;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use specs::{Dispatcher, DispatcherBuilder, World, RunNow};

use systems::{ControlSystem, RenderingSystem, MoveSystem};

struct MainState<'a, 'b> {
    frames: usize,
    world: World,
    system_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new(ctx: &mut Context) -> GameResult<MainState<'a, 'b>> {
        graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);

        let mut world = World::new();

        components::register_components(&mut world);

        let system_dispatcher: Dispatcher<'a, 'b> = DispatcherBuilder::new()
            .with(
                MoveSystem,
                "move_system",
                &[]
            )
            .build();

        entities::create_static(&mut world);
        entities::create_moving(&mut world);
        entities::create_controled(&mut world);

        let state = MainState {
            frames: 0,
            world,
            system_dispatcher,
        };

        Ok(state)
    }
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.system_dispatcher.dispatch(&self.world.res);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        {
            let mut rs = RenderingSystem::new(ctx);
            rs.run_now(&mut self.world.res);
        }

        graphics::present(ctx);

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: event::Keycode,
        _keymod: event::Mod,
        _repeat: bool
    ) {
        let mut cs = ControlSystem::new(keycode, true);
        cs.run_now(&mut self.world.res);
    }

    fn key_up_event(
        &mut self,
        _context: &mut Context,
        keycode: event::Keycode,
        _keymod: event::Mod,
        _repeat: bool

    ) {
        let mut cs = ControlSystem::new(keycode, false);
        cs.run_now(&mut self.world.res);
    }
}

fn main() {
    let c = conf::Conf::new();
    println!("Starting with default config: {:#?}", c);

    let ctx = &mut Context::load_from_conf("ggez-specs-hello-world", "ggez", c).unwrap();

    match MainState::new(ctx) {
        Ok(ref mut game) => {
            let result = event::run(ctx, game);
            if let Err(e) = result {
                println!("Error encountered running game: {}", e);
            } else {
                println!("Game exited cleanly.");
            }
        }
        Err(e) => {
            println!("Could not load game!");
            println!("Error: {}", e);
        }
    }
}
