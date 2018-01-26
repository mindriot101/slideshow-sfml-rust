use component::Component;

use traits::{Renderable, Updateable};
use sfml::graphics::{Font, RenderTarget, Shader};
use sfml::system::Vector2u;

pub(crate) struct Slide<'font, 'texture> {
    components: Vec<Component<'font, 'texture>>,
}

impl<'font, 'texture> Slide<'font, 'texture> {
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

    pub(crate) fn with_shader(mut self, shader: &Shader) -> Self {
        self
    }
}

impl<'font, 'texture> Renderable for Slide<'font, 'texture> {
    fn draw<T>(&self, target: &mut T)
    where
        T: RenderTarget,
    {
        for component in &self.components {
            component.draw(target);
        }
    }
}

impl<'font, 'texture> Updateable for Slide<'font, 'texture> {
    fn update(&mut self, time: f32, resolution: Vector2u) {
        for component in self.components.iter_mut() {
            component.update(time, resolution);
        }
    }
}
