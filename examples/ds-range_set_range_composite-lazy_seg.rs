use std::cell::RefCell;

use mod_int::SMint;
use proconio::{fastout, input};
use rustc_hash::FxHashMap;
use segment_tree::{LazySegmentTree, Monoid, MonoidAction};

type Mint = SMint<998_244_353>;

thread_local! {
    static MEMO: RefCell<FxHashMap<(Affine, u32), Affine>> = RefCell::new(FxHashMap::default());
}

#[fastout]
fn main() {
    input! { n: usize, q: usize, ab: [(u64, u64); n], }

    let max_exp = n.next_power_of_two().trailing_zeros();
    let mut lst =
        LazySegmentTree::<Composite, Set>::from(Vec::from_iter(ab.into_iter().map(|(a, b)| {
            Composite {
                composed: Affine::new(a, b),
                num: 1,
            }
        })));

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { l: usize, r: usize, c: u64, d: u64, }

            if MEMO.with_borrow(|map| map.len()) > n {
                lst.apply_all();
                MEMO.with_borrow_mut(|map| map.clear());
            }

            let affine = Affine::new(c, d);
            if !MEMO.with_borrow(|map| map.contains_key(&(affine, 1))) {
                let mut pow = affine;
                for exp in 0..=max_exp {
                    MEMO.with_borrow_mut(|map| map.insert((affine, exp), pow));
                    pow = pow.composite(&pow);
                }
            }

            lst.apply(
                l..r,
                Set {
                    affine: Some(affine),
                },
            );
        } else if flag == 1 {
            input! { l: usize, r: usize, x: u64, }

            let res = lst.eval(l..r).composed.apply(&Mint::new(x));
            println!("{}", res)
        } else {
            unreachable!()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Affine {
    tilt: Mint,
    offset: Mint,
}

impl Affine {
    fn new(tilt: u64, offset: u64) -> Self {
        Self {
            tilt: Mint::new(tilt),
            offset: Mint::new(offset),
        }
    }
}

impl MonoidAction<Mint> for Affine {
    const IS_COMMUTATIVE: bool = false;

    fn identity() -> Self {
        Self::new(1, 0)
    }

    fn composite(&self, rhs: &Self) -> Self {
        Self {
            tilt: rhs.tilt * self.tilt,
            offset: rhs.tilt * self.offset + rhs.offset,
        }
    }

    fn apply(&self, arg: &Mint) -> Mint {
        self.tilt * arg + self.offset
    }
}

#[derive(Clone, Copy)]
struct Composite {
    composed: Affine,
    num: u32,
}

impl Monoid for Composite {
    fn identity() -> Self {
        Self {
            composed: Affine::identity(),
            num: 1,
        }
    }

    fn binary_operation(&self, rhs: &Self) -> Self {
        Self {
            composed: self.composed.composite(&rhs.composed),
            num: self.num + rhs.num,
        }
    }
}

#[derive(Clone, Copy)]
struct Set {
    affine: Option<Affine>,
}

impl MonoidAction<Composite> for Set {
    const IS_COMMUTATIVE: bool = false;

    fn identity() -> Self {
        Self { affine: None }
    }

    fn composite(&self, rhs: &Self) -> Self {
        let affine = if let Some(affine) = self.affine {
            Some(affine)
        } else {
            rhs.affine
        };

        Self { affine }
    }

    fn apply(&self, arg: &Composite) -> Composite {
        if let Some(affine) = self.affine {
            let num = arg.num;
            if affine == Affine::identity() {
                return Composite {
                    composed: Affine::identity(),
                    num,
                };
            }

            let exp = num.trailing_zeros();
            MEMO.with_borrow(|map| {
                let composed = *map.get(&(affine, exp)).expect("!!");
                Composite { composed, num }
            })
        } else {
            *arg
        }
    }
}
