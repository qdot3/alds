# ビット演算

符号なしの整数型を前提とします。

## Least Significant <span style="color: red;">Set</span> Bit (LS<span style="color: red;">S</span>B）

LSSBは立っているビットの内最小のものを指します。
最下位ビット（LSB）ではありません。

### LSSBの位置

```rust, ignore
i.trailing_zeros() // == <_>::BITS if i == 0
```

### LSSBそのもの

```rust, ignore
i & i.wrapping_neg() // == 0 if i == 0
```

原理：

```rust, ignore
0b1010_0100.wrapping_neg() == 0b0101_1011 + 1 == 0b0101_1100 // non-zero case
```

### LSSBを減じる

```rust, ignore
i &= i.wrapping_sub(1) // == 0 if i == 0
```

原理：

```rust, ignore
0b1010_1000.wrapping_sub(1) == 0b1010_0111 // non-zero case
```

## Most Significant <span style="color: red;">Set</span> Bit (MS<span style="color: red;">S</span>B）

MSSBは立っているビットの内最大のものを指します。
最上位ビット（MSB）ではありません。

### MSSBの位置

```rust, ignore
i.ilog2() // panics if i == 0
```

```rust, ignore
<_>::BITS - i.leading_zeros() // == 0 if i == 0
```

### MSSBそのもの

`ilog2()`や`trailing_zeros()`はハードウェア実装を呼び出せれば\\( O (1)\\)です。
ソフトウェア実装の場合は、二分探索で\\( \Theta(\mathrm{BITS})\\)で求めることができます。

直接求める方法があればよいのですが。うーん。

```rust, ignore
1 << i.ilog2() // panics if i == 0
```

```rust, ignore
1.wrapping_shr(i.leading_zeros() + 1) // meaningless if i == 0
```

## 組み合わせ

組み合わせを高速に求めることができます。
計算量は愚直な実装の半分以下です。

### 組み合わせの列挙

```rust, ignore
// {0}, {1}, {0, 1}, .., {n}, .., {0, .., n}の順に走査する
for mut member in 0..1 << n {
    while member > 0 {
        let i = member.trailing_zeros() as usize;
        member ^= 1 << i;

        todo!("i番目の要素に対して何かする")
    }
}
```

計算量解析：
\\[\sum_{k=0}^n k {n \choose k} = n \sum_{k=1}^n {n-1\choose k-1} = n 2^{n-1}\\]

### 組み合わせの列挙2

とびとびのインデックスで指定される要素の組み合わせをすべて求めることもできます。
要素数を\\( k \\)とすると、計算量は\\( \Theta(k2^{k-1}) \\)です。

```rust, ignore
let set = 0b0101_1000; // 3, 4, 6番目の要素の組み合わせを考える（k = 3）
let mut memo = set;
while memo > 0 {
    // {3, 4, 6}, {4, 6}, {3, 6}, {6}, {3, 4}, {4}, {3}の順に走査する
    // {}は個別に扱う必要がある
    let mut sub_set = set & memo;
    // `sub_set`のLSSBを削除し、それよりインデックスが小さな要素を追加する
    memo = set & sub_set.wrapping_sub(1);
    while sub_set > 0 {
        let i = sub_set.trailing_zeros();
        sub_set ^= 1 << i;

        todo!("i番目の要素に対して何かする")
    }
}
```

## 付録

フェニック木において、`trailing_zeros()`を適切なビット演算に置き換えることで実行時間が200 msから120 msになりました。
GitHub ActionsのLinux環境で1回ずつしか計測していませんが、無視できない差です。
