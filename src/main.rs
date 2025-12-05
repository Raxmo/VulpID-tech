use vulpid_tech::render::Engine;

fn main() {
    let engine = Engine::new();
    println!("Welcome to {}!", engine.name());
    println!("Engine initialized successfully.");
}
