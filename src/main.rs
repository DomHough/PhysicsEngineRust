mod vec3;
mod sphere;
mod camera;
mod ray;
mod phong;
mod light;
mod color;

use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId, WindowAttributes};
use sphere::Sphere;
use vec3::Vec3;
use crate::camera::Camera;
use crate::color::Color;
use crate::light::PointLight;
use image; // for saving the buffer

#[derive(Default)]
struct App {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
    image: Option<Vec<u8>>, // RGBA buffer from camera
    dims: (u32, u32)
}

impl ApplicationHandler for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        // Desired resolution
        let (w, h) = (500u32, 500u32);

        // Create + leak the window (simplify lifetime) sized to camera resolution
        let winit_window = el.create_window(
            WindowAttributes::default().with_inner_size(winit::dpi::LogicalSize::new(w, h))
        ).expect("create window");
        let winit_window: &'static Window = Box::leak(Box::new(winit_window));
        self.window = Some(winit_window);

        let st = SurfaceTexture::new(w, h, winit_window);
        let px = Pixels::new(w, h, st).expect("create pixels");
        self.pixels = Some(px);

        // Build scene and render once to a buffer
        let sphere = Sphere::new(2.0, Vec3::new(0.0, 0.0, 5.0));
        let light = PointLight::new(Vec3::new(5.0, 5.0, 0.0), Color::new(1.0, 1.0, 1.0, 0.1));
        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            60.0_f32,
            (w, h)
        );
        self.image = Some(camera.render_sphere(&sphere, &light));
        self.dims = (w, h);
        // Save the image to disk
        if let Some(buf) = &self.image {
            let _ = image::save_buffer(
                "render.png",
                buf,
                w,
                h,
                image::ColorType::Rgba8
            );
        }

        winit_window.request_redraw();
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::Resized(size) => {
                if let Some(p) = self.pixels.as_mut() {
                    let _ = p.resize_surface(size.width.max(1), size.height.max(1));
                }
            }
            WindowEvent::ScaleFactorChanged { mut inner_size_writer, .. } => {
                if let Some(w) = self.window {
                    let size = w.inner_size();
                    let _ = inner_size_writer.request_inner_size(size);
                    if let Some(p) = self.pixels.as_mut() {
                        let _ = p.resize_surface(size.width.max(1), size.height.max(1));
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let (Some(pixels), Some(img)) = (&mut self.pixels, &self.image) {
                    let frame = pixels.frame_mut();
                    if frame.len() == img.len() {
                        frame.copy_from_slice(img);
                    }
                    pixels.render().expect("render");
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("event loop");
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::default();
    event_loop.run_app(&mut app).expect("run");
}
