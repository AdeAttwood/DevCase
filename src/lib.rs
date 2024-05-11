use regex::Regex;

fn concert_replacement(original: &str, replacement: &str) -> String {
    if cruet::is_camel_case(original) {
        cruet::to_camel_case(replacement)
    } else if cruet::is_kebab_case(original) {
        cruet::to_kebab_case(replacement)
    } else if cruet::is_pascal_case(original) {
        cruet::to_pascal_case(replacement)
    } else if cruet::is_snake_case(original) {
        cruet::to_snake_case(replacement)
    } else {
        replacement.to_string()
    }
}

pub fn replace(search: &String, replace: String, input: String) -> String {
    let mut index = 0;
    let mut output = input;
    let search_pattern = match Regex::new(&format!("(?i){search}")) {
        Ok(pattern) => pattern,
        Err(_) => return output,
    };

    while let Some(search_match) = search_pattern.find_at(&output, index) {
        let start = search_match.start();
        let end = search_match.end();

        let mut replacement = String::new();
        match search_pattern.captures(&output[start..end]) {
            Some(captures) => {
                captures.expand(&replace, &mut replacement);
            }
            None => {
                replacement.push_str(&replace);
            }
        };

        let converted_replacement = concert_replacement(&output[start..end], &replacement);
        output.replace_range(start..end, &converted_replacement);

        index = start + converted_replacement.len();
    }

    output
}

#[cfg(test)]
mod tests {
    #[test]
    fn runs_the_search_and_replace() {
        assert_eq!(
            String::from("foo foo"),
            super::replace(
                &String::from(r"\b\w+\b"),
                String::from("foo"),
                String::from("bar baz")
            ),
        );
    }
}
