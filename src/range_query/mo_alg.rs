use itertools::Itertools;

/// Sort queries in Hilbert order.
///
/// `n` is the number of data.
pub fn mo_algorithm(queries: &Vec<(usize, usize)>, n: usize) -> impl IntoIterator<Item = usize> {
    queries
        .iter()
        .map(|&(l, r)| hilbert_order(l, r, n.next_power_of_two().ilog2(), Dir::DOWN))
        .enumerate()
        .sorted_by_key(|(_, ho)| *ho)
        .map(|(i, _)| i)
}

fn hilbert_order(x: usize, y: usize, exp: u32, dir: Dir) -> usize {
    if exp == 0 {
        return 0;
    }

    let exp = exp - 1;
    let pos = 2 * (x >> exp) + (y >> exp);
    let w = 1 << exp;
    let k = match dir {
        Dir::UP => [2, 1, 3, 0],
        Dir::DOWN => [0, 3, 1, 2],
        Dir::LEFT => [2, 3, 1, 0],
        Dir::RIGHT => [0, 1, 3, 2],
    }[pos];
    let (x, y) = (x & (w - 1), y & (w - 1));
    let dir = match dir {
        Dir::UP => [Dir::UP, Dir::UP, Dir::RIGHT, Dir::LEFT],
        Dir::DOWN => [Dir::RIGHT, Dir::LEFT, Dir::DOWN, Dir::DOWN],
        Dir::LEFT => [Dir::LEFT, Dir::DOWN, Dir::LEFT, Dir::UP],
        Dir::RIGHT => [Dir::DOWN, Dir::RIGHT, Dir::UP, Dir::RIGHT],
    }[pos];

    w * w * k + hilbert_order(x, y, exp, dir)
}

#[derive(Clone, Copy)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
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
                row.push(hilbert_order(x, y, exp, Dir::DOWN));
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
