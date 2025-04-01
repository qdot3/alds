# I/O

ロックを所得し、適切に[^note-buffer-size]バッファリングを行うことで、標準入出力の高速化が見込めます。

このページの内容は[Library Checker](https://judge.yosupo.jp/)の問題[Many A + B](https://judge.yosupo.jp/problem/many_aplusb)および[Many A + B (128 bit)](https://judge.yosupo.jp/problem/many_aplusb_128bit)で計測しています。

[^note-buffer-size]: `read_to_end()`などで一度にすべての入力を読み込むこともできますが、不要なデータを保持することは速度の低下につながります。これは、Fastestランキングからも見て取れます。適切に設定することで25%の高速化を実現しました。

## 標準入力

### ロック

標準入力のハンドラーである`Stdin`は`io::stdin()`で取得できます。
これの実装を覗いてみると、`Mutex`が使用されていることが分かります。
`Mutex`は可変なグローバル変数のスレッド安全性を実現するために、排他ロックをとります。

```rust
/// 2025-04-01: Copied from <https://doc.rust-lang.org/std/io/fn.stdin.html>
pub fn stdin() -> Stdin {
    static INSTANCE: OnceLock<Mutex<BufReader<StdinRaw>>> = OnceLock::new();
    Stdin {
        inner: INSTANCE.get_or_init(|| {
            Mutex::new(BufReader::with_capacity(stdio::STDIN_BUF_SIZE, stdin_raw()))
        }),
    }
}
```

`Stdin`からデータを読み込むためには毎回排他ロックをとる必要があります。
競技プログラミングではシングルスレッドでの読み込みで十分ですから、初めにロックをとってからそれを保持し続けるのが良いです。
`Stdin.lock()`で`StdinLock`を取得できます。

### バッファリング

競技プログラミングでは、\\( 10^5 \\)個程度の小さなデータを読み込むことがよくあります。
これらを取得するために毎回readシステムコールを呼ぶよりも、ある程度まとめて読み込んだ方が速いです。
これを実装したものが`BufReader`です。
`StdinLock`は`BufReader`のラッパーです。
なお、`MutexGuard`は`Mutex`のロックを保持していることを表現しており、`drop`と同時にアンロックします。

```rust
/// 2025-04-01: Copied from <https://doc.rust-lang.org/std/io/struct.StdinLock.html>
pub struct StdinLock<'a> {
    inner: MutexGuard<'a, BufReader<StdinRaw>>,
}
```

`StdinLock`のバッファーサイズは通常8 KBです。

```rust
/// 2025-04-01: Copied from <https://doc.rust-lang.org/src/std/sys_common/io.rs.html>
/// See also <https://doc.rust-lang.org/src/std/io/buffered/bufreader.rs.html>.
// Bare metal platforms usually have very small amounts of RAM
// (in the order of hundreds of KB)
pub const DEFAULT_BUF_SIZE: usize = if cfg!(target_os = "espidf") { 512 } else { 8 * 1024 };
```

バッファーを活用した振る舞いは`BufRead`トレイトで提供されます。

- `fill_buf()`は未使用のデータを返します。バッファーをすべて消費していた場合は可能な限り補充します。
- `consume()`は使用したデータを`BufReader`に伝えます[^note-consume]。

`fill_buf()`でバッファーにアクセスできるのですが、`fill_buf()`をたくさん呼び出すと遅いです。
**理由は分かりません。**
`DEFAULT_BUF_SIZE`の整数倍のバッファーを別に確保しておき、`fill_buf()`で得たデータを`extend_from_slice()`ですべてコピーすることで20 %程度高速化できました[^note-fill_buf]。

[^note-consume]: 正しく伝えないとバグります。
[^note-fill_buf]: `extend_from_slice()`は内部で`memcpy`を呼び出します。不正確な表現ですが、「`fill_buf()`を1000回呼ぶコストが高速なコピー1回分よりも大きい」のかもしれません。いずれにしても、メモリアクセスの問題だと思います。

## 標準出力

### ロック

標準入力と同様に、`Stdout.lock()`で`StdoutLock`を取得できます。

### バッファリング

`StdoutLock`もバッファリングされていますが、改行`\n`の度にフラッシュされます。
これを回避するためには明示的に`BuWriter`で`StdoutLock`をラップする必要があります。
デフォルトのバッファーサイズは8 KBですが、手動で`capacity`を設定することもできます。

```rust
/// 2025-04-01: Copied from <https://doc.rust-lang.org/std/io/struct.StdoutLock.html>
pub struct StdoutLock<'a> {
    inner: ReentrantLockGuard<'a, RefCell<LineWriter<StdoutRaw>>>,
}
```
