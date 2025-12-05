use vulpid_tech::render::Engine;
use vulpid_tech::test;

fn main() {
    let engine = Engine::new();
    println!("Welcome to {}!", engine.name());
    println!("Engine initialized successfully.");

    test::PRINT();
}
