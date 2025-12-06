pub fn testing()
{
    println!("Testing from window module");
}

pub struct Window
{
    width: u64,
    height: u64,
    title: String
}
impl Window
{
    pub fn new() -> Self
    {
        return Window {
            width: 0,
            height: 0,
            title: String::new()
        };
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
        return format!("Window {{ width: {}, height: {}, title: {} }}", self.width, self.height, self.title);
    }
}
