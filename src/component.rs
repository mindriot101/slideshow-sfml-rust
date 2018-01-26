use traits::{OriginReset, Renderable, Updateable};
use sfml::system::Vector2u;
use sfml::graphics::{Font, RenderStates, RenderTarget, Shader, Sprite, Text, Transformable};

pub(crate) enum ComponentType<'font, 's> {
    TextComponent(Text<'font>),
    ImageComponent(Sprite<'s>),
}

pub(crate) struct Component<'font, 's, 'texture> {
    relative_position: (f32, f32),
    component_type: ComponentType<'font, 's>,
    shader: Option<&'texture Shader<'texture>>,
}

impl<'font, 's, 'texture> Component<'font, 's, 'texture> {
    pub(crate) fn text(
        text_s: String,
        font: &'font Font,
        size: usize,
        position: (f32, f32),
        shader: Option<&'texture Shader<'texture>>,
    ) -> Self {
        let mut text = Text::new(&text_s, font, size as _);
        text.reset_origin();
        Component {
            relative_position: position,
            component_type: ComponentType::TextComponent(text),
            shader: shader,
        }
    }

    pub(crate) fn image<'a>(
        mut sprite: Sprite<'s>,
        position: (f32, f32),
        shader: Option<&'texture Shader<'texture>>,
    ) -> Self {
        sprite.reset_origin();
        Component {
            relative_position: position,
            component_type: ComponentType::ImageComponent(sprite),
            shader: shader,
        }
    }
}

impl<'font, 's, 'texture> Updateable for Component<'font, 's, 'texture> {
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
            ComponentType::ImageComponent(ref mut sprite) => {
                let (newx, newy) = {
                    let x = self.relative_position.0 * resolution.x as f32;
                    let y = self.relative_position.1 * resolution.y as f32;
                    (x, y)
                };
                sprite.set_position((newx, newy));
            }
        }
    }
}

impl<'font, 's, 'texture> Renderable for Component<'font, 's, 'texture> {
    fn draw<T>(&self, target: &mut T)
    where
        T: RenderTarget,
    {
        match self.component_type {
            ComponentType::TextComponent(ref text) => {
                let mut states = RenderStates::default();
                states.shader = self.shader;
                target.draw_with_renderstates(text, states);
            }
            ComponentType::ImageComponent(ref sprite) => {
                let mut states = RenderStates::default();
                states.shader = self.shader;
                target.draw_with_renderstates(sprite, states);
            }
        }
    }
}
