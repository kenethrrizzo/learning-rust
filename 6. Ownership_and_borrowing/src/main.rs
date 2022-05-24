/*
*   Ownership: Propiedad, que es dueño de.
*   Borrowing: Pedir prestado.
*/

/*
*   Rust no tiene Garbage collector.
*   Cada data tiene UN propietario.
*/

fn main() {
    /*  
    *   STACK:
    *   implementado como stack (pila, estructura de dato)
    *   tamaño fijo
    *   rápida, basta con mover el puntero
    *   es liberada cuando se alcanza el fin del scope
    */
    /*  
    *   HEAP:
    *   flexible
    *   costoso de asignar y recuperar datos
    *   es liberada cuando no tiene dueño
    */

    let mut age: u32 = 21;
    add_age(&mut age);  // para los tipos de datos "primitivos" se pasa una copia por defecto, tambien podemos pasarle una variable por referencia
    println!("Age: {}", age);

    let mut name: String = String::from("Keneth");
    concat_names(&mut name); // pasar por referencia
    println!("Name: {}", name); 

    let name2: String = String::from("Camila");
    print_names(name2.clone()); // más costoso, se le pasa una copia de name2, por lo que name2 sigue teniendo su valor
    println!("Name: {}", name2);
}

fn add_age(age: &mut u32) {
    *age += 1;
}

fn concat_names(name: &mut String) {
    *name = format!("{} Riera", *name); // dereferenciar, obtener el valor del espacio de memoria
    println!("{}", *name)
}

fn print_names(mut name: String) {
    name = format!("{name} Lucas");
    println!("{}", name);
}