

// The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test
// not when you run `cargo build`
#[cfg(test)] 
mod tests {
    use std::fs;
    use toml;
    use ace4r::Data;
    use ace4r::Auth;

    // Test if toml file was read correctly
    #[test]
    fn test_toml(){
        println!("Hello, world! Time to automate edit of Reddit!");

        let redirect_port = 6666;
        let USER_AGENT: String;
        let CLIENT_ID: String;
        let CLIENT_SECRET: String;
        let USERNAME: String;
        let PASSWORD: String;
    
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
        
        // Testing
        let USER_AGENT_T = "windows:roux:v0.1.0 /u/your_user_name";
        let CLIENT_ID_T = "client id here";
        let CLIENT_SECRET_T = "client secret here";
        let USERNAME_T = "your-username";
        let PASSWORD_T = "your-password";
        // Read from file
        USER_AGENT = data.auth.user_agent.clone();
        CLIENT_ID = data.auth.client_id.clone();
        CLIENT_SECRET = data.auth.client_secret.clone();
        USERNAME = data.auth.username.clone();
        PASSWORD = data.auth.password.clone();
    
        println!("[OK] {}", data.auth.user_agent);
        assert_eq!( USER_AGENT_T ,USER_AGENT);
        assert_eq!( CLIENT_ID_T , CLIENT_ID);
        assert_eq!( CLIENT_SECRET_T ,CLIENT_SECRET);
        assert_eq!( USERNAME_T, USERNAME);
        assert_eq!( PASSWORD_T, PASSWORD);
    }
}