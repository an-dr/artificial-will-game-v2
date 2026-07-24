# extensions

One Cargo workspace, one crate per WASM extension (game logic loaded by
[bones](../vendor/bones) at runtime). Sharing a workspace means one
`target/`, so building every extension is one command with a predictable
output directory — no per-extension build step, no copying `.wasm` files
into place:

```sh
cargo build --release --target wasm32-wasip2
```

Output: `target/wasm32-wasip2/release/<name>.wasm` for every member —
exactly what the repo root `bones.toml`'s `extensions_dir` points at.

Adding an extension: create a new crate here (`cdylib`, depends on
`wit-bindgen`, generates against `../vendor/bones/wit`) and add it to this
file's `[workspace] members`. See [hello](hello) for the reference shape.
