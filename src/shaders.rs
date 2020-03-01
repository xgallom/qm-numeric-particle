pub fn vertex_shader() -> &'static str {
    std::include_str!("shaders/vertex_shader.glsl")
}

pub fn fragment_shader() -> &'static str {
    std::include_str!("shaders/fragment_shader.glsl")
}
