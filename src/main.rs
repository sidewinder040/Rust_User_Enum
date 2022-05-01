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
    
    // Vector
    let mut users: Vec<User> = Vec::new();
    users.push(my_user);
    
    match users.get(0) {
        Some(vec_user) => {
            println!("Name: {}",vec_user.username);
            println!("Age: {}", vec_user.age);
        }
        None => println!("Error: Missing Element!!"),
    }
}
