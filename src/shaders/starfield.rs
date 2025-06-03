use macroquad::prelude::{Material, MaterialParams, UniformType, load_material};

const FRAGMENT_SHADER: &str = include_str!("starfield-fragment-shader.glsl");
const VERTEX_SHADER: &str = include_str!("starfield-vertex-shader.glsl");

pub fn create_starfield_shader() -> Material {
    //let mut direction_modifier: f32 = 0.0;
    load_material(
        VERTEX_SHADER,
        FRAGMENT_SHADER,
        MaterialParams {
            uniforms: vec![
                ("iResolution".to_owned(), UniformType::Float2),
                ("direction_modifier".to_owned(), UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .unwrap()
}
