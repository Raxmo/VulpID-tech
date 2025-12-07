use winit::window::Window as WinitWindow;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;

pub struct Window
{
    width: u64,
    height: u64,
    title: String,
    winit_window: Option<Box<WinitWindow>>
}

impl Window
{
    pub fn new() -> Self
    {
        Window
        {
            width: 0,
            height: 0,
            title: String::new(),
            winit_window: None
        }
    }

    pub fn width(mut self, value: u64) -> Self
    {
        self.width = value;
        return self;
    }

    pub fn height(mut self, value: u64) -> Self
    {
        self.height = value;
        return self;
    }

    pub fn title(mut self, value: &str) -> Self
    {
        self.title = value.to_string();
        return self;
    }

    pub fn to_string(&self) -> String
    {
        return format!("Window {{ width: {}, height: {}, title: {} }}", 
            self.width, 
            self.height, 
            self.title);
    }

    pub fn launch(mut self, event_loop: &EventLoop<()>) -> Self
    {
        let attributes = WinitWindow::default_attributes()
            .with_title(&self.title)
            .with_inner_size(LogicalSize::new(self.width as f64, self.height as f64));
        
        match event_loop.create_window(attributes) {
            Ok(window) => {
                self.winit_window = Some(Box::new(window));
            },
            Err(_) => {
                println!("Failed to create window");
            }
        }
        
        return self;
    }
}