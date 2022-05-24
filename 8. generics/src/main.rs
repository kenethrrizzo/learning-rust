struct Point<T, V> {
    x: T,
    y: V
}

fn main() {
    let point_a: Point<i32, f64> = Point { x: 10, y: 5.5 };
    let point_b: Point<i32, f64> = Point { x: 43, y: 2.4 };
    print_coordinates(point_a, point_b);

    let point_c: Point<f64, i32> = Point { x: 10.4, y: 5 };
    println!("{} : {}", point_c.x, point_c.y);    
}

fn print_coordinates<T: std::fmt::Display, V: std::fmt::Display>(point_a: Point<T, V>, point_b: Point<T, V>) {
    println!("{} : {}", point_a.x, point_a.y);
    println!("{} : {}", point_b.x, point_b.y);
}