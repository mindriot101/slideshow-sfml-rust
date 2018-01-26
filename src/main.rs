extern crate sfml;

use std::collections::HashMap;
use sfml::system::{Clock, Vector2u};
use sfml::graphics::{Color, Font, RenderTarget, RenderWindow, Text, Transformable};
use sfml::window::{Event, Key, Style};

trait Updateable {
    fn update(&mut self, time: f32, resolution: Vector2u);
}

trait Renderable {
    fn draw<T>(&self, target: &mut T)
    where
        T: RenderTarget;
}

// TODO: use this!
trait Slideable: Renderable + Updateable {}

enum ComponentType<'font> {
    TextComponent(Text<'font>),
}

struct Component<'font> {
    relative_position: (f32, f32),
    component_type: ComponentType<'font>,
}

trait OriginReset: Transformable {
    fn reset_origin(&mut self);
}

impl<'font> OriginReset for Text<'font> {
    fn reset_origin(&mut self) {
        let rect = self.global_bounds();
        self.set_origin((rect.width * 0.5, rect.height * 0.5));
    }
}

impl<'font> Component<'font> {
    fn text(text_s: &str, font: &'font Font, size: usize, position: (f32, f32)) -> Self {
        let mut text = Text::new(text_s, font, size as _);
        text.reset_origin();
        Component {
            relative_position: position,
            component_type: ComponentType::TextComponent(text),
        }
    }
}

impl<'font> Renderable for Component<'font> {
    fn draw<T>(&self, target: &mut T)
    where
        T: RenderTarget,
    {
        match self.component_type {
            ComponentType::TextComponent(ref text) => {
                target.draw(text);
            }
        }
    }
}

impl<'font> Updateable for Component<'font> {
    fn update(&mut self, _time: f32, resolution: Vector2u) {
        match self.component_type {
            ComponentType::TextComponent(ref mut text) => {
                let (newx, newy) = {
                    let x = self.relative_position.0 * resolution.x as f32;
                    let y = self.relative_position.1 * resolution.y as f32;
                    (x, y)
                };
                text.set_position((newx, newy));
            }
        }
    }
}

struct Slide<'font> {
    components: Vec<Component<'font>>,
}

impl<'font> Slide<'font> {
    fn blank() -> Self {
        Slide {
            components: Vec::new(),
        }
    }

    fn add_text(
        mut self,
        text: &str,
        font: &'font Font,
        size: usize,
        position: (f32, f32),
    ) -> Self {
        self.components
            .push(Component::text(text, font, size, position));
        self
    }
}

impl<'font> Renderable for Slide<'font> {
    fn draw<T>(&self, target: &mut T)
    where
        T: RenderTarget,
    {
        for component in &self.components {
            component.draw(target);
        }
    }
}

impl<'font> Updateable for Slide<'font> {
    fn update(&mut self, time: f32, resolution: Vector2u) {
        for component in self.components.iter_mut() {
            component.update(time, resolution);
        }
    }
}

struct Slideshow<'font> {
    current_slide: usize,
    slides: Vec<Slide<'font>>,
}

impl<'font> Slideshow<'font> {
    fn new() -> Self {
        Slideshow {
            current_slide: 0,
            slides: Vec::new(),
        }
    }

    fn add(&mut self, slide: Slide<'font>) {
        self.slides.push(slide);
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
