fn main() {
    let name = String::from("Merve");
    greet(name);

    let number = 42;
    print_number(number);

    let message = create_message();
    println!("Message: {}", message);

}

fn greet(name: String) {
    println!("Hello, {}", name);

}

fn print_number(number: i32) {
    println!("Number: {}", number);

}

fn create_message() -> String {
    let message = String::from("Hello, World!");
    message
}
