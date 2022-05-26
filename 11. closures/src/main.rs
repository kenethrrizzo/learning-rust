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

    let age: Option<u32> = Some(20);

    if let Some(age) = age {
        println!("Age -> {}", age);
    }

    let mut messages: Option<i32> = Some(50);

    println!("Mensajes: {}", messages.unwrap_or_default());
    // poco legible
    loop {
        match messages {
            Some(value) => {
                if value > 0 {
                    println!("Tienes {} mensajes en tu bandeja.", value);
                    messages = Some(value - 1);
                } else {
                    println!("No tienes mensajes en tu bandeja.");
                    messages = None;
                }
            },
            None => { break; }
        }
    }
    println!("Mensajes: {}", messages.unwrap_or_default());

    // más legible
    messages = Some(50);
    println!("Mensajes: {}", messages.unwrap_or_default());
    while let Some(value) = messages {
        if value > 0 {
            println!("Tienes {} mensajes en tu bandeja.", value);
            messages = Some(value - 1);
        } else {
            println!("No tienes mensajes en tu bandeja.");
            messages = None;
        }
    }
    println!("Mensajes: {}", messages.unwrap_or_default());
}