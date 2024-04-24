#[test]
fn convert() {
    let tests = [
    ("⚡️🤯ТАЄМНИЙ план Трампа щодо України🔥🦾Часів Яр: АВДІЇВКА-2?✈️Дрони на 3000 км! Новини від Яніни",
    "ТАЄМНИЙ план Трампа щодо України Часів Яр: АВДІЇВКА-2? Дрони на 3000 км! Новини від Яніни"),
    ("Армія РФ штурмує позиції ЗСУ на Донбасі: «Ідуть напролом. Їх не цікавить кількість загиблих»",
    "Армія РФ штурмує позиції ЗСУ на Донбасі: «Ідуть напролом. Їх не цікавить кількість загиблих»")
    ];

    for (i, t) in tests {
        let o: String = i
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
        let o = o.split_whitespace().collect::<Vec<&str>>().join(" ");
        println!("{}", o.trim());
        let parts: Vec<&str> = o.split(" ").collect();
        let mid = parts.len() / 2;
        let top_text = parts[..mid].join(" ");
        let bottom_text = parts[mid..].join(" ");
        println!("TOP:\n{}\nBOTTOM:\n{}", top_text, bottom_text);
        assert_eq!(t, o.trim())
    }
}
