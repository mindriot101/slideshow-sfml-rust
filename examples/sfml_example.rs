extern crate sfml;

use sfml::graphics::{CircleShape, Color, Font, RenderTarget, RenderWindow, Sprite,
                     Text, Texture, Transformable};

use sfml::window::{Event, Key, Style};

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Borrowed resources",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let frank = Texture::from_file("run_tree/images/frank.jpeg").unwrap();

    let font = Font::from_file("run_tree/fonts/sansation.ttf").unwrap();

    let mut circle = CircleShape::with_texture(&frank);
    circle.set_radius(60.0);
    circle.set_position((100.0, 100.0));

    let mut sprite = Sprite::new();
    sprite.set_texture(&frank, true);
    sprite.set_position((400.0, 300.0));
    sprite.set_scale((0.5, 0.5));

    let mut title = Text::new("Borrowed resources example", &font, 50);
    title.set_position((50.0, 0.0));

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                _ => {},
            }
        }

        window.clear(&Color::BLACK);
        window.draw(&circle);
        window.draw(&sprite);
        window.draw(&title);
        window.display();
    }
}
