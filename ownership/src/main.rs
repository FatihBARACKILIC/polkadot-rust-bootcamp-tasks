fn main() {
    // > Exp: 1
    // Bir String değişken oluşturup onun referansını bir fonksiyona atıyoruz
    // Referans olarak yolladığımız için tekrar bu scope içerisinde aynı değişkeni kullanabiliyoruz
    let greet_message = String::from("Hello, Rust!");
    print_string(&greet_message);
    print_string(&greet_message);

    // > Exp: 2
    // Bu sefer mutable(değiştirilebilir) bir değişken oluşturup onun referansını
    // create_greet_message ye yolluyoruz. Değişkenin içeriğini değiştirip başka
    // bir fonksiyonda kullanıyoruz.
    let mut user_name = String::from("Fatih");
    create_greet_message(&mut user_name);
    print_string(&user_name);
}

fn create_greet_message(user_name: &mut String) {
    // aldığımız mutable referansı * ile dereference edip normal haline çevirip
    // istediğimiz değişikliği yapıyoruz
    *user_name = format!("Hello, {}!", user_name);
}

fn print_string(str: &String) {
    println!("{}", str);
}
