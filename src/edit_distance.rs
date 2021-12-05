#[allow(dead_code)]
fn check_edit_distance(left: &str, right: &str) -> bool {
    /*
    Given two words, check that the words are at most one edit away from each
    other (CTCI 1.5). An edit is defined as:

    (1) a removed character
    (2) an added character
    (3) a substituted character
    */

    // lengths differing by >1 requires multiple insertions/deletions
    if (left.len() as i32 - right.len() as i32).abs() > 1 {
        return false;
    }

    let mut left_chars = left.chars();
    let mut right_chars = right.chars();

    let mut no_edits = true;
    let mut lc = left_chars.next();
    let mut rc = right_chars.next();

    loop {
        match (lc, rc, no_edits) {
            (None, _, _) | (_, None, _) => return true, // reached end of either string
            (Some(l), Some(r), _) if l == r => {
                // matching char
                lc = left_chars.next();
                rc = right_chars.next();
            }
            (Some(_), Some(_), false) => return false, // already seen an edit
            (Some(l), Some(r), true) => {
                // insert/delete/substitute
                no_edits = false;

                let next_l = left_chars.next();
                let next_r = right_chars.next();
                match (next_l, next_r) {
                    (None, _) | (_, None) => continue,
                    (Some(next_l), Some(next_r)) => {
                        if (l == next_r) | (r == next_l) {
                            lc = left_chars.next();
                            rc = right_chars.next();
                        } else {
                            lc = Some(next_l);
                            rc = Some(next_r);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_string() {
        assert!(check_edit_distance("pale", "pale"))
    }

    #[test]
    fn test_deletion() {
        assert!(check_edit_distance("pale", "ple"))
    }

    #[test]
    fn test_addition() {
        assert!(check_edit_distance("pales", "pale"))
    }

    #[test]
    fn test_insertion() {
        assert!(check_edit_distance("pale", "peale"))
    }

    #[test]
    fn test_substitution() {
        assert!(check_edit_distance("pale", "bale"))
    }

    #[test]
    fn test_end_substitution() {
        assert!(check_edit_distance("pale", "palk"))
    }

    #[test]
    fn test_double_substitution() {
        assert!(!check_edit_distance("pale", "bake"))
    }

    #[test]
    fn test_double_addition() {
        assert!(!check_edit_distance("pale", "paless"))
    }
}
