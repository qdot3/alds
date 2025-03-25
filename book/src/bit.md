# ビット演算

## 最上位ビット（Most Significant Bit, MSB）

### 最上位ビットの位置

高速な（簡潔な）方法はないかも

```rust, ignore
i.ilog2() // panics if i == 0
```

```rust, ignore
u64::BITS - i.leading_zeros() // == 0 if i == 0
```

TODO: O(1)の複雑な（＝定数倍が重い）方法があるらしい

## 最下位の1ビット（Least Significant <span style="color: red;">Set</span> Bit, LS<span style="color: red;">S</span>B）

### 最下位の1ビットの位置

```rust, ignore
i.trailing_zeros()
```

### 最下位の1ビットそのもの

```rust, ignore
i & i.wrapping_neg()
```

### 最下位の1ビットを減じる

```rust, ignore
i &= i.wrapping_sub(1)
```

## 組み合わせ

### 組み合わせの全列挙

```rust, ignore
for mut member in 0..1 << n {
    while member > 0 {
        let i = member.trailing_zeros() as usize;
        member ^= 1 << i;

        todo!("i番目のメンバーに対して何かする")
    }
}
```

計算量解析：
\\[\sum_{k=0}^n k {n \choose k} = n \sum_{k=1}^n {n-1\choose k-1} = n 2^{n-1}\\]

## 付録

フェニック木において、`trailing_zeros()`を適切なビット演算に置き換えることで実行時間が200 msから120 msになった。GitHub ActionsのLinux環境で1回ずつしか計測していないので、参考程度にとどめておくこと。

`v1.85.1`時点では、`ilog2()`や`leading_zeros()`の類は内部で`intrinsics::ctlz`や`intrinsics::cttz`を呼び出している。これはハードウェア命令があればそれを用いるが、なければ二分探索などのソフトウェア実装を用いることを意味する。とくに後者の場合、\\( O(\log \mathrm{BITS})\\)のコストがかかるため遅くなる。
