pub struct User {
    pub name: String,
    pub age: u32,
}

impl User {
    pub fn new(name: impl Into<String>, age: u32) -> Self {
        Self {
            name: name.into(),
            age,
        }
    }

    pub fn display(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }

    pub fn update_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn update_age(&mut self, age: u32) {
        self.age = age;
    }
}
