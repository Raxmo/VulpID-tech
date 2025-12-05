use vulpid_tech::window::Window;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;

fn main() {
    println!("VulpID-tech Engine");

    let event_loop = EventLoop::new().unwrap();
    let window = Window::new(&event_loop).expect("Failed to create window");

    println!("Window created");

    event_loop
        .run(|event, _elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                std::process::exit(0);
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        })
        .unwrap();
}
