use component::Component;

use traits::{Renderable, Updateable};
use description::{ImageDescription, TextDescription};
use sfml::graphics::RenderTarget;
use sfml::system::Vector2u;

pub(crate) struct Slide<'font, 's, 'texture> {
    components: Vec<Component<'font, 's, 'texture>>,
}

impl<'font, 's, 'texture> Slide<'font, 's, 'texture> {
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

    pub(crate) fn add_image(mut self, desc: ImageDescription<'s, 'texture>) -> Self {
        self.components
            .push(Component::image(desc.sprite, desc.position, desc.shader));
        self
    }
}

impl<'font, 's, 'texture> Renderable for Slide<'font, 's, 'texture> {
    fn draw<T>(&self, target: &mut T)
    where
        T: RenderTarget,
    {
        for component in &self.components {
            component.draw(target);
        }
    }
}

impl<'font, 's, 'texture> Updateable for Slide<'font, 's, 'texture> {
    fn update(&mut self, time: f32, resolution: Vector2u) {
        for component in self.components.iter_mut() {
            component.update(time, resolution);
        }
    }
}
