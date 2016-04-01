extern crate sdl2;
extern crate time;

mod game;

use sdl2::pixels::Color;

use game::{Game, View, ViewAction};

struct DefaultView;

impl View for DefaultView {
    fn update(&mut self, game: &mut Game, _: u32) -> ViewAction {
        if game.events.now.quit || game.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        ViewAction::None
    }

    fn render(&mut self, game: &mut Game, _: u32) {
        game.renderer.set_draw_color(Color::RGB(0, 0, 0));
        game.renderer.clear();
    }
}

fn main() {
    let mut game = Game::new("Rustboy", || {
        Some(Box::new(DefaultView))
    });

    game.run();
}
