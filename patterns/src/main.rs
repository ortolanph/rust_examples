fn main() {
    let name = String::from("Ortolan");
    let position = 8;

    println!(
        "Character at position {} is {}",
        position,
        match name.chars().nth(position) {
            Some(c) => c.to_string(),
            None => "NO CHARACTER FOUND".to_string()
        }
    );
}
