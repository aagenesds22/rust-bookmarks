pub mod google;
pub mod twitter;
pub mod github;

pub fn get_command_from_query_string(query_string: String) -> (String, Option<String>) {

    // exercise: convert to "if let" the following 4 lines
    let local_query_string = &query_string;

    if query_string.contains(' ') && query_string.find(' ') > Some(0) {
        let index_of_space = query_string.find(' ').unwrap_or(0);
        let command_stringified = String::from(&local_query_string[..index_of_space]);
        let parsed_query = String::from(&local_query_string[index_of_space+1..]);
        return (command_stringified, Some(parsed_query));
    }
    println!("get command {}", query_string);
    (query_string, None)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_command_from_query_string_no_whitespace() {
        let (actual_tw, _query_string) = get_command_from_query_string(String::from("tw"));
        let (actual_gh, _query_string) = get_command_from_query_string(String::from("gh"));
        let (actual_fb, _query_string) = get_command_from_query_string(String::from("fb"));

        let expected_tw = "tw";
        let expected_fb = "fb";
        let expected_gh = "gh";
        assert_eq!(actual_tw, expected_tw);
        assert_eq!(actual_gh, expected_gh);
        assert_eq!(actual_fb, expected_fb);
    } 
    
    #[test]
    fn test_get_command_from_query_string_with_whitespace() {
        let (actual, _query_string) = get_command_from_query_string(String::from("tw @fbOpenSource"));
        let expected = "tw";
        assert_eq!(actual, expected);
    } 

    #[test]
    fn test_get_command_and_parsed_query_username() {
        let (_actual_cmd, actual_query_string) = get_command_from_query_string(String::from("tw @fbOpenSource"));

        let expected:Option<String> = Some(String::from("@fbOpenSource"));

        assert_eq!(actual_query_string, expected);
    }
}