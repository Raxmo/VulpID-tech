use vulpid_tech::Window;

fn main() 
{
    println!("hello world!");
    let twin = vulpid_tech::Window::new().title("From the other side");
    println!("{}", twin.to_string());
}
