use crate::color::Color;
use crate::consts::EPS;
use crate::vec3::Vec3;
use crate::light::{AmbientLight, Light}; // for multi-light shading
use crate::material::Material; // moved Material to its own file

// Tunable attenuation constants (smaller => brighter at distance)
const ATTEN_LINEAR: f32 = 0.05;   // was effectively 0 with only quadratic term before
const ATTEN_QUAD: f32 = 0.01;     // reduced from implicit 0.1 (much less aggressive)

pub fn shade_multi_light(
    normal: Vec3,
    view_dir: Vec3,
    point: Vec3,
    lights: &[&dyn Light],
    material: &Material,
    ambient_light: &AmbientLight,
) -> Color {
    let n = normal.normalized();
    let v = view_dir.normalized();
    let mut r_acc = ambient_light.color.r * ambient_light.intensity * material.ambient.r;
    let mut g_acc = ambient_light.color.g * ambient_light.intensity * material.ambient.g;
    let mut b_acc = ambient_light.color.b * ambient_light.intensity * material.ambient.b;
    for &ls in lights {
        let to_light = *ls.position() - point;
        let dist2 = to_light.dot(&to_light).max(EPS); // avoid divide-by-zero
        let dist = dist2.sqrt();
        let light_dir = to_light / dist; // normalized
        let ndotl = n.dot(&light_dir).max(0.0);
        if ndotl <= 0.0 { continue; }
        // Diffuse component (per channel)
        let diff_r = material.diffuse.r * ndotl;
        let diff_g = material.diffuse.g * ndotl;
        let diff_b = material.diffuse.b * ndotl;

        // Specular component
        let reflect = (n * (2.0 * n.dot(&light_dir)) - light_dir).normalized();
        let spec_angle = reflect.dot(&v).max(0.0);
        let spec_factor = spec_angle.powf(material.shininess.max(0.0));
        let spec_r = material.specular.r * spec_factor;
        let spec_g = material.specular.g * spec_factor;
        let spec_b = material.specular.b * spec_factor;

        // Gentler attenuation: 1 / (1 + L*d + Q*d^2)
        let attenuation = 1.0 / (1.0 + ATTEN_LINEAR * dist + ATTEN_QUAD * dist2);

        let intensity = ls.intensity(); // user-controlled brightness scalar

        r_acc += (diff_r + spec_r) * ls.color().r * attenuation * intensity;
        g_acc += (diff_g + spec_g) * ls.color().g * attenuation * intensity;
        b_acc += (diff_b + spec_b) * ls.color().b * attenuation * intensity;
    }

    // Clamp final accumulated color to [0,1]
    Color::new(r_acc.clamp(0.0, 1.0), g_acc.clamp(0.0, 1.0), b_acc.clamp(0.0, 1.0), 1.0)
}
