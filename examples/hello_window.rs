use vulpid_tech::Window;
use winit::event_loop::EventLoop;
use winit::event::{Event, WindowEvent};

fn main() 
{
    let evntlp = EventLoop::new().unwrap();

    println!("hello world!");
    let twin = vulpid_tech::Window::new()
        .title("From the other side")
        .width(600)
        .height(400)
        .launch(&evntlp);
    println!("{}", twin.to_string());

    let _ = evntlp.run(move |event, _elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                std::process::exit(0);
            }
            _ => {}
        }
    });
}
