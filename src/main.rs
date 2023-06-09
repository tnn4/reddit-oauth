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

#[derive(Parser,Debug)]
#[command(author="tnn4",version="v0.1.0", about="about", long_about=None)]
struct Cli {
    #[arg(short,long)]
    get_auth: bool,
}

// https://github.com/reddit-archive/reddit/wiki/OAuth2
#[tokio::main]
async fn main(){

    // ## DATA ## 

    let args = Cli::parse();

    let state_id=nanoid!(); // random id
    let port=7778; // redirect port
    let redirect_uri=format!("http://localhost:{}/authorize_callback", port);
    let is_tmp=true; // Indicates whether or not your app needs a permanent token.
    let auth_file_name = "auth.toml";  // your auth-credentials file location
    let scope =  String::from( // these are the scopes the app will need
        "account,edit,flair,history,identity,livemanage,\
        modconfig,modcontributors,modflair,modlog,modmail,\
        modnote,modothers,modposts,modself,modwiki,mysubreddits,\
        privatemessages,read,report,save,structuredstyles,submit,\
        subscribe,vote,wikiedit,wikiread"
    );

    let _my_oauth_struct = rsraw::OAuthURL{
        client_id: "some_id".to_string(),
        response_type: RESPONSE_TYPE.to_string(),
        state: nanoid!(),
        redirect_uri: format!("http://localhost:{}/authorize_callback", port),
        duration: "temporary".to_string(),
        scope: scope.clone(),
    };


    
    // ## AUTH FILE PROCESSING ##
    // Put your information in the auth.toml, and keep it secure
    // Read from ../auth.toml and put info into our struct
    let _auth_toml_as_string = fs::read_to_string("../auth.toml");
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
    
    let client_id =      data.auth.client_id.clone();
    let client_secret =  data.auth.client_secret.clone();
    let reddit_username =       data.auth.username.clone();
    let reddit_password =       data.auth.password.clone();

    let my_oauth = vec!(
        ("client_id", client_id.as_str()),
        ("response_type", RESPONSE_TYPE),
        ("state", &state_id),
        ("redirect_uri", &redirect_uri),
        ("duration", rsraw::get_auth_duration_type(is_tmp)),
        ("scope", &scope.as_str()),
    );

    // Build authorization URL
    let reddit_url=String::from("https://www.reddit.com/api/v1/authorize?");
    let query_string=querystring::stringify(my_oauth);
    let final_auth_url=format!("{}{}",reddit_url, query_string);
    println!("[AUTHORIZATION_URL]:\n{} ", final_auth_url);

    // ## Retrieve Access Token ##
    // Auth URL  now works, its time to retrieve the access token

    // ## Build User agent URL ##
    // IMPORTANT: Make sure to send a user agent with your request
    // <platform>:<app ID>:<version string> (by /u/<reddit username>)

    // https://dtantsur.github.io/rust-openstack/reqwest/struct.ClientBuilder.html#method.user_agent
    let user_agent=get_user_agent(reddit_username);
    println!("\n[USER_AGENT]: {}\n",user_agent);
    // ## Client Configuration 
    
    let client_result = reqwest::Client::builder()
        .user_agent(user_agent)
        .build(); // -> Result<Client>

    let client_ok = match client_result {
        Ok(client) => Ok(client),
        Err(err) => {
            println!("[ERR]: Unable to build client: {}", err);
            Err(err)
        }
    };

    // impl Future<Output= Result<Response,Error>>
    // Future type Output: the type of value produced on completion
    
    // Do get request if --get-auth flag is present
    if args.get_auth == true {
        // keep going
        
    } else {
        println!("\nrun: `cargo run --release -- --get-auth` to actually send a
            GET request with your authorization url\n");
        return
    }
    let client= client_ok.unwrap();
    
    
    // ## GET request to auth-url ##
    let res=client.get(&final_auth_url)
        .send() 
        .await; // -> Result<Response, Error>

    // you're going to get a redirect URL
    // HTTP redirect code = 301

    // see: https://docs.rs/reqwest/latest/reqwest/struct.Response.html
    let response_ok : Result<Response,reqwest::Error> = match res {
        Ok(response) => {
            println!("Got a response.");
            Ok(response)
        },
        Err(err) => {
            println!("[ERROR:RESPONSE]: {}", err);
            panic!("Got an invalid response from the server. Cannot continue.")
        }
    };


    let response=response_ok.unwrap();

    let response_status = response.status();
    
    // You need to parse the query parameters for the URI for use:
    // see: https://github.com/reddit-archive/reddit/wiki/OAuth2#token-retrieval-code-flow
    // error=error codes
    // code= one time use code that may be exchanged for bearer token
    // state= this value should be the same one as the initial request, you need to verify this
    
    // this should be the redirect URL reddit gives you
    // it's not
    let response_url=response.url().clone();

    // let test_string="http://localhost:7778/authorize_callback?state=AXNJ_k_QMVBAgT79KyUjw&code=YcIqKbQ72V9R3wXSXnGz9GV3i_EBQQ#_";
    
    println!("[RESPONSE_STATUS]: {}", response_status.as_str());
    println!("[RESPONSE_URL]: {}", response_url);
    
    // https://docs.rs/querystring/latest/querystring/fn.querify.html
    let base_with_query_url = response_url.clone();
    
    let mut query_url = "";

    // todo!("figure out how to automate this");
    // We have to manually get the redirected authorization url for now
    let mut redirect_auth_url = String::new();
    let stdin = std::io::stdin();
    println!("Copy and paste the redirect URL you got here, it should have state and code parameters in the query string: ");
    stdin.read_line(&mut redirect_auth_url);

    // we need to isolate the query string
    if redirect_auth_url == "" { 
        println!("[ERROR]: you need a redirect URL with the proper credentials");
    } else {
        match Url::parse(redirect_auth_url.as_str()) {
            Ok(url) =>{},
            Err(parse_error) => {
                panic!("Invalid URL detected. Cannot continue.");
            }
        }

    }
    query_url = &redirect_auth_url;
    // Isolate query string
    let to_be_replaced = format!("{}?", redirect_uri);
    let query_url2 = &query_url.replace(to_be_replaced.as_str(), ""); // this does nothing for some reason
    // note: this returns the replaced string as a new allocation, without modifying the original <-- whoops

    // let len_to_strip = redirect_uri.len()+1;
    // query_url = &query_url[len_to_strip..];// we'll take a string slice instead

    println!("\n[QUERY_STRING_URL]: {}", &query_url2);
    
    let query_list = querystring::querify(&query_url2);
    
    // place to store one-time use code
    let mut code: &str = "";
    // see: https://en.wikipedia.org/wiki/Query_string
    // query strings are composed of series of field-value pairs
    for query in query_list.iter() {
        println!("\n[QUERY]: {:?}", query);
        // parse tuple
        let (field, mut value) = query;
        // error check
        if field == &"error" {
            println!("[ERR:RESPONSE]: {}",value);
            return
        }

        // state check
        if field == &"state" {
            assert_eq!(value, &state_id);
        }

        // save the code if the query was valid
        if field == &"code" {
            println!("[OK]: we got a code");
            // sanitize the value
            trim_newline(&mut value.to_string());
            code = value;
        } else {
            println!("[ERROR]: missing CODE");
        }
    }

    use std::borrow::Borrow;
    // ERROR
    let response_text_result=response.text().await; // ERROR
    let response_text = match response_text_result {
        Ok(string) => {
            string
        },
        Err(err) => {
            println!("[ERR]: {}",err);
            err.to_string()
        }
    };

    // println!("response_text: {}", response_text);
    // 
    // ## It's time to get our access token ##
    /*
    https://www.reddit.com/r/redditdev/comments/xdud2v/bad_request_400_when_requesting_reddit_oauth2/
    Redirect URI Fix Fragments

    The last, but likely least impactful, change we're implementing is adding a "fix fragment" #_ 
    to the end of the redirect URI in the Location header in response to a POST request to /api/v1/authorize. 
    This should be transparent as browsers and url parsers should drop the fragment when redirecting.
    
    FIXED
     */
    println!("[CODE]: {}",code);
    if code == "" {
        panic!("[ERROR]: No code. Something's wrong");
    }

    let credentials: rsraw::Credentials = rsraw::get_credentials(client, code.to_string(), redirect_uri, client_id, client_secret).await;

}



fn get_user_agent(_reddit_username: String) -> String {
    // IMPORTANT: Make sure to send a user agent with your request
    // <platform>:<app ID>:<version string> (by /u/<reddit username>)
    let platform = env::consts::OS;
    let app_id = "test-app-1";
    let version_string="v0.1.0";
    let user_string=format!("(by /u/{})", _reddit_username);

    // https://dtantsur.github.io/rust-openstack/reqwest/struct.ClientBuilder.html#method.user_agent
    let user_agent=format!("{}:{}:{} {}",platform,app_id,version_string,user_string);
    user_agent
}



fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

