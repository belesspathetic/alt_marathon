use regex::Regex;

pub fn gen_text(title: String) -> (String, String, String) {
    let re = Regex::new(r"\p{Emoji}").unwrap();
    let clean_text = re.replace_all(title.as_str(), "");
    let parts: Vec<&str>;
    let mut _top_text = String::new();
    let mut _bottom_text = String::new();

    let mut _font_size = String::new();

    // Generate text
    if clean_text.contains(".") {
        parts = clean_text.split(".").collect();
        if parts.len() >= 4 {
            _top_text = format!("{}{}", parts[0], parts[1]);
            _bottom_text = format!("{}{}", parts[2], parts[3]);
        } else {
            _top_text = parts[0].to_string();

            if parts[1].chars().count() >= 45 || parts[0].chars().count() >= 45 {
                _font_size = "26".to_string()
            } else {
                _font_size = "42".to_string()
            }
            _bottom_text = parts[1].to_string();
        }
    } else if clean_text.contains("|") {
        parts = clean_text.split("|").collect();
        _top_text = parts[0].to_string();
        _bottom_text = parts[1].to_string();
        if parts[1].chars().count() >= 45 || parts[0].chars().count() >= 45 {
            _font_size = "26".to_string()
        } else {
            _font_size = "42".to_string()
        }
    } else {
        parts = clean_text.split(" ").collect();
        let mid = parts.len() / 2;
        _top_text = parts[..mid].join(" ");
        _bottom_text = parts[mid..].join(" ");
        _font_size = "26".to_string()
    }

    (_top_text, _bottom_text, _font_size)
}
