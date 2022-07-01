struct User {
    username: String,
    age: u16,
}

impl User {
    // TODO: Add a print User method
    fn print_user(&self) {
        println!("-------------------");
        println!("Name: {}", self.username);
        println!("Age: {}", self.age);
    }
}

fn main() {
    let my_user = User {
        username: String::from("Mark Crouch"),
        age: 57,
    };
    
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
        User::print_user(&user);
    }
}
