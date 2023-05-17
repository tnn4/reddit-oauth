use std::fs;
use std::env;

use toml;
use querystring::{querify,stringify};
use nanoid::nanoid;
use clap::Parser;

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
        ("duration", rsraw::get_auth_duration(is_tmp)),
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
    let platform = env::consts::OS;
    let app_id = "test-app-1";
    let version_string="v0.1.0";
    let user_string=format!("(by /u/{})", reddit_username);

    // https://dtantsur.github.io/rust-openstack/reqwest/struct.ClientBuilder.html#method.user_agent
    let user_agent=format!("{}:{}:{} {}",platform,app_id,version_string,user_string);
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

    // Send GET request to auth-url
    let res=client_ok.unwrap().get(&final_auth_url)
        .send() 
        .await;

    // you're going to get a redirect URL
    // HTTP redirect code = 301

    // see: https://docs.rs/reqwest/latest/reqwest/struct.Response.html
    let response_result = match res {
        Ok(response) => {
            println!("Got a response");
            Ok(response)
        },
        Err(err) => {
            println!("[ERROR]: {}", err);
            Err(err)
        }
    };

    let response=response_result.unwrap();

    let response_status = response.status();
    
    // You need to parse the query parameters for the URI for use:
    // see: https://github.com/reddit-archive/reddit/wiki/OAuth2#token-retrieval-code-flow
    // error=error codes
    // code= one time use code that may be exchanged for bearer token
    // state= this value should be the same one as the initial request, you need to verify this
    
    // this should be the redirect URL reddit gives you
    let response_url=response.url();

    // let test_string="http://localhost:7778/authorize_callback?state=AXNJ_k_QMVBAgT79KyUjw&code=YcIqKbQ72V9R3wXSXnGz9GV3i_EBQQ#_";
    
    println!("response_status: {}", response_status.as_str());
    println!("response_url: {}", response_url);
    
    // https://docs.rs/querystring/latest/querystring/fn.querify.html
    let base_with_query_url = response_url.clone();
    
    // we need to isolate the query string
    let query_url = base_url_with_query.to_string().replace(format!("{}?", redirect_uri).as_str(), "");
    
    println!("\n[QUERY_STRING_URL]: {}", query_url;
    
    let query_list = querystring::querify(&query_url);
    for query in query_list.iter() {
        println!("\n[QUERY_STRING]: {:?}", query_string);
    }
    
    
    // verify state
    assert_eq!("","");
    
    let response_text_result=response.text().await;
    let response_text = match response_text_result {
        Ok(string) => {
            string
        },
        Err(err) => {
            println!("[ERR]: {}",err);
            err.to_string()
        }
    };

    println!("response_text: {}", response_text);
}