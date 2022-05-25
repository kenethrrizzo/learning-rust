use std::fmt::{Debug, Display};

fn main() {
    let human: Human = Human { name: String::from("Keneth"), age: 21 };
    println!("{}", human.say_hello());
    println!("{:?}", human);

    let cat: Cat = Cat { name: "Pastelito".to_string() };
    println!("{}", cat.say_hello());
    println!("{}", cat);

    println!("Humans speak {}, cats speak {}", Human::language(), Cat::language());

    let age: Option<i32> = Some(15);
    println!("Is of legal age? {}", age.is_of_legal_age());
}

#[derive(Debug)]
struct Human {
    name: String,
    age: u32
}
struct Cat {
    name: String
}

impl Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Name: {}", self.name)
    }
}

trait Speak {
    fn say_hello(&self) -> String;

    fn language() -> String {
        String::from("No language")
    }
}

impl Speak for Human {
    fn say_hello(&self) -> String {
        String::from(format!("Hello, world. I'm {} and I am {} years old.", self.name, self.age))
    }

    fn language() -> String {
        String::from("Spanish")
    }
}

impl Speak for Cat {
    fn say_hello(&self) -> String {
        String::from("Miau!")
    }
}

trait DriversLicence {
    fn is_of_legal_age(&self) -> bool;
}

impl DriversLicence for Option<i32> {
    fn is_of_legal_age(&self) -> bool {
        match self {
            Some(age) => *age >= 18,
            None => false,
        }
    }
}