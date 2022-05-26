fn main() {
    /* 
    *   Collections:
    *     - Vector
    *     - Strings
    *     - Hashmap
    */

    // Vector
    use_of_vectors();
    
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