![gihub workflow status](https://github.com/den-taku/test-iced/actions/workflows/on_merge.yml/badge.svg)

# test-iced
Attempt to use Iced

# Getting Started

- [x] In `Cargo.toml`, replace `"rust-template"` with project name and write description.
- [x] Rechoose appropriate LICENSE.
- [x] Modify `Dockerfile`.
- [ ] Set up `develop` branch as the default bransh.
- [ ] Fit `README.md` to your project.

# Notes

- [Iced](https://github.com/iced-rs/iced)
  - クロスコンパイルに対応
  - latest: v0.3 (2022/2/9)
  - Elmの影響を受けている
  - Rustのみで実装
  - 参考記事
    - [RustでGUIプログラミング – Icedでtourを触りつつ日本語表示対応をしてみた](https://dev.classmethod.jp/articles/rust-with-gui-tryon-tour/)
    - [Rust・GUIフレームワークicedでランチャーを作った](https://zenn.dev/kyoheiu/articles/40273bda9d5168)
    - [RustのIcedを使って簡単な画像ビューア](https://zenn.dev/tris/articles/e60efe7c60a770)
    - [Rust GUI crate調査: iced](https://toyamaguchi.hatenablog.com/entry/2020/03/31/233000)

- Architecture
  - State: 状態を管理
  - Message: イベント
  - View logic: Stateに従ってウィジェットなどを表示
  - Update logic: Messageに従って状態を更新
  - 非同期処理
    - Command: 単発的な非同期処理，結果をMessageとして渡す
    - Subscription: 常に待機させておき，定期的にMessageを渡す

- リンクに失敗
- `~/.cargo/config`に以下のように追記
  - ```toml
    [target.x86_64-apple-darwin]
    rustflags = [
      "-C", "link-arg=-undefined",
      "-C", "link-arg=dynamic_lookup",
    ]

    [target.aarch64-apple-darwin]
    rustflags = [
      "-C", "link-arg=-undefined",
      "-C", "link-arg=dynamic_lookup",
    ]
    ```