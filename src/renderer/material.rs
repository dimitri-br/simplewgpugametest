use crate::Texture;

pub struct Material{
    texture: Texture,
    shininess: f32,
    metallic: f32,
}

impl Material{
    pub fn new(texture: Texture, shininess: f32, metallic: f32) -> Self{
        Self{
            texture,
            shininess,
            metallic
        }
    }
}