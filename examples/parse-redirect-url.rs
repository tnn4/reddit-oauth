# example of isolating the query parameters

fn main() {
    let base_with_query_test_url="http://localhost:7778/authorize_callback?state=AXNJ_k_QMVBAgT79KyUjw&code=YcIqKbQ72V9R3wXSXnGz9GV3i_EBQQ#_";

    // println!("response_status: {}", response_status.as_str());
    // println!("response_url: {}", response_url);
    
    // https://docs.rs/querystring/latest/querystring/fn.querify.html
    let base_with_query_url = base_with_query_test_url.clone();
    
    
    // we need to isolate the query string by stripping the domain out, 
    // here it's http://localhost:7778/authorize_callback?
    let query_url=base_with_query_url.to_string().replace("http://localhost:7778/authorize_callback?", "");
    
    // let query_string_url = base_url_with_query.to_string().replace(format!("{}?", redirect_uri).as_str(), "");
    
    println!("\n[QUERY_STRING_URL]: {}", base_with_query_url);
    
    let query_list = querystring::querify(&query_url);
    for query in query_list.iter() {
        println!("\n[QUERY_STRING]: {:?}", query);
    }
    
    
    // verify state
    assert_eq!("","");
}