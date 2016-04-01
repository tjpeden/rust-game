#[macro_use]
mod events;
mod view;

use std::thread;
use std::time::{Duration, Instant};

// use time::{Duration, PreciseTime};
use sdl2::render::Renderer;

pub use self::view::{View, ViewAction};

events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_left: Left,
        key_right: Right,
        key_space: Space
    },
    other: {
        quit: Quit { .. }
    }
}

pub struct Game<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,
    current_view: Option<Box<View>>,
}

impl<'window> Game<'window> {
    pub fn new<F: Fn() -> Option<Box<View>>>(title: &str, init: F) -> Self {
        let sdl = ::sdl2::init().unwrap();
        let video = sdl.video().unwrap();

        let window = video.window(title, 640, 480)
            .position_centered()
            .opengl()
            .resizable()
            .build()
            .unwrap();

        Game {
            events: Events::new(sdl.event_pump().unwrap()),
            renderer: window.renderer().accelerated().build().unwrap(),
            current_view: init(),
        }
    }

    pub fn run(&mut self) {
        let second = Duration::new(1, 0);
        let interval = second / 60;
        let mut before = Instant::now();
        let mut last_second = before;
        let mut fps = 0u16;

        loop {
            let now = Instant::now();
            let difference = now.elapsed();
            let elapsed = difference.subsec_nanos() / 10;

            if difference < interval {
                thread::sleep(interval - difference);
                continue;
            }

            before = now;
            fps += 1;

            if now.duration_from_earlier(last_second) > second {
                println!("FPS: {}", fps);
                last_second = now;
                fps = 0;
            }

            match self.update(elapsed) {
                ViewAction::None => {
                    self.render(elapsed);
                }

                ViewAction::Quit => {
                    break;
                }

                ViewAction::ChangeView(new_view) => {
                    self.current_view = Some(new_view);
                }
            }
        }
    }

    fn update(&mut self, elapsed: u32) -> ViewAction {
        if let Some(current_view) = self.current_view {
            self.events.pump(&mut self.renderer);

            current_view.update(self, elapsed)
        } else {
            ViewAction::None
        }
    }

    fn render(&mut self, elapsed: u32) {
        if let Some(current_view) = self.current_view {
            current_view.render(self, elapsed);

            self.renderer.present();
        }
    }
}