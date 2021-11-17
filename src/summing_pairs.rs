#[allow(dead_code)]
fn summing_pairs(input_list: Vec<usize>) -> Vec<(usize, usize)> {
    /*
    Given an unsorted list of integers on [1,99], write a function that emits
    each pair (drawn without replacement) that sums to 100.

    We approach the problem by looping through the input list, and checking
    each number against cached counts. If we see a match, emit the pair and
    decrement the count. Otherwise, increment the cached count. This way, we
    emit each pair when we see the second element of the pair, while only
    requiring O(n) time for a single loop.

    The more straightforward (or generalizable) approach would be to cache the
    counts in a hashmap rather than the array. However, we can leverage the
    bounded range for the input numbers to store counts in a much smaller
    memory footprint.
    */
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    let mut counts = [0; 99];

    for num in input_list {
        let target = 100 - num;
        let count = counts[target - 1];
        if count > 0 {
            pairs.push((target, num));
            counts[target - 1] -= 1;
        } else {
            counts[num - 1] += 1;
        }
    }

    return pairs;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let source = vec![1, 1, 99, 99];
        let target = vec![(1, 99), (1, 99)];
        assert_eq!(summing_pairs(source), target);
    }

    #[test]
    fn test_2() {
        let source = vec![1, 2, 3, 98, 99];
        let target = vec![(2, 98), (1, 99)];
        assert_eq!(summing_pairs(source), target);
    }

    #[test]
    fn test_3() {
        let source = vec![50, 50, 50, 50];
        let target = vec![(50, 50), (50, 50)];
        assert_eq!(summing_pairs(source), target);
    }
}
