fn main() {
    println!("Hello, world!");
    another_function();
    another_function2(5);

    let x = 5;
    let y = 3;
    let res = sum(x, y);

    println!("The sum of {} and {} is {}", x, y, res);
}

fn another_function() {
    println!("Another function");
}

fn another_function2(x:i32) {
    println!("Another function 2, x={}", x);
}

fn sum(x:i32, y:i32) -> i32 {
    let b = x + y;

    b * 100
}