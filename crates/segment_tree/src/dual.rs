use std::ops::RangeBounds;

use crate::MapMonoid;

#[derive(Debug, Clone)]
pub struct DualSegmentTree<T, F: MapMonoid<T>> {
    data: Box<[T]>,
    /// one-based indexing buffer of pending maps
    maps: Box<[F]>,
}

impl<T, F: MapMonoid<T>> DualSegmentTree<T, F> {
    /// Returns `[l, r)`.
    fn inner_range<R>(&self, range: R) -> (usize, usize)
    where
        R: RangeBounds<usize>,
    {
        let l = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Included(r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.data.len(),
        };

        (l + self.maps.len(), r + self.maps.len())
    }

    fn shift_down(&mut self, i: usize) {
        let n = self.maps.len();

        if 2 * i < n {
            self.maps[2 * i] = self.maps[i].composite(&self.maps[2 * i]);
        } else {
            self.data[2 * i - n] = self.maps[i].apply(&self.data[2 * i - n]);
        }

        if 2 * i + 1 < n {
            self.maps[2 * i + 1] = self.maps[i].composite(&self.maps[2 * i + 1])
        } else if 2 * i + 1 < n + self.data.len() {
            self.data[2 * i + 1 - n] = self.maps[i].apply(&self.data[2 * i + 1 - n]);
        }

        self.maps[i] = F::identity()
    }

    pub fn apply<R>(&mut self, range: R, map: F)
    where
        R: RangeBounds<usize>,
    {
        let (mut l, mut r) = self.inner_range(range);

        // apply pending maps
        for d in (1..=self.maps.len().trailing_zeros()).rev() {
            self.shift_down(l >> d);
            self.shift_down((r - 1) >> d);
        }

        if l % 2 == 1 {
            self.data[l - self.maps.len()] = map.apply(&self.data[l - self.maps.len()]);
            l += 1
        }
        if r % 2 == 1 {
            r -= 1;
            self.data[r - self.maps.len()] = map.apply(&self.data[r - self.maps.len()]);
        }
        (l, r) = (l / 2, r / 2);

        while l < r {
            if l % 2 == 1 {
                self.maps[l] = map.clone();
                l += 1
            }
            if r % 2 == 1 {
                r -= 1;
                self.maps[r] = map.clone()
            }

            l /= 2;
            r /= 2;
        }
    }

    pub fn get(&mut self, i: usize) -> &T {
        // apply pending maps
        {
            let i = i + self.maps.len();
            for d in (1..=self.maps.len().trailing_zeros()).rev() {
                self.shift_down(i >> d);
            }
        }
        &self.data[i]
    }
}

impl<T, F: MapMonoid<T>> From<Vec<T>> for DualSegmentTree<T, F> {
    fn from(data: Vec<T>) -> Self {
        let data = data.into_boxed_slice();
        let maps = vec![F::identity(); data.len().next_power_of_two()].into_boxed_slice();

        Self { data, maps }
    }
}
