fn main() {
    println!("Hello, world!");
    another_function(25, 4);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {}", y);

    let result: i32 = add_two_numbers(3, 5);
    println!("The result is {}", result);

    if_in_a_let_statement(10);
    if_in_a_let_statement(5);
    loop_with_returning();
    looping_in_reverse();

    let mut s: String = String::from("Hola, ");
    s.push_str("Mundo.");

    println!("{s}");

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let roman_number = num_as_roman(8237);
    println!("{}", roman_number);

    println!("{}", alphabet_position("The narwhal bacons at midnight."));
    println!(
        "{}",
        alphabet_position("The sunset sets at twelve o' clock.")
    );
    let reversed_word: String = reverse_word("Keneth");
    println!("{}", reversed_word);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn add_two_numbers(x: i32, y: i32) -> i32 {
    x + y
}

fn if_in_a_let_statement(x: u8) {
    let description: &str = if x % 2 == 0 { "par" } else { "impar" };
    println!("The number {} is {}", x, description);
}

fn loop_with_returning() {
    let mut counter: u32 = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

fn looping_in_reverse() {
    for i in (1..10).rev() {
        println!("{}", i);
    }
}

fn reverse_word(word: &str) -> String {
    word.chars()
        .rev()
        .map(|w| w.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn num_as_roman(mut num: i32) -> String {
    let mut letters = String::new();
    let symbols = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    for &(n, symbol) in symbols.iter() {
        while num >= n {
            letters.push_str(symbol);
            num -= n;
        }
    }
    letters
}

fn alphabet_position(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c >= &'a' && c <= &'z')
        .map(|c| (c as u32 - 96).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
