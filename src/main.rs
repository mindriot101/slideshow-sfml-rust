extern crate sfml;

use std::collections::HashMap;
use sfml::system::Clock;
use sfml::graphics::{Color, Font, RenderTarget, RenderWindow, Shader, Text, Transformable};
use sfml::window::{Event, Key, Style, VideoMode};

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
    let green_shader = Shader::from_file(None, None, Some("run_tree/shaders/green.glslf"))
        .expect("loading green shader");

    let resolution = (1280, 720);

    /* Create the render window */
    let mut fullscreen = false;
    let mut window = RenderWindow::new(
        resolution,
        "Borrowed resources",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    /* Create slideshow */
    let mut slideshow = Slideshow::new();
    slideshow.add(
        Slide::blank()
            .add_text("Hello world", &fonts["sansation"], 84, (0.5, 0.5))
            .with_shader(&green_shader),
    );
    slideshow.add(Slide::blank().add_text("Second slide", &fonts["sansation"], 84, (0.3, 0.5)));
    let n_slides = slideshow.len();

    let mut clock = Clock::start();

    /* Main loop */
    loop {
        /* Handle events */
        while let Some(event) = window.poll_event() {
            match event {
                /* Quit */
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                }
                | Event::KeyPressed { code: Key::Q, .. } => return,

                /* Next slide */
                Event::KeyPressed { code: Key::N, .. }
                | Event::KeyPressed {
                    code: Key::Space, ..
                } => {
                    slideshow.current_slide = (slideshow.current_slide + 1).min(n_slides - 1);
                }

                /* Previous slide */
                Event::KeyPressed { code: Key::P, .. }
                | Event::KeyPressed {
                    code: Key::BackSpace,
                    ..
                } => {
                    if let Some(value) = slideshow.current_slide.checked_sub(1) {
                        slideshow.current_slide = value;
                    }
                }

                /* Fullscreen mode */
                Event::KeyPressed { code: Key::F, .. } => {
                    fullscreen = !fullscreen;
                    let (new_mode, new_style) = if fullscreen {
                        let new_mode = VideoMode::desktop_mode();
                        let new_style = Style::CLOSE | Style::FULLSCREEN;
                        ((new_mode.width, new_mode.height), new_style)
                    } else {
                        let new_style = Style::CLOSE;
                        (resolution, new_style)
                    };

                    window = RenderWindow::new(
                        new_mode,
                        "Borrowed resources",
                        new_style,
                        &Default::default(),
                    );
                }
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
