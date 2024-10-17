use regex::Regex;

pub fn strip_irc_formatting(input: &str) -> String {
    // Remove color codes
    let re_color = Regex::new(r"\x03\d{1,2}(?:,\d{1,2})?").unwrap();
    let without_color = re_color.replace_all(input, "");

    // Remove other formatting codes
    let re_formatting = Regex::new(r"[\x02\x1D\x1F\x16\x0F]").unwrap();
    re_formatting.replace_all(&without_color, "").to_string()
}

