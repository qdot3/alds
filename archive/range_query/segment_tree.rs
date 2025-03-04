use std::fmt::Debug;

pub struct SegmentTree<T> {
    data: Vec<T>,
    offset: usize,

    op: Box<dyn Fn(&T, &T) -> T>,
    id: Box<dyn Fn() -> T>,
}

impl<T> SegmentTree<T> {
    pub fn from_vec(
        data: Vec<T>,
        op: impl Fn(&T, &T) -> T + 'static,
        id: impl Fn() -> T + 'static,
    ) -> Self {
        let offset = data.len();
        let mut data = Vec::from_iter(std::iter::repeat_with(&id).take(offset).chain(data));
        for i in (1..offset).rev() {
            data[i] = op(&data[i * 2], &data[i * 2 + 1])
        }

        Self {
            data,
            offset,
            op: Box::new(op),
            id: Box::new(id),
        }
    }

    pub fn update(&mut self, i: usize, value: T) {
        let mut i = i + self.offset;
        self.data[i] = value;
        while i > 2 {
            i >>= 1;
            self.data[i] = (self.op)(&self.data[i * 2], &self.data[i * 2 + 1]);
        }
    }

    pub fn query(&self, range: std::ops::Range<usize>) -> T {
        let std::ops::Range { start, end } = range;
        let (mut l, mut r) = (start + self.offset, end + self.offset);

        let (mut res_l, mut res_r) = ((self.id)(), (self.id)());
        while r > l {
            if l & 1 == 1 {
                res_l = (self.op)(&res_l, &self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res_r = (self.op)(&res_r, &self.data[r]);
            }

            l >>= 1;
            r >>= 1;
        }

        (self.op)(&res_l, &res_r)
    }
}

impl<T: Debug> Debug for SegmentTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SegmentTree")
            .field("data", &self.data)
            .field("offset", &self.offset)
            .finish()
    }
}
