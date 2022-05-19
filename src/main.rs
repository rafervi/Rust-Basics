#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let name = "Alex";
    let mut age = 32;
    let amount :i64 = 68168198161984;
    age = 43;

    println!("{}", age);

    let color = "blue";
    let color = 86;
    println!("{}",color);

    let (a, b, c)=(43, 85, "red");

    let pi: f32=4.0;
    let million = 1_000_000;
    println!("{}",million);

    let is_day = true;
    let is_night = false;
    println!("{}", is_day);
    let char1 = 'A';
    println!("{}",char1);

    let emoji = '\u{1F601}';
    println!("{}",emoji);

    let cat = "Fluffy";
    let cat: &'static str = "Fluffy";
    println!("{}",cat);
    let dog = String::new();
    let mut dog = String::from("Max");
    println!("{}",dog);
    let owner = format!("Hi I'm {} the owner of {}", "Mark",dog);
    println!("{}",owner);

    println!("{}", dog.len());

    dog.push(' ');
    dog.push_str("the dog");
    println!("{}", dog);

    let new_dog = dog.replace("the", "is my");
    println!("{}", new_dog);

    const URL:&str = "google.com";

    println!("{}",URL);

    let a = 4+8;
    let b = 18/3;
    let c = 10%3;
    println!("a={}, b={}, c={}", a, b, c);
    println!("{}", a >= b);
    println!("{}",a >=b && b<=c);
    let bitwise_and = 10&3;
    let bitwise_or = 10|3;
    let bitwise_xor = 10^3;
    let bitwise_not = !10;
    let bitwise_left_shift = 10 << 1;
    let bitwise_right_shift = 10 >> 1;
    println!("{}",bitwise_and);
    println!("{}",bitwise_or);
    println!("{}",bitwise_xor);
    println!("{}",bitwise_not);
    println!("{}",bitwise_left_shift);
    println!("{}",bitwise_right_shift);
    say_hi();
    println!("General Kenobi");
    let mut name = "john";
    let greeting = say_hello(&mut name);
    println!("{}",greeting);
    say_hello(&mut name);
    println!("{}", name);

}
fn say_hello(name: &mut &str) -> String{
    let  greeting =format!("Hello {}", name);
    //return greeting;
    greeting /* if its the only result */
}

fn say_hi() {
    println!("Hello there");

}

