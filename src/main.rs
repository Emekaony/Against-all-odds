use std::fmt::format;

fn main() {
    let mut colors = ["white", "green", "blue", "purple"];
    println!("Before updating, colors is: {:?}", colors);
    update_colors(&mut colors[2..4]);
    println!("Colors is now: {:?}", colors);

    // using tuples
    let person_details: (&str, bool, i16) = ("Nnaemeka", true, 24);

    // using structs
    let mut emeka = Person {
        first_name: String::from("Nnaemeka"),
        last_name: String::from("Onyeokoro"),
        age: 23,
        employed: (true, String::from("Google")),
    };

    if emeka.employed.0 {
        println!("Emeka is employed at: {}", emeka.employed.1);
    } else {
        println!("Emeka is about to be employed at Google as a Rust engineer.");
    }

    emeka.display_details();
    println!("{:?}", emeka);

    emeka.update_first_name("kamsi");
    emeka.display_details();

    // invoke static method of the Person Struct
    Person::say_hello();
}

// for slices
fn update_colors(color_slice: &mut [&str]) {
    println!("Updating colors now...");
    color_slice[0] = "MAGENTA";
    color_slice[1] = "CYAN";
}

// structures
#[derive(Debug)]
#[allow(unused)]
struct Person {
    first_name: String,
    last_name: String,
    age: u32,
    employed: (bool, String), // if employed, then where
}

impl Person {
    fn display_details(&self) {
        let message = format!("{} works at {}", self.first_name, self.employed.1);
        println!("{}", message);
    }

    fn update_first_name(&mut self, new_name: &str) {
        self.first_name = new_name.to_string();
    }

    fn say_hello() {
        println!("I am a Person");
    }
}
