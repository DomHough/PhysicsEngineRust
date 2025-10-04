use crate::color::Color;
use crate::vec3::Vec3;

pub fn phong_shade(
    normal: Vec3,
    light_dir: Vec3,
    view_dir: Vec3,
    light_color: Color,
    material_ambient: Color,
    material_diffuse: Color,
    material_specular: Color,
    shininess: f32,
    ambient_light: Color
) -> Color {
    // Normalize input vectors
    let n = normal.normalized();
    let l = light_dir.normalized();
    let v = view_dir.normalized();

    // Ambient component
    let ambient = ambient_light * material_ambient;

    // Diffuse component
    let ndotl = n.dot(&l).max(0.0);
    let diffuse = material_diffuse * ndotl;

    let specular = if ndotl > 0.0 {
        let r = (n * (2.0 * n.dot(&l)) - l).normalized();
        let s = r.dot(&v).max(0.0).powf(shininess.max(0.0));
        material_specular * s
    } else {
        Color::new(0.0, 0.0, 0.0, 0.0)
    };

    // Final color
    let color = ambient + (diffuse + specular) * light_color;
    color
}