// SchwarzDigits fork of libsqlite3-sys: the bundled engine is SQLite3
// Multiple Ciphers (sqlite3mc/PROVENANCE.txt). On wasm32-unknown-unknown the
// bindings come from sqlite-wasm-rs, the same amalgamation compiled for
// wasm (src/wasm.rs); every other target is upstream's crate over the
// amalgamation build.rs compiles (src/native.rs).
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
mod wasm;
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub use wasm::*;

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
mod native;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub use native::*;
