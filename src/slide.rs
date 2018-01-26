use component::Component;

use traits::{Renderable, Updateable};
use sfml::graphics::{Font, RenderTarget};
use sfml::system::Vector2u;

pub(crate) struct Slide<'font> {
    components: Vec<Component<'font>>,
}

impl<'font> Slide<'font> {
    pub(crate) fn blank() -> Self {
        Slide {
            components: Vec::new(),
        }
    }

    pub(crate) fn add_text(
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
