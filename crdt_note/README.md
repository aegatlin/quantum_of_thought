# crdt_note

## The Rust-to-Wasm-to-JS/TS pipeline

### With wasm-pack

Install the tool

```sh
cargo install wasm-pack
```

This creates a `crdt_note/pkg/` dir with a `package.json` and all the various wasm, js, and ts files necessary to import the package into `client_web`.

```sh
cd crdt_note
wasm-pack build .
```

### Without wasm-pack (incomplete)

The [Rust and WebAssembly Working Group](https://blog.rust-lang.org/inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org/) was recently sunsetted, so the future of `wasm-pack` is unclear. Also unclear is if the steps below actually work.

This command adds the wasm compilation target, which is not enabled by default.

```sh
rustup target add wasm32-unknown-unknown
```

This command builds the wasm itself, now in `/target/wasm32-unknown-unknown/`.

```sh
cargo build --target wasm32-unknown-unknown --release
```

This command installs the wasm-bindgen cli.

https://github.com/wasm-bindgen/wasm-bindgen

```sh
cargo install wasm-bindgen-cli
```

This command creates the `pkg/` dir with the wasm and the associated JS and TS code.

```sh
wasm-bindgen --out-dir ./pkg ./target/wasm32-unknown-unknown/release/crdt_note.wasm
```

Now in `client_web/package.json` you can include the dep.

```json
{
  "dependencies": {
    "crdt_note": "file:../crdt_note/pkg"
  }
}
```

Running `npm install` will then bundle it in the app.
