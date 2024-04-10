pub fn reverse(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    if input.is_empty() {
        return String::new();
    }
    let mut start = 0;
    let mut end = chars.len() - 1;

    while start < end {
        chars.swap(start, end);

        start += 1;
        end -= 1;
    }

    let reversed_string: String = chars.into_iter().collect();

    reversed_string
}
