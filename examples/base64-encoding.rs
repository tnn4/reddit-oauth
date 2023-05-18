use base64::{engine::general_purpose, Engine as _};


fn main() {
    // Now you have to send a post request
    // to this target: https://www.reddit.com/api/v1/access_token
    // with this POST data:
    // grant_type=authorization_code&code=CODE&redirect_uri=URI
    //
    // grant_type= authorization_code, using standard code based flow
    // code = code you retrieved
    // redirect_uri = you already have it
    let code="some_code";
    let redirect_uri="http://localhost:7778/authorize_callback";
    let token_url="https://www.reddit.com/api/v1/access_token";
    println!("Sending POST request to {}", token_url);
    let client_id="d#432jdXJFret";
    let client_secret="&293432sdfhcx";

    let post_body=format!("grant_type={}&code={}&redirect_uri={}", "authorization_code", &code, &redirect_uri);
    let credentials = format!("{}:{}",&client_id,&client_secret);
    let credentials_base64=general_purpose::STANDARD.encode(&credentials); 
    println!("[BASE64_ENCODING]: {}",credentials_base64);
    println!("[POST_BODY]: {}", post_body);

    let aladdin_base64=general_purpose::STANDARD.encode("Aladdin:open sesame");
    assert_eq!("QWxhZGRpbjpvcGVuIHNlc2FtZQ==", aladdin_base64.as_str());

}