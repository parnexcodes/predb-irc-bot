use regex::Regex;
use log::info;

pub fn parse_pre_message(msg: &str) -> Option<(String, String)> {
    let cleaned_msg = remove_irc_color_codes(msg);
    let re = Regex::new(r"(?i)\(PRE\)\s*\(([^)]+)\)\s*\((.+)\)").unwrap();
    
    let result = re.captures(&cleaned_msg).map(|captures| {
        let category = captures.get(1).map_or("", |m| m.as_str()).to_string();
        let release_name = captures.get(2).map_or("", |m| m.as_str()).to_string();
        (category, release_name)
    });
    
    if let Some((category, release_name)) = &result {
        info!("Parsed PRE message. Category: {}, Release: {}", category, release_name);
    }
    
    result
}

fn remove_irc_color_codes(input: &str) -> String {
    let re = Regex::new(r"\x03\d{0,2}(?:,\d{1,2})?").unwrap();
    re.replace_all(input, "").to_string()
}

pub fn extract_group_name(release_name: &str) -> String {
    release_name.split('-').last().unwrap_or("").trim_end_matches(')').to_string()
}
