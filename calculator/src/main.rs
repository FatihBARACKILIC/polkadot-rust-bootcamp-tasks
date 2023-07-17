use std::{io::stdin, process::exit};

// İşlemlerimiz için belirtilen enum
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn main() {
    // Kullanıcıdan değer almadan önce mutable değişkenlerimizi içi empty
    //  olarak tanımlıyoruz.
    let mut first_number = String::new();
    let mut operator = String::new();
    let mut second_number = String::new();

    println!("Enter the first number: ");
    // Kullanıcıdan değer alma işlemi get_value fonksiyonu ile yapılmaktadır.
    // Kullanıcıdan değer alınır ve trim ile gereksiz boşluklar silinir.
    // Ve referans üzerinden oluşturulan değerlere alınan değerler atanır
    get_value(&mut first_number);
    // Parse işlemi parse_float fonksiyonu içerisinde yapılmaktadır.
    // her parse işleminde hata kontrolü de yapılmaktadır.
    // faklı veri tipine dönüştürülme yapıldığı için borrowing kullanılamaz
    //  bu işlemde shadowing kullandım.
    let first_number = parse_float(&first_number);

    println!("(+, -, *, /)");
    println!("Enter the operation: ");
    get_value(&mut operator);
    // Kullanıcıdan alınan string değer operator_match fonksiyonu içerisinde
    //  validation yapılıp char tipine dönüşüyor
    let operator = operator_match(operator);

    println!("Enter the second number: ");
    get_value(&mut second_number);
    let second_number = parse_float(&second_number);

    // Kullanıcıdan değerler alındı ve değerler parse edildi şimdi işlem
    //  kısmına geldik. Kullanıcının girdiği operatör'e göre enum değer
    //  operation değişkenine atanıyor.
    let operation = match operator {
        '+' => Operation::Add(first_number, second_number),
        '-' => Operation::Subtract(first_number, second_number),
        '*' => Operation::Multiply(first_number, second_number),
        '/' => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation! Please enter a valid operation.");
            // Hata olması durumunda uygulama exit ile durdurulacaktır.
            exit(400);
        }
    };

    // enum calculate fonksiyonuna gönderilip sonuç ekrana yazdırılıyor.
    let result = calculate(operation);
    println!("{first_number} {operator} {second_number} = {result}");
}

// Kullanıcıdan değer alma işlemi burada yapılmaktadır.
fn get_value(variable: &mut String) {
    stdin()
        .read_line(variable)
        .expect("Something goes wrong! Please try again.");
    // Herhangi bir hata olması durumunda expect hata mesajını ekrana yazacaktır.

    // Referans alınan değişkene atama yapmak için başına yıldız koymamız lazım.
    // variable şu an string bir değeri göstermiyor.
    //  Şu an fonksiyona atanan değişkeni gösteriyor.
    // trim ile de boşlukları(whitespace) i siliyoruz.
    *variable = variable.trim().to_string();
}

// Parse işlemi diğer dillerde tek satırlık kod ama Rust'da hata kontrolü de
//  yapmak gerekiyor. Bu yüzden parse işlemini bir fonksiyon olarak böldüm.
// parse_float fonksiyonu &str değeri f64 e dönüştürdüğünden dolayı
//  borrowing kullanamadım.
fn parse_float(variable: &str) -> f64 {
    match variable.parse() {
        Ok(v) => v,
        Err(_) => {
            println!("Invalid value! Please enter a number.");
            // Hata olması durumunda uygulama exit ile durdurulacaktır.
            exit(400);
        }
    }
}

// Main'de çok yer kapladığı için parçaladım bu kısmı
fn operator_match(operator: String) -> char {
    // kullanıcıdan alınan değer chars ile char array ine dönüştürüyor.
    // next ile de ilk değeri ele alıyoruz.
    match operator.chars().next() {
        Some(c) => {
            // alınan değer bu işlemlerden biri değil ise hata veriyor program
            if !['+', '-', '*', '/'].contains(&c) {
                println!("Invalid operation! Please enter a valid operation.");
                // Hata olması durumunda uygulama exit ile durdurulacaktır.
                exit(400);
            }
            c
        }
        None => {
            println!("Something goes wrong! Please try again.");
            // Hata olması durumunda uygulama exit ile durdurulacaktır.
            exit(500);
        }
    }
}

// işlem yapılır ve f64 olarak sonuç geri döner.
fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(n1, n2) => n1 + n2,
        Operation::Subtract(n1, n2) => n1 - n2,
        Operation::Multiply(n1, n2) => n1 * n2,
        Operation::Divide(n1, n2) => n1 / n2,
    }
}

// ? Ek Bilgi
// Rust'un kendine ait bir typeof fonksiyonu yok sanırım.
// bulduğum fonksiyonların hiç biri çalışmadı bu yüzden
// Dev.to'da böyle bir kod gördüm.
// https://dev.to/azure/rust-how-to-find-a-variable-data-type-1bdp
// fn type_of<T>(_: &T) -> &'static str {
//     std::any::type_name::<T>()
// }
