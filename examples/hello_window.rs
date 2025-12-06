use vulpid_tech::window::Window;
use winit::application::ApplicationHandler;
use winit::event::{WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use std::sync::Arc;

struct App {
    window: Option<Arc<Window>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            // Create a raw window first
            let attributes = winit::window::Window::default_attributes()
                .with_title("VulpID-tech")
                .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));
            
            let raw_window = event_loop.create_window(attributes).expect("Failed to create window");
            let window = Arc::new(Window::new_from_raw(raw_window));
            self.window = Some(window);
            
            println!("Window created");
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.window = None;
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() {
    println!("VulpID-tech Engine");

    let event_loop = EventLoop::new().unwrap();
    let mut app = App { window: None };

    let _ = event_loop.run_app(&mut app);
}
