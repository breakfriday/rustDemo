use std::fmt;

const DEFAULT_NAME: &str = "kitty";
const DEFAULT_AGE: u32 = 30;
const MAX_AGE: u32 = 150;

#[derive(Debug, Clone, PartialEq, Eq)]
struct User {
    name: String,
    age: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Admin {
    user: User,
    permissions: Vec<String>,
}

trait Greeter {
    fn greeting(&self) -> String;
}

impl User {
    fn init_new(user_name: impl Into<String>, user_age: u32) -> Result<Self, String> {
        let user = Self {
            name: user_name.into(),
            age: user_age,
        };
        user.validate()?;
        Ok(user)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> u32 {
        self.age
    }

    fn update_name(&mut self, new_name: impl Into<String>) {
        self.name = new_name.into();
    }

    fn update_age(&mut self, new_age: u32) -> Result<(), String> {
        if new_age > MAX_AGE {
            return Err(format!("age must be less than or equal to {MAX_AGE}"));
        }

        self.age = new_age;
        Ok(())
    }

    fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("name cannot be empty".into());
        }

        if self.age > MAX_AGE {
            return Err(format!("age must be less than or equal to {MAX_AGE}"));
        }

        Ok(())
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: DEFAULT_NAME.into(),
            age: DEFAULT_AGE,
        }
    }
}

impl Greeter for User {
    fn greeting(&self) -> String {
        format!(
            "hello, my name is {} and I am {} years old",
            self.name, self.age
        )
    }
}

impl fmt::Display for User {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Name: {}, Age: {}", self.name, self.age)
    }
}

impl Admin {
    fn new(user: User, permissions: Vec<String>) -> Self {
        Self { user, permissions }
    }

    fn add_permission(&mut self, permission: impl Into<String>) {
        self.permissions.push(permission.into());
    }

    fn permissions(&self) -> &[String] {
        &self.permissions
    }
}

impl Greeter for Admin {
    fn greeting(&self) -> String {
        format!("admin {} is ready", self.user.name())
    }
}

fn greet(item: &impl Greeter) {
    println!("{}", item.greeting());
}

fn main() -> Result<(), String> {
    let mut user = User::init_new(DEFAULT_NAME, DEFAULT_AGE)?;

    println!("{user}");
    greet(&user);

    user.update_age(31)?;
    user.update_name("tinny");

    println!("updated user: {user}");
    println!("getter result: name={}, age={}", user.name(), user.age());

    let mut admin = Admin::new(user.clone(), vec!["read".into(), "write".into()]);
    admin.add_permission("delete");

    greet(&admin);
    println!("admin permissions: {:?}", admin.permissions());

    let greeters: Vec<Box<dyn Greeter>> = vec![Box::new(user), Box::new(admin)];
    for greeter in greeters {
        println!("dynamic dispatch: {}", greeter.greeting());
    }

    let default_user = User::default();
    println!("default user: {default_user}");

    Ok(())
}
