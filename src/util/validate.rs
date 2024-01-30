pub fn validate(input: &str) -> bool {
    let allowed_chars = "01";
    let valid = input.len() == 3 && input.chars().all(|c| allowed_chars.contains(c));
    if !valid {
        println!("Please input using only 0 or 1, and only 3 characters long")
    }
    valid
}
