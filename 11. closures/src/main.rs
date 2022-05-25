fn main() {
    // Closures: función que es definida en linea (inline)
    let sum = |num1: i32, num2: i32| -> i32 {
        num1 + num2
    };
    println!("{}", sum(3, 7));

    let mut counter: i32 = 1;

    let mut add = move || {
        counter += 1;
        println!("{}", counter);
    };

    let variable = &counter; //borrowing
    add();
    println!("{}", counter);
    println!("{}", variable);
}