/// Sort interval queries in Hilbert order.
///
/// ## Mo's Algorithm
///
/// See [this](https://codeforces.com/blog/entry/61203).
///
/// ## Example
///
/// ```
/// use alds::range_query::mo_algorithm;
///
/// let queries = vec![(0, 1), (0, 5), (0, 10), (2, 3), (2, 9), (4, 9), (7, 8), (9, 10)];
/// for i in mo_algorithm(&queries) {
///     let (l, r) = queries[i];
///     // do something
/// }
/// ```
pub fn mo_algorithm(queries: &[(usize, usize)]) -> Vec<usize> {
    let mut res = Vec::from_iter(0..queries.len());
    let exp = queries
        .iter()
        .map(|(l, r)| l.max(r))
        .max()
        .unwrap_or(&0)
        .next_power_of_two()
        .ilog2();
    let h_order = Vec::from_iter(queries.iter().map(|&(x, y)| hilbert_order(x, y, exp)));

    res.sort_unstable_by_key(|&i| h_order[i]);
    res
}

/// Calculate Hilbert order.
fn hilbert_order(x: usize, y: usize, exp: u32) -> usize {
    fn _hilbert_order(x: usize, y: usize, exp: u32, dir: Dir) -> usize {
        if exp == 0 {
            return 0;
        }

        let exp = exp - 1;
        let pos = 2 * (x >> exp) + (y >> exp);
        let w = 1 << exp;
        let k = match dir {
            Dir::Up => [2, 1, 3, 0],
            Dir::Down => [0, 3, 1, 2],
            Dir::Left => [2, 3, 1, 0],
            Dir::Right => [0, 1, 3, 2],
        }[pos];
        let (x, y) = (x & (w - 1), y & (w - 1));
        let dir = match dir {
            Dir::Up => [Dir::Up, Dir::Up, Dir::Right, Dir::Left],
            Dir::Down => [Dir::Right, Dir::Left, Dir::Down, Dir::Down],
            Dir::Left => [Dir::Left, Dir::Down, Dir::Left, Dir::Up],
            Dir::Right => [Dir::Down, Dir::Right, Dir::Up, Dir::Right],
        }[pos];

        w * w * k + _hilbert_order(x, y, exp, dir)
    }

    _hilbert_order(x, y, exp, Dir::Down)
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calc_hilbert_order(exp: u32) -> Vec<Vec<usize>> {
        let w = 1 << exp;

        let mut res = Vec::with_capacity(w);
        for x in 0..w {
            let mut row = Vec::with_capacity(w);
            for y in 0..w {
                row.push(hilbert_order(x, y, exp));
            }
            res.push(row);
        }

        res
    }

    #[test]
    fn test_hilbert_order_1() {
        assert_eq!(calc_hilbert_order(1), vec![vec![0, 3], vec![1, 2]])
    }

    #[test]
    fn test_hilbert_order_2() {
        assert_eq!(
            calc_hilbert_order(2),
            vec![
                vec![00, 01, 14, 15],
                vec![03, 02, 13, 12],
                vec![04, 07, 08, 11],
                vec![05, 06, 09, 10]
            ]
        )
    }

    #[test]
    fn test_hilbert_order_3() {
        assert_eq!(
            calc_hilbert_order(3),
            vec![
                vec![00, 03, 04, 05, 58, 59, 60, 63],
                vec![01, 02, 07, 06, 57, 56, 61, 62],
                vec![14, 13, 08, 09, 54, 55, 50, 49],
                vec![15, 12, 11, 10, 53, 52, 51, 48],
                vec![16, 17, 30, 31, 32, 33, 46, 47],
                vec![19, 18, 29, 28, 35, 34, 45, 44],
                vec![20, 23, 24, 27, 36, 39, 40, 43],
                vec![21, 22, 25, 26, 37, 38, 41, 42],
            ]
        )
    }
}
