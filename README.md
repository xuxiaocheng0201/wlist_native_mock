# wlist_flutter_mock


## Mock

1. 安装 Rust: https://www.rust-lang.org/zh-CN/learn/get-started
2. 安装 [flutter_rust_bridge](https://crates.io/crates/flutter_rust_bridge): `cargo install flutter_rust_bridge_codegen`
3. 安装 [cargo-expand](https://crates.io/crates/cargo-expand): `cargo install cargo-expand`
4. 添加项目：`flutter_rust_bridge_codegen integrate`
5. 修改配置文件 `flutter_rust_bridge.yaml`:
```yaml
rust_input: crate::api
rust_root: rust/
dart_output: lib/generated/rust

enable_lifetime: true
```
6. 新建在 `dart_output` 中配置的文件夹
7. 删除 rust 文件夹，使用 submodule: `git submodule add https://github.com/wlist_org/wlist_native_mock rust`
8. 生成代码：`flutter_rust_bridge_codegen generate`
