// a.rs
//
// 这是一个 Rust 入门学习文件，重点不是做一个完整项目，而是集中展示 Rust
// 相比其他常见语言更特殊、也更重要的语言设计：
//
// 1. 所有权 ownership
// 2. 借用 borrowing
// 3. 可变性 mutability
// 4. 生命周期 lifetime
// 5. Option / Result 错误处理
// 6. match 模式匹配
// 7. struct / enum / trait
// 8. 泛型 generic
// 9. 闭包 closure
// 10. 模块 module
//
// 运行方式：
//
//   rustc a.rs
//   ./a
//
// 或者把它声明成 Cargo 的 bin target，然后运行：
//
//   cargo run --bin a

use std::fmt::Display;

fn main() {
    println!("== Rust language tour ==");

    variables_and_mutability();
    ownership_move_and_clone();
    borrowing_references();
    slices_example();
    structs_and_methods();
    enums_and_match();
    option_example();
    result_example();
    traits_and_generics();
    lifetimes_example();
    closures_and_iterators();
    module_example::run();
}

fn variables_and_mutability() {
    println!("\n-- 1. variables and mutability --");

    let x = 10;
    println!("x = {x}");

    let mut count = 0;
    count += 1;
    count += 1;
    println!("count = {count}");

    let value = "42";
    let value: i32 = value.parse().expect("not a number");
    println!("parsed value = {value}");
}

fn ownership_move_and_clone() {
    println!("\n-- 2. ownership, move and clone --");

    let name = String::from("Rust");
    let moved_name = name;
    println!("moved_name = {moved_name}");

    let a = String::from("hello");
    let b = a.clone();
    println!("a = {a}, b = {b}");

    let n1 = 100;
    let n2 = n1;
    println!("n1 = {n1}, n2 = {n2}");
}

fn borrowing_references() {
    println!("\n-- 3. borrowing and references --");

    let text = String::from("borrow me");
    print_length(&text);
    println!("text is still usable: {text}");

    let mut title = String::from("Rust");
    append_language(&mut title);
    println!("title = {title}");
}

fn print_length(s: &String) {
    println!("length = {}", s.len());
}

fn append_language(s: &mut String) {
    s.push_str(" language");
}

fn slices_example() {
    println!("\n-- 4. slices --");

    let message = String::from("hello rust world");
    let first = first_word(&message);
    println!("first word = {first}");

    let numbers = [10, 20, 30, 40, 50];
    let middle = &numbers[1..4];
    println!("middle = {:?}", middle);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }

    s
}

fn structs_and_methods() {
    println!("\n-- 5. structs and methods --");

    let mut user = User {
        name: String::from("Alice"),
        age: 20,
        active: true,
    };

    user.say_hello();
    user.birthday();
    user.say_hello();

    let older_user = User { age: 30, ..user };
    older_user.say_hello();
}

struct User {
    name: String,
    age: u32,
    active: bool,
}

impl User {
    fn say_hello(&self) {
        println!(
            "hello, I am {}, age {}, active = {}",
            self.name, self.age, self.active
        );
    }

    fn birthday(&mut self) {
        self.age += 1;
    }
}

fn enums_and_match() {
    println!("\n-- 6. enums and match --");

    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 0, 0),
    ];

    for message in messages {
        handle_message(message);
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn handle_message(message: Message) {
    match message {
        Message::Quit => println!("quit"),
        Message::Move { x, y } => println!("move to ({x}, {y})"),
        Message::Write(text) => println!("write: {text}"),
        Message::ChangeColor(r, g, b) => println!("color = rgb({r}, {g}, {b})"),
    }
}

fn option_example() {
    println!("\n-- 7. Option --");

    let names = vec!["Alice", "Bob", "Carol"];

    let first = names.get(0);
    let missing = names.get(99);

    print_optional_name(first);
    print_optional_name(missing);

    if let Some(name) = names.get(1) {
        println!("second name = {name}");
    }
}

fn print_optional_name(name: Option<&&str>) {
    match name {
        Some(value) => println!("found name: {value}"),
        None => println!("name not found"),
    }
}

fn result_example() {
    println!("\n-- 8. Result --");

    match divide(10.0, 2.0) {
        Ok(value) => println!("10 / 2 = {value}"),
        Err(error) => println!("error: {error}"),
    }

    match divide(10.0, 0.0) {
        Ok(value) => println!("10 / 0 = {value}"),
        Err(error) => println!("error: {error}"),
    }

    match parse_and_double("21") {
        Ok(value) => println!("double = {value}"),
        Err(error) => println!("parse error: {error}"),
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn parse_and_double(input: &str) -> Result<i32, std::num::ParseIntError> {
    let number = input.parse::<i32>()?;
    Ok(number * 2)
}

fn traits_and_generics() {
    println!("\n-- 9. traits and generics --");

    let article = Article {
        title: String::from("Rust ownership"),
        author: String::from("Alice"),
    };

    let video = Video {
        title: String::from("wgpu triangle"),
        duration_seconds: 120,
    };

    notify(&article);
    notify(&video);

    print_twice(123);
    print_twice("hello");
}

trait Summary {
    fn summary(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

struct Video {
    title: String,
    duration_seconds: u32,
}

impl Summary for Article {
    fn summary(&self) -> String {
        format!("article '{}' by {}", self.title, self.author)
    }
}

impl Summary for Video {
    fn summary(&self) -> String {
        format!("video '{}' duration {}s", self.title, self.duration_seconds)
    }
}

fn notify(item: &impl Summary) {
    println!("notify: {}", item.summary());
}

fn print_twice<T: Display>(value: T) {
    println!("{value}, {value}");
}

fn lifetimes_example() {
    println!("\n-- 10. lifetimes --");

    let a = String::from("short");
    let b = String::from("a much longer string");

    let result = longest(a.as_str(), b.as_str());
    println!("longest = {result}");
}

fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

fn closures_and_iterators() {
    println!("\n-- 11. closures and iterators --");

    let numbers = vec![1, 2, 3, 4, 5, 6];

    let doubled: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
    println!("doubled = {:?}", doubled);

    let even_sum: i32 = numbers.iter().filter(|n| *n % 2 == 0).sum();
    println!("even sum = {even_sum}");

    let prefix = String::from("item");
    let make_label = move |id: i32| format!("{prefix}-{id}");

    println!("{}", make_label(7));
}

mod module_example {
    pub fn run() {
        println!("\n-- 12. modules --");

        let config = Config::new(String::from("debug"), 8080);
        config.print();

        helper();
    }

    pub struct Config {
        mode: String,
        port: u16,
    }

    impl Config {
        pub fn new(mode: String, port: u16) -> Self {
            Self { mode, port }
        }

        pub fn print(&self) {
            println!("config: mode = {}, port = {}", self.mode, self.port);
        }
    }

    fn helper() {
        println!("private helper called");
    }
}
