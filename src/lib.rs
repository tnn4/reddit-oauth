use serde_derive::Deserialize;


#[derive(Deserialize)]
pub struct Data {
    pub auth: Auth,
}

/// user_agent:
/// client_id:
/// client_secret:
/// username: your reddit username
/// password: your reddit password
#[derive(Deserialize)]
pub struct Auth {
    pub user_agent: String,
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,

}

pub struct OAuthURL {
    pub client_id: String, // client app you made during registration
    pub response_type: String, // must be string code
    pub state: String, // Generate a unique random string for each authorization request, use nanoid
    pub redirect_uri: String, // http://localhost:<port>/authorize_callback
    pub duration: String, // temporary, permanent
    pub scope: String, // Space separated list of scope strings
}

pub fn get_auth_duration(is_tmp: bool)-> &'static str {
    if is_tmp {
        "temporary"
    } else {
        "permanent"
    }
}

pub async fn make_request() {
    let client = reqwest::Client::new();
        // impl Future<Output= Result<Response,Error>>
    // Future type Output: the type of value produced on completion
    let res=client.get("https://reddit.com/r/rust")
        .send() 
        .await;
    let response = match res {
        Ok(response) => {
            println!("Got a response");
            Ok(response)
        },
        Err(err) => {
            println!("[ERROR]: {}", err);
            Err(err)
        }
    };

    let response_status = response.unwrap().status();
    println!("response_status: {}", response_status.as_str());
}