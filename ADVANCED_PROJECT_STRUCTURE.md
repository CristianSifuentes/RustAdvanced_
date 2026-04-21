# Advanced Rust Scientific Project Structure

This project is designed as an **expert-level Rust 2024 demo** with step-by-step advanced features and production-oriented patterns.

## Tree

```text
.
├── Cargo.toml
├── README.md
├── ADVANCED_PROJECT_STRUCTURE.md
└── src
    ├── main.rs
    └── advanced
        ├── mod.rs
        ├── app.rs
        ├── scientific.rs
        ├── patterns.rs
        ├── async_pipeline.rs
        └── ffi_boundary.rs
```

## Where each advanced Rust concept is implemented

1. **Rust 2024 edition (modern defaults)**  
   - `Cargo.toml` (`edition = "2024"`).

2. **Module architecture and separation of concerns**  
   - `src/advanced/mod.rs` and all submodules.

3. **PhantomData and type-level units (zero-cost safety)**  
   - `Quantity<T, Unit>` + marker units (`Meter`, `Second`) in `src/advanced/scientific.rs`.

4. **Traits + generic numeric behavior**  
   - `Mean<T>` trait and generic impl for slices in `src/advanced/scientific.rs`.

5. **Const generics**  
   - `Vector<const N: usize>` with compile-time sized vector math in `src/advanced/scientific.rs`.

6. **HRTB (Higher-Ranked Trait Bounds)**  
   - `report_with` function (`for<'b> Fn(&'b T) -> String`) in `src/advanced/scientific.rs`.

7. **Declarative macros (`macro_rules!`)**  
   - `telemetry!` macro in `src/advanced/patterns.rs`.

8. **Strong error modeling**  
   - `LabError` enum + `Display` + `Error` in `src/advanced/patterns.rs`.

9. **Trait objects and dynamic dispatch**  
   - `Analyzer` trait + `Box<dyn Analyzer>` usage in `src/advanced/app.rs`.

10. **Concurrency primitives (`Arc<RwLock<_>>`)**  
    - `SharedNotebook` in `src/advanced/patterns.rs`.

11. **Async/await + low-level Future polling**  
    - `run_pipeline` async fn and custom `block_on` executor in `src/advanced/async_pipeline.rs`.

12. **Atomics for cancellation semantics**  
    - `CancelToken` using `AtomicBool` in `src/advanced/async_pipeline.rs`.

13. **Bounded channels for backpressure**  
    - `mpsc::sync_channel(4)` in `src/advanced/async_pipeline.rs`.

14. **Unsafe boundary discipline / FFI-style wrapper**  
    - `NonNullSlice<'a>` with explicit safety invariants in `src/advanced/ffi_boundary.rs`.

15. **Unsafe encapsulation with safe API**  
    - `checked_view` in `src/advanced/ffi_boundary.rs` and usage in `src/advanced/app.rs`.

16. **End-to-end orchestration and explanation flow**  
    - `run_demo` function in `src/advanced/app.rs` (Step 13 to Step 19 comments).

## How to run

```bash
cargo run
cargo test
cargo clippy --all-targets --all-features
```

## Why this is “scientific style”

- Uses deterministic transformations (vector dot, means, checksums).
- Keeps invariants explicit at unsafe boundaries.
- Applies reproducible patterns (clear modules, typed units, bounded channels, explicit errors).
- Demonstrates both correctness-first and performance-aware Rust design.
