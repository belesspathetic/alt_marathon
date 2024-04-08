pub fn gen_text(title: String) -> (String, String, String) {
    let clean_text: String = title
        .replace("\u{fe0f}", " ")
        .chars()
        .filter_map(|c| {
            let uc = c as u32;
            if (uc >= 0x1F600 && uc <= 0x1F64F) ||   // Emoji in the basic plane
            (uc >= 0x1F300 && uc <= 0x1F5FF) ||   // Repair symbols
            (uc >= 0x1F680 && uc <= 0x1F6FF) ||   // Transport and map symbols
            (uc >= 0x2600 && uc <= 0x27BF) ||     // Weather symbols
            (uc >= 0x1F300 && uc <= 0x1F6FF) ||   // Emoji from addings A and B
            (uc >= 0x1F900 && uc <= 0x1F9FF)
            {
                Some(' ')
            } else {
                Some(c)
            }
        })
        .collect();
    let clean_text = clean_text
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");
    let parts: Vec<&str>;

    parts = clean_text.split(" ").collect();
    let mid = parts.len() / 2;
    let top_text = parts[..mid].join(" ");
    let bottom_text = parts[mid..].join(" ");
    let font_size = "32".to_string();

    (top_text, bottom_text, font_size)
}
