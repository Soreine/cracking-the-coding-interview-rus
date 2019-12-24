use std::collections::HashMap;

/*
Check Permutation: Given two strings, write a method to decide if one is a permutation of the other.
*/

// // O(n2 log n)
// fn is_permutation(a: &str, b: &str) -> bool {
//     if a.len() != b.len() {
//         return false;
//     }

//     fn sort(s: &str) -> Vec<char> {
//         let mut s_vec = s.chars().collect::<Vec<char>>();
//         s_vec.sort_unstable();
//         return s_vec;
//     }

//     let a_sorted = sort(a);
//     let b_sorted = sort(b);

//     return a_sorted.eq(&b_sorted);
// }

// O(n)
fn is_permutation(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    fn count_chars(s: &str) -> HashMap<char, usize> {
        let mut char_counts: HashMap<char, usize> = HashMap::new();
        for char in s.chars() {
            let current_count = char_counts.entry(char).or_insert(0);
            *current_count += 1;
        }

        return char_counts;
    }

    let a_count = count_chars(a);
    let b_count = count_chars(b);

    return a_count.eq(&b_count);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            is_permutation(&String::from("bcda"), &String::from("cdab")),
            true
        );
        assert_eq!(
            is_permutation(&String::from("abcd"), &String::from("abcdd")),
            false
        );
        assert_eq!(
            is_permutation(&String::from("aaaa"), &String::from("aaab")),
            false
        );
    }
}

fn main() {
    is_permutation(&String::from("bcda"), &String::from("cdab"));
}
