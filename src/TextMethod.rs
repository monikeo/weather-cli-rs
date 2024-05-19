pub fn convert_to_title_case(text: &str) -> String {
    let mut converted_text = String::with_capacity(text.len());
    let mut capitalize_next = true;

    for ch in text.chars() {
        if capitalize_next {
            converted_text.extend(ch.to_uppercase());
            capitalize_next = false;
        } else {
            converted_text.push(ch);
        }
        capitalize_next = ch.is_whitespace();
    }
    converted_text
}
