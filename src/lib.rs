use regex::Regex;

pub fn replace(search: &String, replace: String, input: String) -> String {
    let mut index = 0;
    let mut output = input;
    let search_pattern = Regex::new(search).unwrap();

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

        index = start + replacement.len();
        output.replace_range(start..end, &replacement);
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
