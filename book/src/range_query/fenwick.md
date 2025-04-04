# フェニック木

フェニック木は一点更新と区間クエリを対数時間で処理できるデータ構造です。
逆元の存在が要求されますが、演算が可換である必要はありません。

## 構造

![fenwick tree](fenwick.drawio.svg)

フェニック木はセグメント木から、右の子を取り除いた構造をしています。
要素の逆元が存在するとき親と左の子から右の子を復元できるため、セグメント木と同じ操作が可能になります[^seg-ve-fenwick]。

1オリジンのインデックスを採用することで、親を高速に求めることができます[^one-based-indexing]。
`i`番目の頂点の親のインデックスは`i + LSSB(i)`です。

[^seg-ve-fenwick]: セグメント木に比べて、空間計算量が半減し、時間計算量が定数倍悪化します。
[^one-based-indexing]: 0オリジンのインデックスでもエミュレートできますが、インデックス操作を頻繁に行うため、パフォーマンスが大きく悪化する可能性があります。

## 一点更新

![fenwick tree](fenwick_update.drawio.svg)

図の緑の頂点を更新することを考えます。
この頂点は明示的には保存されておらず、\\( a_2^{-1} \circ a_3^{-1} \circ a_4 \\)でアクセスできます。
4番目と8番目の頂点は次式のように更新されます。

\\[\begin{align}
    a_4' &= a_2 \circ a_3 \circ \Big(l \circ (a_2^{-1} \circ a_3^{-1} \circ a_4) \circ r \Big) \\\\
    a_8' &= a_4' \circ (a_4^{-1} \circ a_8)
\end{align}\\]

演算が可換でない場合を意識して、\\(x' = l \circ x \circ r\\)で更新しています[^non-commutative]。
更新処理が本質的に**再帰的**であることがわかります。
再帰を展開するにはスタックが必要ですが、これを構造体のフィールドとしてもっておいて再利用することで、アロケーションコストを削減できます。
計算量は\\( \Theta(\log N) \\)です。

演算が可換である場合には頂点を復元する必要がないので、計算量は\\( O(\log N) \\)に改善し、定数倍も軽くなります。

[^non-commutative]: 行列や直線などを要素としてもつ場合。

## 区間クエリ

![fenwick tree](fenwick_prefix.drawio.svg)

1番目から`i`番目までの頂点の積\\( \mathrm{prefix}(i) := a_1 \circ \cdots \circ a_i \\)をもとめることを考えます。
`i`番目の頂点のサイズが`LSSB(i)`と一致することに注意すると、次のアルゴリズムで求めることができます。

```rust, ignore
// 単位元で初期化
let mut result = T::identity();
while i > 0 {
    // 計算順序に注意！
    result = data[i].bin_op(&result);
    // LSSB(i)を除去
    i &= i.wrapping_sub(1)
}
```

計算量は\\( O(\log N) \\)です。

任意の区間\\((l, r]\\)上の積は次のようにかけます。

\\[\begin{align}
a_{l + 1} \circ \cdots \circ a_r
&= (a_l^{-1} \circ \cdots a_1^{-1}) \circ (a_1 \circ \cdots \circ a_r) \\\\
&= \Big( \mathrm{prefix}(l) \Big)^{-1} \circ \mathrm{prefix}(r)
\end{align}\\]

愚直に求めると計算量は\\( O(\log N) \\)ですが、図に示したように重複した部分を無視することで\\( O\Big(\log (l \oplus r)\Big) \\)に改善できます[^range-query-optimization]。
たとえば、次の通りです[^branch-prediction]。

```rust, ignore
// 共通のプレフィックス**以外**の部分を取り出すためのビットマスク
let mask = !0 >> (l ^ r).leading_zeros();

let mut result_l = T::identity();
while l & mask != 0 {
    result_l = data[l].bin_op(&result_l);
    l &= l.wrapping_sub(1)
}
// 計算順序を守るために別のレジスターに格納する
let mut result_r = T::identity();
while r & mask != 0 {
    result_r = data[r].bin_op(&result_r);
    r &= r.wrapping_sub(1)
}

// 計算順序を守ってまとめる
let result = result_l.inverse().bin_op(&result_r)
```

[^range-query-optimization]: 二項演算\\(\circ\\)が軽い場合には悪化する可能性があります。
[^branch-prediction]: `mask`を直接計算せずに「`l`と`r`のうち小さい方から処理して、両者が一致したら終了する」こともできます。筆者は分岐予測の観点からこれを避けましたが、パイプラインがつまりそうなのであまり意味がないかもしれません。なお、先にインデックスを更新するように書くこともできますが、アセンブリでは無視されていました。

## 構築

愚直には、単位元で初期化した配列に一点更新を\\( N \\)回繰り返すことで\\( \Theta(N \log N) \\)で実現できます。
これは`i`番目の頂点の祖先すべてを更新すること意味しています。
しかし、親がその親を更新することを考えると親だけ更新すれば十分です。
計算量は\\( \Theta(N) \\)です。

```rust, ignore
let mut data = vec![0];
data.extend(initial_values);

// 子から親の順に更新される！
for i in 1..data.len() {
    // LSSB(i)を加える
    let mut p = i + (i & i.wrapping_neg());
    if p < data.len() {
        // 計算順序に注意！
        data[p] = data[p].bin_op(&data[i])
    }
}
```
