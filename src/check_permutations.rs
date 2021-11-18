use std::collections::hash_map::Entry;
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

#[allow(unused)]
fn check_permutations_2(left: &String, right: &String) -> bool {
    /*
    Write a function to check whether two strings are permutations of each
    other (CTCI 1.2).

    As an alternative, build the count map only of the first word, then count
    down the characters as you read through the second word. This allows early
    stopping, as we can break immediately if we ever see a character in `right`
    that did not occur in `left`, or if we decrement a count below zero. After
    reading both words, the count map is effectively the count of different
    characters between the two words, which we assert is uniformly zero for
    a permutation.
    */
    let mut counts: HashMap<char, i32> = HashMap::new();

    for character in left.chars() {
        let count = counts.entry(character).or_insert(0);
        *count += 1;
    }

    for character in right.chars() {
        let count = counts.entry(character);
        match count {
            Entry::Vacant(count) => return false,
            Entry::Occupied(count) => {
                let inner = count.into_mut();
                if *inner == 0 {
                    return false;
                }
                *inner -= 1;
            }
        }
    }

    for count in counts.values() {
        if *count != 0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match() {
        let left = String::from("abba");
        let right = String::from("baab");
        assert!(check_permutations(&left, &right));
        assert!(check_permutations_2(&left, &right));
        assert!(check_permutations_2(&right, &left));
    }

    #[test]
    fn test_mismatch() {
        let left = String::from("racecar");
        let right = String::from("carcar");
        assert!(!check_permutations(&left, &right));
        assert!(!check_permutations_2(&left, &right));
        assert!(!check_permutations_2(&right, &left));
    }
}
