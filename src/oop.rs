struct User {
    name: String,
    age: u32,
}

const DEFAULT_NAME: &str = "kitty";

const DEFAULT_AGE: u32 = 30;

impl User {
    fn init(user_name: String, user_age: u32) -> Self {
        Self {
            name: user_name,
            age: user_age,
        }
    }

    fn say_hello(&self) {
        println!(
            "hello, my name is {} and I am {} years old",
            &self.name, self.age
        );
    }

    fn update_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    fn update_name(&mut self, new_name: impl Into<String>) {
        self.name = new_name.into();
    }

    fn init_new(use_name: impl Into<String>, age: u32) -> Self {
        Self {
            // 无论你传进来的是 &str 还是 String，
            // 调用 .into() 之后，它都会统一变成 String。
            name: use_name.into(),
            age,
        }
    }
}

fn main() {
    let mut user = User::init(String::from(DEFAULT_NAME), DEFAULT_AGE);
    user.say_hello();
    user.update_age(100);
    user.update_name("tinny");
    user.say_hello();

    println!(
        "default_name is {}  ,default_age is {}",
        DEFAULT_NAME, DEFAULT_AGE
    )
}
