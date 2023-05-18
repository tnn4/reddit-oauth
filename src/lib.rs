// use serde_derive::Deserialize;

use std::fs;
use std::env;

use toml;
use querystring::{querify,stringify};
use nanoid::nanoid;
use clap::Parser;

use reqwest::Response;
use url::{Url, ParseError};
use serde::Deserialize;
use base64::{engine::general_purpose, Engine as _};

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

#[derive(Deserialize)]
pub struct Credentials {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,// unix epoch in seconds
    pub scope: String,
    pub refresh_token: Option<String>,
}

pub fn get_auth_duration_type(is_tmp: bool)-> &'static str {
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

pub async fn get_credentials(client: reqwest::Client, code: String, redirect_uri: String,
    client_id: String, client_secret: String) -> Credentials {
    // Now you have to send a post request
    // to this target: https://www.reddit.com/api/v1/access_token
    // with this POST data:
    // grant_type=authorization_code&code=CODE&redirect_uri=URI
    //
    // grant_type= authorization_code, using standard code based flow
    // code = code you retrieved
    // redirect_uri = you already have it
    
    let token_url="https://www.reddit.com/api/v1/access_token";
    println!("Sending POST request to {}", token_url);

    // s[0..len] is length of entire string
    let code_len = &code.len();
    let fixed_code=&code[..(code_len-3)];

    let post_body=format!("grant_type={}&code={}&redirect_uri={}", "authorization_code", fixed_code, &redirect_uri);
    
    //todo!("Fix base64 encoding");
    let credentials = format!("{}:{}",&client_id,&client_secret);
    let credentials_base64=general_purpose::STANDARD.encode(&credentials); 
    
    println!("[BASE64_ENCODED_CREDENTIALS]: {}", credentials_base64);
    println!("[POST_BODY]: {}", post_body);

    let res2 = client.post(token_url)
    // see: https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication
    // The Authorization and Proxy-Authorization request headers contain the credentials to authenticate a user agent with a (proxy) server. 
    // Authorization: <type> <credentials>
    // user     client_id
    // password client_secret
    // If the user agent wishes to send the userid "Aladdin" and password
    // "open sesame", it would use the following header field:
    // see: https://www.geeksforgeeks.org/http-headers-authorization/#
    // Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==
        .header(reqwest::header::AUTHORIZATION, "Basic ".to_owned() + &credentials_base64)
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(post_body)
        .send()
        .await; // -> Result<Response,Error>

    let response_ok = match res2 {
        Ok(response) => {
            Ok(response)
        },
        Err(err) => {
            println!("[ERROR from Response]: {}", err);
            Err(err)
        }
    };


    let response = response_ok.unwrap();

    let status_code = response.status();
    let response_headers = response.headers();
    println!("[STATUS_CODE]: {}", status_code.as_str());
    println!("[RESPONSE_HEADERS]: {:?}", response_headers);

    /*
    let response_text_result = response.text().await;
    let response_text = match response_text_result {
        Ok(text) => {
            println!("[RESPONSE_TEXT]: {}", text);
            text
        },
        Err(err) => "error".to_string(),
    };
    //println!("[RESPONSE_TEXT]: {}", response_text);
    */
    if status_code.as_u16() == 200 {
        println!("[OK]: success, you should now have a JSON body to retrieve");
    } else if status_code.as_u16() == 401 {
        panic!("[ERROR]: Invalid credentials were supplied");
    } else if status_code.as_u16() == 400 {
        panic!("[ERROR]: Bad Request");
    } else {
        panic!("[ERROR]: something went wrong, Cannot continue.");
    }

    // run: $cargo add reqwests --features json
    // you'll need it to retrieve the access token
    /*
    {
        "access_token": Your access token,
        "token_type": "bearer",
        "expires_in": Unix Epoch Seconds,
        "scope": A scope string,
        "refresh_token": Your refresh token
    }
     */
    
    // Deserialize the json
    let json_result = response.json::<Credentials>().await;
    let json_ok:Result<Credentials, ()> = match json_result {
        Ok(credentials) => Ok(credentials),
        Err(err) => {
            println!("Unable to deserialize JSON into the struct");
            panic!("{}", format!("error: {}",err));
        },
    };

    let credentials=json_ok.unwrap();
    let access_token=&credentials.access_token;
    let token_type=&credentials.token_type;
    let expires_in=&credentials.expires_in;
    let scope=&credentials.scope;
    let refresh_token_maybe=&credentials.refresh_token;
    if let None = refresh_token_maybe {
        println!("No refresh token found");
    } else {
        println!("Found a refresh token.");
    }

    println!("\nACCESS_TOKEN]: {}", access_token);
    println!("[EXPIRES_IN]: {}", expires_in);
    
    println!("You may now make API requests to reddit's servers on behalf of the user the token is for"); 
    println!("by including the following header in your HTTP requests to endpoints listedat https://www.reddit.com/dev/api/:");
    println!("Authorization: bearer ACCESS_TOKEN");
    /*
    let res = client
        .post(url)
        .header("Authorization", access_token)
        .send()
        .await;
     */
     credentials
}