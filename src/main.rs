// Get used to how Options

fn is_it_null(value: Option<i32>) -> bool {
    match value {
        Some(_x) => false,
        None => true,
    }
}

fn main() {
    let ten: Option<i32> = Some(20);
    let null: Option<i32> = None;

    println!("Is this value null? {}", is_it_null(ten));
    println!("Is this value null? {}", is_it_null(null));
}
