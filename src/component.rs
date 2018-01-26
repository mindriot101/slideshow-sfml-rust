use traits::{OriginReset, Renderable, Updateable};
use sfml::system::Vector2u;
use sfml::graphics::{Font, RenderTarget, Text, Transformable};

pub(crate) enum ComponentType<'font> {
    TextComponent(Text<'font>),
}

pub(crate) struct Component<'font> {
    relative_position: (f32, f32),
    component_type: ComponentType<'font>,
}

impl<'font> Component<'font> {
    pub(crate) fn text(text_s: &str, font: &'font Font, size: usize, position: (f32, f32)) -> Self {
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
