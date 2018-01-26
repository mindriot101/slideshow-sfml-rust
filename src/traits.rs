use sfml::system::Vector2u;
use sfml::graphics::{RenderTarget, Transformable};

pub(crate) trait Updateable {
    fn update(&mut self, time: f32, resolution: Vector2u);
}

pub(crate) trait Renderable {
    fn draw<T>(&self, target: &mut T)
    where
        T: RenderTarget;
}

// TODO: use this!
pub(crate) trait Slideable: Renderable + Updateable {}

pub(crate) trait OriginReset: Transformable {
    fn reset_origin(&mut self);
}
