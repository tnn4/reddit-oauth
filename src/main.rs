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






// https://github.com/reddit-archive/reddit/wiki/OAuth2
#[tokio::main]
async fn main(){

    // ## DATA ## 

    let state_id=nanoid!(); // random id
    let port=7778; // redirect port
    let redirect_uri=format!("http://localhost:{}/authorize_callback", port);
    let is_tmp=true; // Indicates whether or not your app needs a permanent token.
    let auth_file_name = "auth.toml";  // your auth-credentials file location
    let scope =  String::from(
        "identity,edit,flair,history,modconfig,modflair,\
        modlog,modposts,modwiki,mysubreddit,privatemessages,\
        read,report,save,submit,subscribe,vote,wikiedit,wikiread"
    );

    let my_oauth_struct = rsraw::OAuthURL{
        client_id: "some_id".to_string(),
        response_type: RESPONSE_TYPE.to_string(),
        state: nanoid!(),
        redirect_uri: format!("http://localhost:{}/authorize_callback", port),
        duration: "temporary".to_string(),
        scope: scope.clone(),
    };


    
    // ## AUTH FILE PROCESSING ##
    // Read from ../auth.toml and put info into our struct
    let auth_toml_as_string = fs::read_to_string("../auth.toml");
    // Read contents of auth.toml
    let contents = match fs::read_to_string(auth_file_name) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("[ERROR] reading file `{}`", auth_file_name);
            std::process::exit(1);
        }
    };

    // Process (deserialize) contents of auth file into Data struct
    let data: rsraw::Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("[ERROR] unable to load data from {}", auth_file_name);
            std::process::exit(1);
        }
    };
    
    let user_agent =     data.auth.user_agent.clone();
    let client_id =      data.auth.client_id.clone();
    let client_secret =  data.auth.client_secret.clone();
    let username =       data.auth.username.clone();
    let password =       data.auth.password.clone();

    let my_oauth = vec!(
        ("client_id", client_id.as_str()),
        ("response_type", RESPONSE_TYPE),
        ("state", &state_id),
        ("redirect_uri", &redirect_uri),
        ("duration", rsraw::get_auth_duration(is_tmp)),
        ("scope", &scope.as_str()),
    );

    // Build authorization URL
    let reddit_url=String::from("https://www.reddit.com/api/v1/authorize?");
    let query_string=querystring::stringify(my_oauth);
    let final_auth_url=format!("{}{}",reddit_url, query_string);
    println!("Your Authorization  URL is:\n{} ", final_auth_url);

}