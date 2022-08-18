fn main() {
    let tup: (i32, f64, u8) = (500, 3.5, 1);

    let (x, y, z) = tup;

    let a = tup.0;
    let b = tup.1;
    let c = tup.2;

    println!("x = {}, y = {} and z = {}", x, y, z);
    println!("a = {}, b = {} and c = {}", a, b, c);
}
