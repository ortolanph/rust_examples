fn main() {
    let a = 15;

    if a < 10 {
        println!("Condition is true")
    } else {
        println!("Condition is false")
    }

    if a % 4 == 0 {
        println!("A is disible by 4");
    } else if a % 3 == 0 {
        println!("A is disible by 3");
    } else if a % 2 == 0 {
        println!("A is disible by 2");
    } else {
        println!("A is not disible by 4, 3 or 2");
    }
}
