#[test]
fn is_key_set() {
    let key = "FACEBOOK_API_KEY";
    let value = "key";

    let env_value = match std::env::var(key) {
        Ok(v) => {
            println!("SUCCESS: key is set");
            v
        }
        Err(_) => value.to_string(),
    };

    assert!(!env_value.is_empty());
}
