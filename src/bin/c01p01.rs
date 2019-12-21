use std::collections::HashSet;

/*
Is Unique: Implement an algorithm to determine if a string has all unique characters. What if you
cannot use additional data structures?
*/

// O(n)
fn all_chars_unique_part_a(s: &str) -> bool {
    let mut chars = HashSet::new();
    let mut has_duplicate = false;
    for char in s.chars() {
        if chars.contains(&char) {
            has_duplicate = true;
            break;
        }
        chars.insert(char);
    }

    return !has_duplicate;
}

// O(n2)
fn all_chars_unique_part_b(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        None => false,
        Some(first_char) => {
            if chars.any(|char| char == first_char) {
                return false;
            }

            return !all_chars_unique_part_b(&s[1..]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        assert_eq!(all_chars_unique_part_a(&String::from("abcdefg")), true);
        assert_eq!(all_chars_unique_part_a(&String::from("abcdefga")), false);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(all_chars_unique_part_b(&String::from("abcdefg")), true);
        assert_eq!(all_chars_unique_part_b(&String::from("abcdefga")), false);
    }
}

fn main() {
    all_chars_unique_part_a(&String::from("helloworld"));
    all_chars_unique_part_b(&String::from("helloworld"));
}
