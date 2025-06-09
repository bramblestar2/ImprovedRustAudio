mod api;
mod engine;
mod audio;
mod utils;
mod manager;

fn main() {

    let file = "/home/jay/Downloads/C418 - The Fighter.mp3";

    let mut engine = engine::Engine::new();

    let builder = engine.builder().set_file(file).set_fade_in(5.0).set_fade_out(10.0).set_end(20.0).set_start(10.0);

    let id = engine.create(builder);
    engine.manager().play(id);

    for (i, audio) in engine.list().iter() {
        println!("{}: {:?}", i, audio);
    }

    std::io::stdin().read_line(&mut String::new()).unwrap();

    engine.remove(id);

    for (i, audio) in engine.list().iter() {
        println!("{}: {:?}", i, audio);
    }
}
