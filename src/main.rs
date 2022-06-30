struct User {
    username: String,
    age: u16,
}

impl User {
    // TODO: Add a print User method
}

fn main() {
    println!("Hello, world!");
    let my_user = User {
        username: String::from("Mark Crouch"),
        age: 57,
    };
    // println!("My user's name is: {}",
    //     my_user.username);
    // println!("My user's age is: {}",
    //     my_user.age);
    
    // Vector
    let mut users: Vec<User> = Vec::new();
    users.push(my_user);

    // Add another user (add straigth to Users vector)
    users.push(User { 
        username: (String::from("Bob Smith")), 
        age: (52) 
    });
    
    // match users.get(0) {
    //     Some(vec_user) => {
    //         println!("Name: {}",vec_user.username);
    //         println!("Age: {}", vec_user.age);
    //     }
    //     None => println!("Error: Missing Element!!"),
    // }

    for user in users {
        println!("-------------------");
        println!("Name: {}", user.username);
        println!("Age: {}", user.age);
    }
}
