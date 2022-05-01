struct User {
    username: String,
    age: u16,
}

fn main() {
    println!("Hello, world!");
    let my_user = User {
        username: String::from("Mark Crouch"),
        age: 57,
    };
    println!("My user's name is: {}",
        my_user.username);
    println!("My user's age is: {}",
        my_user.age);
}
