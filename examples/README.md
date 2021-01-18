# Examples

## Running examples

You will need:
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Any development local web server (like `python3 -m http.server`)

To Run:
```sh
cd <example-dir>
wasm-pack build --target web --dev
python3 -m http.server
```
