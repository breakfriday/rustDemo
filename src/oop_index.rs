mod oop1;

use oop1::User;

fn main() {
    let mut user = User::new("Alice", 30);
    user.display();
    user.update_name("Bob");
    user.update_age(35);
    user.display();
}
