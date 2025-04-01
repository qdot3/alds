# alds library

[![Actions Status](https://github.com/qdot3/alds/workflows/verify/badge.svg)](https://github.com/qdot3/alds/actions)

競技プログラミング用のコードを実装・検証しています。
まだまだ発展途上ですから、ファイル構造が変わる場合があります。

- `/crates/*/src`：各種アルゴリズム・データ構造の実装があります。
- `/crates/*/examples`：検証用コードがあります。使用例としても使えます。
- `/book`：ライブラリ作成時に得た知見を`mdbook`でまとめていきたい。
- `/archive`：ゴミ箱。お宝が眠っているかも？

## Policy

Rustは速くて安全な言語ですから、なるべく`safe`なコードで書きます。
また、バリーデーションを積極的に行います。
このため、十分に高速化されていないことがあります。

安全性を保障できる場合に限り、`unsafe`なコードをかくことがあります。
つまり、安全なインターフェイスを提供します。

## Competitive Programming

`Cargo.toml`の`[dependencies]`に下記のスニペットをコピペする。

```text
TODO (workspace.membersを正規表現でいい感じに加工して下さい)
```

[`cargo-equip`](https://github.com/qryxip/cargo-equip)でバンドルして提出する。

## License

Choose MIT or Apache-2.0 at your opinion.
