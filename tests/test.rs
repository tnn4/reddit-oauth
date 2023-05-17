

// The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test
// not when you run `cargo build`
#[cfg(test)]
use std::{println as info, println as warn}; // tell tests not to hide output
mod tests {
    use std::fs;
    use toml;
    use rsraw::Data;
    use rsraw::Auth;

    // Test if toml file was read correctly
    #[test]
    fn test_toml(){
        

        let redirect_port = 7778;
        let user_agent: String;
        let client_id: String;
        let client_secret: String;
        let username: String;
        let password: String;
    
        let auth_file_name = "auth-example.toml";
    
        // Read contents of auth.toml
        let contents = match fs::read_to_string(auth_file_name) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("[ERROR] reading file `{}`", auth_file_name);
                std::process::exit(1);
            }
        };
    
        // Process (deserialize) contents of auth file into Data struct
        let data: Data = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("[ERROR] unable to load data from {}", auth_file_name);
                std::process::exit(1);
            }
        };
        
        // Testing variables
        let user_agent_test = "windows:roux:v0.1.0 /u/your_user_name";
        let client_id_test = "client id here";
        let client_secret_test = "client secret here";
        let username_test = "your-username";
        let password_test = "your-password";
        // Read from file
        user_agent = data.auth.user_agent.clone();
        client_id = data.auth.client_id.clone();
        client_secret = data.auth.client_secret.clone();
        username = data.auth.username.clone();
        password = data.auth.password.clone();
    
        // Do Tests
        println!("[OK] {}", data.auth.user_agent);
        assert_eq!( user_agent_test ,user_agent);
        assert_eq!( client_id_test , client_id);
        assert_eq!( client_secret_test , client_secret_test);
        assert_eq!( username_test, username);
        assert_eq!( password_test, password);
    }
}