extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use std::collections::HashMap;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`').add(b'-').add(b'_');
const SEARCH_PULL: &'static str = "/pull/";


pub fn make_github_search_url(url: &str, query_string: Option<String>) -> String {
    let mut search_url: String = String::from(url);

    if query_string == None {
        return String::from(url);
    }

    let unwrapped_query_opt = query_string.as_ref().unwrap();

    if unwrapped_query_opt.contains("@") && unwrapped_query_opt.find("@") == Some(0) {
        search_url.push_str(&unwrapped_query_opt[1..]);
        return search_url;
    }

    let splitted_query = unwrapped_query_opt.split_whitespace();

    let vectored_query = &splitted_query.collect::<Vec<_>>();

    let mut hash_org:HashMap<String, String> = HashMap::new();

    if vectored_query.len() == 3 {
        hash_org.insert(String::from("org"), String::from(vectored_query[0]));
        hash_org.insert(String::from("repo"), String::from(vectored_query[1]));
        hash_org.insert(String::from("pull"), String::from(vectored_query[2]));
        let format_org_repo = format!("{}/{}", hash_org.get("org").unwrap(), hash_org.get("repo").unwrap());
        search_url.push_str(&format_org_repo);
        search_url.push_str(SEARCH_PULL);
        println!("{}", search_url);
        let return_search_url = format!("{}", utf8_percent_encode(&format!("{}{}", search_url, hash_org.get("pull").unwrap()), FRAGMENT));
        println!("{}", return_search_url);
        String::from(&return_search_url)
    } else {
        hash_org.insert(String::from("org"), String::from(vectored_query[0]));
        hash_org.insert(String::from("repo"), String::from(vectored_query[1]));
        let format_org_repo = format!("{:?}/{:?}", &hash_org.get("org"), &hash_org.get("repo"));
        search_url.push_str(&format_org_repo);
        let return_search_url = format!("{}", search_url);
        return_search_url
    }

}