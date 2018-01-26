use sfml::graphics::{Font, Shader, Sprite};

pub(crate) struct TextDescription<'font, 'texture: 'font> {
    pub text: String,
    pub font: &'font Font,
    pub size: usize,
    pub position: (f32, f32),
    pub shader: Option<&'texture Shader<'texture>>,
}

pub(crate) struct ImageDescription<'s, 'texture: 's> {
    pub sprite: Sprite<'s>,
    pub position: (f32, f32),
    pub shader: Option<&'texture Shader<'texture>>,
}
