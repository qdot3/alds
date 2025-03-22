/// A streaming iterator which enumerates permutations.
#[derive(Debug, Clone)]
pub struct Permutation<T: Ord> {
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

impl<T: Ord> Permutation<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self {
            data,
            state: PermutationState::Entry,
        }
    }

    pub fn next(&mut self) -> Option<&[T]> {
        match self.state {
            PermutationState::Mid => {
                if let Some(i) = self.data.windows(2).rposition(|lr| lr[0] < lr[1]) {
                    let j = self.data.iter().rposition(|v| v > &self.data[i]).unwrap();
                    self.data.swap(i, j);
                    self.data[i + 1..].reverse();

                    Some(&self.data)
                } else {
                    self.state = PermutationState::End;

                    None
                }
            }
            PermutationState::End => None,
            PermutationState::Start | PermutationState::Entry => {
                self.state = PermutationState::Mid;

                Some(&self.data)
            }
        }
    }

    pub fn next_back(&mut self) -> Option<&[T]> {
        match self.state {
            PermutationState::Mid => {
                if let Some(i) = self.data.windows(2).rposition(|lr| lr[0] > lr[1]) {
                    let j = self.data.iter().rposition(|v| v < &self.data[i]).unwrap();
                    self.data.swap(i, j);
                    self.data[i + 1..].reverse();

                    Some(&self.data)
                } else {
                    self.state = PermutationState::Start;

                    None
                }
            }
            PermutationState::Start => None,
            PermutationState::End | PermutationState::Entry => {
                self.state = PermutationState::Mid;

                Some(&self.data)
            }
        }
    }
}

impl<T: Ord> From<Vec<T>> for Permutation<T> {
    fn from(value: Vec<T>) -> Self {
        Self::new(value)
    }
}

impl<T: Ord> FromIterator<T> for Permutation<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::new(Vec::from_iter(iter))
    }
}
