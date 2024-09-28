// https://leetcode.com/problems/longest-substring-without-repeating-characters

mod longest_substring_without_repeating_characters {
    use std::collections::HashSet;

    #[allow(unused)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        // Letters in the currently considered substring.
        let mut letters: HashSet<char> = HashSet::new();
        let mut substring_start: usize = 0;
        let mut substring_end: usize = 0;
        let mut max_length: usize = 0;
        for c in s.chars() {
            substring_end += 1;
            if letters.contains(&c) {
                loop {
                    let start_letter: char = s.as_bytes()[substring_start] as char;
                    substring_start += 1;
                    if start_letter == c {
                        break;
                    }
                    letters.remove(&start_letter);
                }
            } else {
                letters.insert(c);
                max_length = max_length.max(substring_end - substring_start);
            }
        }
        max_length as i32
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
        }

        #[test]
        fn test_2() {
            assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
        }

        #[test]
        fn test_3() {
            assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
        }
    }
}
