fn main() {
    let nums = [1, 2, 3];
    for num in nums.iter() {
        println!("{}", num + 1);
    }
    println!("{:?}", nums);

    let mut counter: Counter = Counter::new();
    counter.next();
    counter.next();
    match counter.next() {
        Some(count) => println!("{}", count),
        None => println!("End of counter")
    };
    match counter.next() {
        Some(count) => println!("{}", count),
        None => println!("End of counter")
    };
}

struct Counter {
    count: i32
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= 3 {
            return None;
        }
        self.count += 1;
        return Some(self.count);
    }
}