fn main() {
    // CONSTANTS
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is {}", MAX_POINTS);

    // MUTABLE VARIABLES (VARIABLES ARE INMUTABLES BY DEFAULT)
    let mut x: i32 = 5;
    println!("The value of x is -> {}", x);
    x = 6;
    println!("The value of x is -> {}", x);

    // SHADOWING VARIABLES
    let spaces: &str = "    ";
    println!("The value of spaces is -> {}", spaces);
    let spaces: usize = spaces.len();
    println!("The value of spaces is -> {}", spaces);

    print_name_and_lastname("Keneth", "Riera");

    let is_true: bool = true;
    if is_true {
        println!("f is true");
    }

    let float_number: f64 = 10.0;
    println!("{}", float_number / 2.0);

    // CHAR USES SINGLE QUOTES
    let hot_emoji: char = '🥵';
    println!("{}", hot_emoji);

    // COMPOUND TYPES
    // TUPLE (ANY TYPES)
    let tuple: (i32, f64, char) = (30, 10.50, '🤔');
    // DESTRUCTURE
    let (x, y, z): (i32, f64, char) = tuple;
    println!("x: {}, y: {}, z: {}", x, y, z);

    let emotions = ('😔', '😁', '😠');
    println!("Sad -> {}", emotions.0);
    println!("Happy -> {}", emotions.1);
    println!("Angry -> {}", emotions.2);

    // ARRAY (SAME TYPES)
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    for month in months {
        println!("{}", month);
    }

    println!("My birthday is {} 2nd", months[11]);
}

fn print_name_and_lastname(name: &str, lastname: &str) {
    println!("{} {}", name, lastname);
}
