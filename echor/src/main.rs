use clap::App;
fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Dmitry <dmitryi@proton.me>")
        .about("Rust echo")
        .get_matches();
    println!("Hello, world!");
}
