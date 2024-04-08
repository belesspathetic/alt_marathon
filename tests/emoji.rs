#[test]
fn convert() {
    let tests = [
    ("⚡️🤯ТАЄМНИЙ план Трампа щодо України🔥🦾Часів Яр: АВДІЇВКА-2?✈️Дрони на 3000 км! Новини від Яніни",
    "ТАЄМНИЙ план Трампа щодо України Часів Яр: АВДІЇВКА-2? Дрони на 3000 км! Новини від Яніни"),
    ("⚡️⚡️БІНДЕРИ ОБСТРЕЛЯЛИ! МАМОЧКА, МЫ ТОНЕМ! Після ЦИХ кадрів Сімоньян у ВСЬОМУ звинуватила...",
    "БІНДЕРИ ОБСТРЕЛЯЛИ! МАМОЧКА, МЫ ТОНЕМ! Після ЦИХ кадрів Сімоньян у ВСЬОМУ звинуватила...")
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
                    Some(' ') // Заменяем символ-эмодзи на пробел, возвращая None
                } else {
                    Some(c) // Оставляем другие символы без изменений
                }
            })
            .collect();
        let o = o.split_whitespace().collect::<Vec<&str>>().join(" ");
        println!("{}", o.trim());
        assert_eq!(t, o.trim())
    }
}
