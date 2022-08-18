use std::fs::File;

fn main() {
    let f1 = File::open("hello.txt");

    let _my_file = match f1 {
        Ok(file) => file,
        Err(file_error) => {
            println!("Message [{}]", file_error.to_string());
            panic!("File was not found");
        },
    };

    //let _f2 = File::open("hello1.txt").unwrap();
    let _f3 = File::open("hello2.txt").expect("The file was not found");
}
