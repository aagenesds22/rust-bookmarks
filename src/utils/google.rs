extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
const SEARCH_URI: &'static str = "search?q=";

pub fn make_google_search_url(url: &str, query_string: Option<String>) -> String {
    let mut search_url: String = String::from(url);

    if query_string == None {
        return String::from(url);
    }

    search_url.push_str(SEARCH_URI);

    let unwrapped_query_opt = query_string.as_ref().unwrap();

    let query_encoded = utf8_percent_encode(unwrapped_query_opt, FRAGMENT);
    
    let return_search_url = format!("{}{}", search_url, query_encoded); 

    println!("{}", return_search_url);
    
    return_search_url

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_google_search_url() {
        let query:Option<String> = Some(String::from("Test"));
        let expected = "https://google.com/search?q=Test";
        assert_eq!(make_google_search_url("https://google.com/", query), expected);

    }

    #[test]
    fn test_google_encoded_search_url() {
        let query:Option<String> = Some(String::from("Test two"));
        let expected = "https://google.com/search?q=Test%20two";
        assert_eq!(make_google_search_url("https://google.com/", query), expected);

    }
}