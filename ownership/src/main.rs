fn main() {
    let greet_message = String::from("Hello, Rust!");
    print_string(greet_message);

    let mut user_name = String::from("Fatih");
    let greet_message = create_greet_message(&mut user_name);
    print_string(greet_message);

    let mut some_string = String::from("Lorem Ipsum");
    {
        let string_reference_1 = &mut some_string;
        string_reference_1.insert_str(0, "Reformated ");
        println!("{}", string_reference_1);
        println!("{}", some_string);
    }
    let string_reference_2 = &mut some_string;
    string_reference_2.insert_str(11, "2 ");
    println!("{}", string_reference_2);
    println!("{}", some_string);
}

fn create_greet_message(user_name: &mut String) -> String {
    let greet_message = format!("Hello, {}!", user_name);
    greet_message
}

fn print_string(str: String) {
    println!("{}", str);
}
