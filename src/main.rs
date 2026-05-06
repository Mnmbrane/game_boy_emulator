mod core;
mod frontend;

use crate::core::GameBoy;
use crate::frontend::Frontend;

fn main() {
    println!("Hello, world!");
    let game_boy = GameBoy::new();
    let frontend = Frontend::new();
}
