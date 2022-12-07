# Risc0 WASM Example

This example shows unwanted `wasm-bindgen` artifacts that lands into the WASM file.

### Build WASM file.
```bash
$ cargo build --release --target wasm32-unknown-unknown
```

### Inspect the WASM file.
```bash
$ wasm2wat target/wasm32-unknown-unknown/release/verify.wasm | head -n 30
(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (param i32 i32 i32) (result i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32 i32 i32)))
  (type (;4;) (func (param i32 i64 i32 i32)))
  (type (;5;) (func (param i32 i32 i32 i32)))
  (type (;6;) (func (param i32 i32 i32 i32 i32)))
  (type (;7;) (func (param i32 f32 i32 i32)))
  (type (;8;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;9;) (func (param i32 f64 i32 i32)))
  (type (;10;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (type (;11;) (func (result i32)))
  (type (;12;) (func (param i32)))
  (type (;13;) (func (param i32) (result i32)))
  (type (;14;) (func))
  (type (;15;) (func (param i32) (result i64)))
  (type (;16;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;17;) (func (param i64 i32 i32) (result i32)))
  (type (;18;) (func (param i32 i32 i64 i32 i32)))
  (type (;19;) (func (param i32 i32 i32 i32 i32 i32)))
  (type (;20;) (func (param i32 i32 f32 i32 i32)))
  (type (;21;) (func (param i32 i32 f64 i32 i32)))
  (import "env" "load_data" (func $load_data (type 11)))
  (import "__wbindgen_placeholder__" "__wbindgen_describe" (func $_ZN12wasm_bindgen19__wbindgen_describe17h402830b0880612dcE (type 12)))
  (import "__wbindgen_placeholder__" "__wbindgen_throw" (func $_ZN12wasm_bindgen16__wbindgen_throw17hc7b411560c64c5a5E (type 0)))
  (import "__wbindgen_externref_xform__" "__wbindgen_externref_table_grow" (func $_ZN12wasm_bindgen9externref31__wbindgen_externref_table_grow17h5d8fced3e7917d66E (type 13)))
  (import "__wbindgen_externref_xform__" "__wbindgen_externref_table_set_null" (func $_ZN12wasm_bindgen9externref35__wbindgen_externref_table_set_null17h644009d4f22d60bcE (type 12)))
  (func $verify (type 14)
    call $load_data
```

