#![feature(proc_macro_hygiene, decl_macro)]
#![deny(elided_lifetimes_in_paths)]

#[macro_use] extern crate rocket;

use rocket::response::Redirect;
mod utils;

struct WebsiteShortcuts {
    tw: String,
    gh: String,
    fb: String,
    go: String,
}

impl WebsiteShortcuts {
    fn new() -> WebsiteShortcuts {
        WebsiteShortcuts {
            tw: String::from("https://twitter.com/"),
            gh: String::from("https://github.com/"),
            fb: String::from("https://facebook.com/"),
            go: String::from("https://google.com/"),
        }
    }

    fn match_url(self, cmd: String, query: Option<String>) -> String {

        let stringified_cmd:&str = cmd.as_str();

        match stringified_cmd {
            "tw" => utils::twitter::make_twitter_search_url(&self.tw, query),
            "fb" => self.fb,
            "gh" => utils::github::make_github_search_url(&self.gh, query),
            _ => utils::google::make_google_search_url(&self.go, query),
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    let websites_url = WebsiteShortcuts::new();
    let (command_from_query, mut full_query) = utils::get_command_from_query_string(cmd);

    if full_query == None {
        full_query = Some(String::from(&command_from_query));
    }
    
    let redirect_url = websites_url.match_url(command_from_query, full_query);
    Redirect::to(redirect_url)
}

fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}


#[cfg(test)]
mod tests {

    // use super::*;

    // #[test]
    // fn test_get_command_from_query_string_no_whitespace() {
    //     let actual = get_command_from_query_string("tw");
    //     let expected = "tw";
    //     assert_eq!(actual, expected);

    // } 

    // fn test_get_command_from_query_string_with_whitespace() {
    //     let actual = get_command_from_query_string("tw @fbOpenSource");
    //     let expected = "tw";
    //     assert_eq!(actual, expected);

    // } 
}