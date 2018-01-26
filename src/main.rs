extern crate sfml;

use std::collections::HashMap;
use sfml::system::Clock;
use sfml::graphics::{Color, Font, RenderTarget, RenderWindow, Text, Transformable};
use sfml::window::{Event, Key, Style};

pub(crate) mod traits;
pub(crate) mod slideshow;
pub(crate) mod slide;
pub(crate) mod component;

use traits::{OriginReset, Renderable, Updateable};
use slideshow::Slideshow;
use slide::Slide;

impl<'font> OriginReset for Text<'font> {
    fn reset_origin(&mut self) {
        let rect = self.global_bounds();
        self.set_origin((rect.width * 0.5, rect.height * 0.5));
    }
}

fn main() {
    /* Load resources */
    let mut fonts = HashMap::new();
    fonts.insert(
        "sansation",
        Font::from_file("run_tree/fonts/sansation.ttf").unwrap(),
    );

    let resolution = (1280, 720);

    /* Create the render window */
    let mut window = RenderWindow::new(
        resolution,
        "Borrowed resources",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    /* Create slideshow */
    let mut slideshow = Slideshow::new();
    slideshow.add(Slide::blank().add_text("Hello world", &fonts["sansation"], 84, (0.5, 0.5)));

    let mut clock = Clock::start();

    /* Main loop */
    loop {
        /* Handle events */
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                _ => {}
            }
        }

        match slideshow.slides.get_mut(slideshow.current_slide) {
            Some(slide) => {
                /* Check for updates */
                let dt = clock.restart();
                slide.update(dt.as_seconds(), window.size());

                /* Render the current slide */
                window.clear(&Color::BLACK);
                slide.draw(&mut window);
                window.display();
            }
            None => panic!("cannot get current slide"),
        }
    }
}
