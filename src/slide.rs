use component::Component;

use traits::{Renderable, Updateable};
use description::TextDescription;
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

    pub(crate) fn add_text(mut self, desc: TextDescription<'font, 'texture>) -> Self {
        self.components.push(Component::text(
            desc.text,
            desc.font,
            desc.size,
            desc.position,
            desc.shader,
        ));
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
