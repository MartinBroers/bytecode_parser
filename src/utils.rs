pub fn find_sequence<T: PartialEq>(vector: &[T], sequence: &[T]) -> Option<usize> {
    if sequence.len() > vector.len() {
        return None;
    }
    (0..=vector.len() - sequence.len()).find(|&i| vector[i..].starts_with(sequence))
}

#[cfg(test)]
mod tests {
    use crate::utils::find_sequence;

    #[test]
    fn find_correct_sequence() {
        let input = vec![1, 2, 3, 4, 5, 6, 7];
        let sequence = vec![4, 5, 6];

        assert!(find_sequence(&input, &sequence).is_some());
    }

    #[test]
    fn find_sequence_at_start() {
        let input = vec![1, 2, 3, 4, 5, 6, 7];
        let sequence = vec![1, 2, 3];

        assert!(find_sequence(&input, &sequence).is_some());
    }

    #[test]
    fn find_sequence_at_end() {
        let input = vec![1, 2, 3, 4, 5, 6, 7];
        let sequence = vec![5, 6, 7];

        assert!(find_sequence(&input, &sequence).is_some());
    }

    #[test]
    fn sequence_longer_than_input() {
        let input = vec![5, 6, 7];
        let sequence = vec![1, 2, 3, 4, 5, 6, 7];

        assert!(find_sequence(&input, &sequence).is_none());
    }

    #[test]
    fn sequence_not_in_input() {
        let input = vec![1, 2, 3, 4, 5, 6, 7];
        let sequence = vec![3, 5, 7];

        assert!(find_sequence(&input, &sequence).is_none());
    }

    #[test]
    fn sequence_works_with_chars() {
        let input = vec!['h', 'e', 'l', 'l', 'o'];
        let sequence = vec!['l', 'o'];

        assert!(find_sequence(&input, &sequence).is_some());
    }
}
