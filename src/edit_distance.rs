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
        match (lc, rc) {
            (None, _) | (_, None) => return true, // reached end of either string
            (Some(l), Some(r)) => {
                lc = left_chars.next();
                rc = right_chars.next();

                match (l, r, no_edits) {
                    (l, r, _) if l == r => continue,
                    (_, _, false) => return false, // seeing second edit
                    (l, r, true) => {
                        no_edits = false;
                        match (lc, rc) {
                            (None, _) | (_, None) => continue,
                            // for insertion/deletion, check next char and move past pair
                            (Some(nl), Some(nr)) => {
                                if l == nr {
                                    rc = right_chars.next();
                                } else if r == nl {
                                    lc = left_chars.next();
                                }
                            }
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
