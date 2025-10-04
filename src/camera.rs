use crate::color::Color;
use crate::light::PointLight;
use crate::phong;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Camera {
    position: Vec3,
    rotation: Vec3,
    fov: f32,
    resolution: (u32, u32),
}

// +Z is forward
// Right handed coordinate system

impl Camera {
    pub fn new(position: Vec3, rotation: Vec3, fov: f32, resolution: (u32, u32)) -> Self {
        Camera { position, rotation, fov, resolution }
    }

    pub fn set_resolution(&mut self, resolution: (u32, u32)) {
        self.resolution = resolution;
    }

    pub fn render_sphere(&self, sphere: &Sphere, light: &PointLight) -> Vec<u8> {
        let width = self.resolution.0;
        let height = self.resolution.1;
        let aspect_ratio: f32 = if height > 0 { width as f32 / height as f32 } else { 1.0 };
        let mut buffer = vec![0u8; (width * height * 4) as usize];

        // Convert FOV (stored in degrees) to radians for tangent
        let fov_rad = self.fov.to_radians();

        for y in 0..height {
            for x in 0..width {
                // pixel center
                let px = x as f32 + 0.5;
                let py = y as f32 + 0.5;

                let sx = (2.0 * px / width as f32) - 1.0;      // -1 .. 1
                let sy = 1.0 - (2.0 * py / height as f32);     //  1 .. -1 (flip y -> top positive)

                let half_h = (0.5 * fov_rad).tan();
                let half_w = aspect_ratio * half_h;

                let cx = sx * half_w;
                let cy = sy * half_h;
                let cz = 1.0f32; // forward (+Z)

                let ray_origin = self.position; // camera origin
                let ray_direction = Vec3::new(cx, cy, cz).normalized(); // normalize for stable shading
                let ray = Ray::new(ray_origin, ray_direction);

                let hit = sphere.intersects(&ray);
                let idx = ((y * width + x) * 4) as usize;
                if let Some((_t, intersect, normal)) = hit {
                    // Light direction: point -> light
                    let light_direction = (light.position - intersect).normalized();
                    // View direction: point -> camera
                    let view_direction = (self.position - intersect).normalized();

                    let color = phong::phong_shade(
                        normal,
                        light_direction,
                        view_direction,
                        light.color,
                        Color::new(0.0,0.0,1.0,1.0),   // ambient material
                        Color::new(0.0,0.0,1.0,1.0),   // diffuse material
                        Color::new(1.0,1.0,1.0,1.0),   // specular material
                        32.0,                           // shininess (increased for tighter highlight)
                        Color::new(0.1,0.1,0.1,1.0),   // ambient light
                    );

                    buffer[idx] = (color.r * 255.0) as u8;      // R
                    buffer[idx + 1] = (color.g * 255.0) as u8;  // G
                    buffer[idx + 2] = (color.b * 255.0) as u8;  // B
                    buffer[idx + 3] = 255;                      // A
                } else {
                    buffer[idx] = 0;
                    buffer[idx + 1] = 0;
                    buffer[idx + 2] = 0;
                    buffer[idx + 3] = 255;
                }
            }
        }
        buffer
    }

    // Convert a point from camera (view) space into world space (translation only for now)
    pub fn camera_to_world(&self, pos: Vec3) -> Vec3 {
        pos + self.position
    }

    // Convert a point from world space into camera (view) space (inverse translation only)
    pub fn world_to_camera(&self, world: Vec3) -> Vec3 {
        world - self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_to_world_identity() {
        let cam = Camera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0,0.0,0.0), 60.0_f32, (800,600));
        let p_cam = Vec3::new(1.0, 2.0, 3.0);
        let p_world = cam.camera_to_world(p_cam);
        assert_eq!(p_world, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn camera_to_world_translation() {
        let cam = Camera::new(Vec3::new(10.0, -2.0, 5.0), Vec3::new(0.0,0.0,0.0), 45.0_f32, (320,240));
        let origin_cam = Vec3::new(0.0, 0.0, 0.0);
        let world = cam.camera_to_world(origin_cam);
        assert_eq!(world, Vec3::new(10.0, -2.0, 5.0));
    }

    #[test]
    fn world_to_camera_translation() {
        let cam = Camera::new(Vec3::new(-3.0, 4.0, -7.0), Vec3::new(0.0,0.0,0.0), 30.0_f32, (100,100));
        let world_point = Vec3::new(-3.0, 4.0, -7.0); // camera position
        let p_cam = cam.world_to_camera(world_point);
        assert_eq!(p_cam, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn round_trip_camera_world_camera() {
        let cam = Camera::new(Vec3::new(5.0, -1.0, 2.5), Vec3::new(0.0,0.0,0.0), 75.0_f32, (1920,1080));
        let p_cam = Vec3::new(1.0, -1.0, 2.0);
        let p_world = cam.camera_to_world(p_cam);
        let p_cam2 = cam.world_to_camera(p_world);
        assert_eq!(p_cam2, p_cam);
    }
}