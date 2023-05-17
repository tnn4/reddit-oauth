use std::fs;

use std::collections::HashMap;

use toml;
use querystring::{querify,stringify};
use nanoid::nanoid;

// ## querystring ##
// type QueryParam<'a> = (&'a str, &'a str)
// type QueryParams<'a> = Vec<QueryParam<'a>>;

// Reddit 
// see: https://github.com/reddit-archive/reddit/wiki/OAuth2
// 60 requests / minute ~ 1 request sec
// Monitor following reponse headers to ensure rates
// X-Ratelimit-Used
// X-Ratelimit-Remaining
// X-Ratelimit-Reset

// Reddit Authorization
// Send the following:
// https://www.reddit.com/api/v1/authorize?client_id=CLIENT_ID&response_type=TYPE&
// state=RANDOM_STRING&redirect_uri=URI&duration=DURATION&scope=SCOPE_STRING

const RESPONSE_TYPE: &'static str ="code";


struct OAuthURL {
    client_id: String, // client app you made during registration
    response_type: String, // must be string code
    state: String, // Generate a unique random string for each authorization request, use nanoid
    redirect_uri: String, // http://localhost:<port>/authorize_callback
    duration: String, // temporary, permanent
    scope: String, // Space separated list of scope strings
}

fn get_auth_duration(is_tmp: bool)-> &'static str {
    if is_tmp {
        "temporary"
    } else {
        "permanent"
    }
}

async fn make_request() {
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

#[tokio::main]
async fn main(){

    let id=nanoid!();

    let port=7778;
    let redirect_uri=format!("http://localhost:{}/authorize_callback",port);
    let is_tmp=true;

    let my_oauth_struct = OAuthURL{
        client_id: "some_id".to_string(),
        response_type: RESPONSE_TYPE.to_string(),
        state: nanoid!(),
        redirect_uri: format!("http://localhost:{}/authorize_callback", port),
        duration: "temporary".to_string(),
        scope: String::from("identity edit flair history modconfig modflair modlog modposts modwiki mysubreddit privatemessages read report save submit subscribe vote wikiedit wikiread"),
    };

    let my_oauth = vec!(
        ("client_id", "some_id"),
        ("response_type", RESPONSE_TYPE),
        ("state", &id),
        ("redirect_uri", &redirect_uri),
        ("duration", get_auth_duration(is_tmp)),
    );

    // Build authorization URL
    let reddit_url=String::from("https://www.reddit.com/api/v1/authorize?");
    let query_string=querystring::stringify(my_oauth);
    let final_auth_url=format!("{}{}",reddit_url, query_string);
    println!("Authorization URL is:{} ", final_auth_url);

}