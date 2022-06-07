extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
const SEARCH_URI: &'static str = "search?q=";

pub fn make_twitter_search_url(url: &str, query_string: Option<String>) -> String {
    let mut search_url: String = String::from(url);

    if query_string == Some("tw".to_string()) || query_string == None {
        return search_url;
    }

    let unwrapped_query_opt = query_string.as_ref().unwrap();


    if unwrapped_query_opt.contains('@') {
        search_url.push_str(&unwrapped_query_opt[1..]);
        return search_url;
    }

    search_url.push_str(SEARCH_URI);
    let query_encoded = utf8_percent_encode(unwrapped_query_opt, FRAGMENT);
    
    let return_search_url = format!("{}{}", search_url, query_encoded); 
    
    return_search_url

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_twitter_single_redirect() {
        let query:Option<String> = None;
        let tw_query:Option<String> = Some(String::from("tw"));

        let actual = make_twitter_search_url("https://twitter.com/", query);
        let expected = String::from("https://twitter.com/");

        let actual_tw = make_twitter_search_url("https://twitter.com/", tw_query);

        assert_eq!(actual, expected);
        assert_eq!(actual_tw, expected);
    }

    #[test]
    fn make_twitter_search_query() {
        let query:Option<String> = Some(String::from("Covid test"));

        let actual = make_twitter_search_url("https://twitter.com/", query);
        let expected = String::from("https://twitter.com/search?q=Covid%20test");

        assert_eq!(actual, expected);
    }

    #[test]
    fn make_twitter_profile() {
        let query:Option<String> = Some(String::from("@1_farmaceutico"));

        let actual = make_twitter_search_url("https://twitter.com/", query);

        let expected = String::from("https://twitter.com/1_farmaceutico");

        assert_eq!(actual, expected);

    }
}