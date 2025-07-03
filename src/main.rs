// this step is what allows stuff to be visible
mod audio;
mod endoder;

// this is where we specify what we actually wanna use
use audio::whoofer::something;

fn main() {
    let message: &str = "The name is Nnaemeka Onyeokoro";
    something();
    println!("{message}")
}
