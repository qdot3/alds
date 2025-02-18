/// # Example
///
/// * Iterates all permutations from given one in ascending/descending order.
///
/// ```rust
/// use alds::math::Permutation;
///
/// let mut seq = Permutation::new(vec![2, 1, 3]);
///
/// assert_eq!(seq.next(), Some(vec![2, 1, 3]));
/// assert_eq!(seq.next(), Some(vec![2, 3, 1]));
/// assert_eq!(seq.next(), Some(vec![3, 1, 2]));
/// assert_eq!(seq.next(), Some(vec![3, 2, 1]));
/// assert_eq!(seq.next(), None);
///
/// assert_eq!(seq.next_back(), Some(vec![3, 2, 1]));
/// assert_eq!(seq.next_back(), Some(vec![3, 1, 2]));
/// assert_eq!(seq.next_back(), Some(vec![2, 3, 1]));
/// assert_eq!(seq.next_back(), Some(vec![2, 1, 3]));
/// assert_eq!(seq.next_back(), Some(vec![1, 3, 2]));
/// assert_eq!(seq.next_back(), Some(vec![1, 2, 3]));
/// assert_eq!(seq.next_back(), None);
/// ```
///
/// * The same elements are not distinguished.
///
/// ```rust
/// use alds::math::Permutation;
///
/// let mut seq = Permutation::new(vec![2, 2, 3]);
///
/// assert_eq!(seq.next(), Some(vec![2, 2, 3]));
/// assert_eq!(seq.next(), Some(vec![2, 3, 2]));
/// assert_eq!(seq.next(), Some(vec![3, 2, 2]));
/// assert_eq!(seq.next(), None);
/// ```
#[derive(Debug, Clone)]
pub struct Permutation<T: Ord + Clone> {
    data: Vec<T>,
    state: PermutationState,
}

#[derive(Debug, Clone)]
enum PermutationState {
    Start,
    Mid,
    End,
    Entry, // use only once at the beginning
}

impl<T: Ord + Clone> Permutation<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self {
            data,
            state: PermutationState::Entry,
        }
    }
}

impl<T: Ord + Clone> From<Vec<T>> for Permutation<T> {
    fn from(data: Vec<T>) -> Self {
        Self::new(data)
    }
}

impl<T: Ord + Clone, const N: usize> From<[T; N]> for Permutation<T> {
    fn from(data: [T; N]) -> Self {
        Self::new(data.to_vec())
    }
}

impl<T: Ord + Clone> FromIterator<T> for Permutation<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        Self::new(Vec::from_iter(iter))
    }
}

impl<T: Ord + Clone> Iterator for Permutation<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            PermutationState::Mid => {
                if let Some(i) = self.data.windows(2).rposition(|lr| lr[0] < lr[1]) {
                    let j = self.data.iter().rposition(|v| v > &self.data[i]).unwrap();
                    self.data.swap(i, j);
                    self.data[i + 1..].reverse();

                    Some(self.data.clone())
                } else {
                    self.state = PermutationState::End;

                    None
                }
            }
            PermutationState::End => None,
            PermutationState::Start | PermutationState::Entry => {
                self.state = PermutationState::Mid;

                Some(self.data.clone())
            }
        }
    }
}

impl<T: Ord + Clone> DoubleEndedIterator for Permutation<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.state {
            PermutationState::Mid => {
                if let Some(i) = self.data.windows(2).rposition(|lr| lr[0] > lr[1]) {
                    let j = self.data.iter().rposition(|v| v < &self.data[i]).unwrap();
                    self.data.swap(i, j);
                    self.data[i + 1..].reverse();

                    Some(self.data.clone())
                } else {
                    self.state = PermutationState::Start;

                    None
                }
            }
            PermutationState::Start => None,
            PermutationState::End | PermutationState::Entry => {
                self.state = PermutationState::Mid;

                Some(self.data.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_element() {
        let mut seq = Permutation::new(vec![1]);

        assert_eq!(seq.next(), Some(vec![1]));
        assert_eq!(seq.next(), None);
        assert_eq!(seq.next_back(), Some(vec![1]));
        assert_eq!(seq.next_back(), None);
    }

    #[test]
    fn count_unique() {
        let seq = Permutation::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let ptn = (1..=9).product();
        assert_eq!(seq.count(), ptn);

        let seq = Permutation::new(vec![1, 1, 1, 1, 2, 2, 2, 3, 3]);
        assert_eq!(seq.count(), ptn / (24 * 6 * 2))
    }

    #[test]
    fn start_from_middle_forward() {
        let mut seq = Permutation::new(vec![2, 3, 1]);

        assert_eq!(seq.next(), Some(vec![2, 3, 1]));
        assert_eq!(seq.next(), Some(vec![3, 1, 2]));
        assert_eq!(seq.next(), Some(vec![3, 2, 1]));
        assert_eq!(seq.next(), None);
    }

    #[test]
    fn start_from_middle_backward() {
        let mut seq = Permutation::new(vec![2, 1, 3]);

        assert_eq!(seq.next_back(), Some(vec![2, 1, 3]));
        assert_eq!(seq.next_back(), Some(vec![1, 3, 2]));
        assert_eq!(seq.next_back(), Some(vec![1, 2, 3]));
        assert_eq!(seq.next_back(), None);
    }
}
