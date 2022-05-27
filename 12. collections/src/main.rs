fn main() {
    /* 
    *   Collections:
    *     - Vector
    *     - Strings
    *     - Hashmap
    */

    // Vector
    use_of_vectors();

    // Strings
    use_of_strings();
    
}

fn use_of_vectors() {
    let mut vector: Vec<i32> = Vec::new();
    for i in 1..20 {
        vector.push(i);
    }
    println!("{:?}", vector);

    match vector.get(30) {
        Some(value) => println!("Value exists -> {}", value),
        None => println!("Value doesn't exists"),
    };

    for v in &vector {
        println!("Value: {}", v);
    }

    for v in &mut vector {
        *v += 10;
    }
    println!("{:?}", vector);

    let vector = vec![1, 2, 3]; // macro para inicializar vectores
    println!("{:?}", vector);
}

fn use_of_strings() {
    let mut name_string: String = String::from("Kenet"); // se guarda en el heap
    name_string.push('h');
    println!("Name String: {}", name_string);

    let name_str: &str = "Camil"; // inmutable  // se guarda en stack, referencia al heap
    println!("Name &str: {}", name_str);

    //convert &str to String
    let mut name_str_converted: String = String::from(name_str);
    name_str_converted.push('a');
    println!("Name converted to String: {}", name_str_converted);

    //convert String to &str
    let name_string_converted: &str = &name_string[..];
    println!("Name converted to &str: {}", name_string_converted);
}