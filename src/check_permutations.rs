use std::collections::HashMap;

#[allow(unused)]
fn check_permutations(left: &String, right: &String) -> bool {
    /*
    Write a function to check whether two strings are permutations of each
    other (CTCI 1.2).

    This is straightforward: build hashmaps of the character counts for each
    string, then compare counts key by key.
    */
    let mut left_counts: HashMap<char, u32> = HashMap::new();
    let mut right_counts: HashMap<char, u32> = HashMap::new();

    for character in left.chars() {
        let count = left_counts.entry(character).or_insert(0);
        *count += 1;
    }

    for character in right.chars() {
        let count = right_counts.entry(character).or_insert(0);
        *count += 1;
    }

    return left_counts == right_counts;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match() {
        let left = String::from("abba");
        let right = String::from("baab");
        assert!(check_permutations(&left, &right));
    }

    #[test]
    fn test_mismatch() {
        let left = String::from("racecar");
        let right = String::from("carcar");
        assert!(!check_permutations(&left, &right));
    }
}
