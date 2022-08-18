fn main() {
    let mut s = String::from("Hello");

    s.push_str(", World!");

    println!("{}", s);

    let s1 = String::from("hello");
    let mut s2 = String::clone(&s1);

    println!("s1 {}", s1);
    println!("s2 {}", s2);

    s2.push_str(", World");

    println!("s2 {}", s2);
}