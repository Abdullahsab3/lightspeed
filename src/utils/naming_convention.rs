pub fn to_camel_case(snake_case: &str) -> String {
    let mut camel_case = String::new();
    let mut capitalize_next = false;
    for c in snake_case.chars() {
        if c == '_' {
            capitalize_next = true;
        } else {
            if capitalize_next {
                camel_case.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                camel_case.push(c);
            }
        }
    }
    camel_case
}

pub fn to_snake_case(camel_case: &str) -> String {
    let mut snake_case = String::new();
    let mut camel_case_chars = camel_case.chars();
    snake_case.push(camel_case_chars.next().unwrap().to_ascii_lowercase());

    for c in camel_case_chars {
        if c.is_ascii_uppercase() {
            snake_case.push('_');
            snake_case.push(c.to_ascii_lowercase());
        } else {
            snake_case.push(c);
        }
    }
    snake_case
}

pub fn to_upper_snake_case(camel_case: &str) -> String {
    to_snake_case(camel_case).to_ascii_uppercase()
}




pub fn kebab_to_snake_case(kebab_case: &str) -> String {
    kebab_case.replace("-", "_")
}