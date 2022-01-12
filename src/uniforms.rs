use glium::uniforms::{AsUniformValue, UniformValue, Uniforms};

pub struct UStorage<'a> {
    pub mapping: std::collections::HashMap<&'a str, &'a dyn AsUniformValue>,
}

impl<'b> Uniforms for UStorage<'b> {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut call: F) {
        for (key, value) in self.mapping.iter() {
            call(key, value.as_uniform_value());
        }
    }
}
