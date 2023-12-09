use reqwest::header::HeaderMap;

pub fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        "content-type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );

    return headers;
}

pub fn parse_recaptcha_response(value: String) -> String {
    value
        .split("resp\",\"")
        .last()
        .unwrap()
        .split('\"')
        .next()
        .unwrap()
        .to_string()
}

pub fn parse_recaptcha_token(value: String) -> String {
    value
        .split("id=\"recaptcha-token\" value=\"")
        .last()
        .unwrap()
        .split('\"')
        .next()
        .unwrap()
        .to_string()
}
