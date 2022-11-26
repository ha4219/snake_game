fn main() {
    let message = "Hello World!";
    let message = print_welcome(message);
    print_welcome(message);
}

fn print_welcome(text: &str) -> &str {
    println!("{}", text);
    return "Hi There";
}
