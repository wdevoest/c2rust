mod lookup_user {
    use std::fs;
    use std::io::{self, BufRead, BufReader};
    use std::path::Path;

    // Define the `User` struct to represent user data.
    #[derive(Debug)]
    pub struct User {
        pub username: String,
        pub password: String,
        pub bankinfo: String,
    }

    // Implement the `lookup_user` function to load user data from a file.
    pub fn lookup_user(username: &str) -> Option<User> {
        // Open the file for reading.
        let file = match fs::File::open(Path::new("users.txt")) {
            Ok(file) => file,
            Err(_) => return None,
        };

        // Create a buffered reader for efficient line-by-line reading.
        let reader = BufReader::new(file);

        // Iterate over the lines of the file.
        for line in reader.lines() {
            // Read a single line of the file.
            let line = match line {
                Ok(line) => line,
                Err(_) => continue,
            };

            // Split the line into a vector of strings by commas.
            let parts: Vec<&str> = line.split(",").collect();

            // Check if the first part of the line matches the given username.
            if parts.len() >= 3 && parts[0] == username {
                // Create a new `User` struct with the data from the line.
                let user = User {
                    username: parts[0].to_string(),
                    password: parts[1].to_string(),
                    bankinfo: parts[2].to_string(),
                };
                return Some(user);
            }
        }

        // If no matching user is found, return `None`.
        None
    }
}

fn main() {
    // Get the username from the user.
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();

    // Lookup the user.
    let user = lookup_user::lookup_user(&username);

    // Check if the user was found.
    if user.is_none() {
        println!("User not found");
        return;
    }

    let user = user.unwrap();

    // Debugging information.

    // Print the username, password, and bank info of the user.
    println!("Username: {}", user.username);
    println!("Password: {}", user.password);
    println!("Bank info: {}", user.bankinfo);
}
