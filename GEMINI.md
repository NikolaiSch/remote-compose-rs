# Gemini Agent Context

## Project Purpose
This project is a **Rust port of AndroidX Remote Compose**. 
The primary focus is on **loading and playing** Remote Compose documents. It aims to provide a native Rust implementation compatible with the AndroidX Remote Compose format, allowing for the parsing, execution, and rendering of remote UI descriptions.

## Using the nix flake
The nix flake exports a number of useful targets for each crate:

- `check-$crate` - A target for running `cargo check` (no executable is built)
- `test-$crate` - A target for running `cargo test` (no executable is built)
- `clippy-$crate` - A target for running `cargo clippy` (no executable is built)
- `example-$crate-$example` - A target for running `cargo run --example` (an executable is built)

For the root crate, there is also:
- `devShell` - A target for a dev shell, which you can use with `nix develop`

we will be using this instead of cargo, when possible 

## Finding Code References
To understand the original implementation and protocol details, use **Android Code Search**:
- **URL**: [cs.android.com](https://cs.android.com) or [github.com/androidx/androidx](https://github.com/androidx/androidx)
- **Search Scope**: `androidx/compose/remote`
- **Key Files**: Look for `Operations.java`, `RemoteComposeCanvas.java`, and related classes in the `androidx.compose.remote` package to verify opcodes, binary formats, and behavior. The module remote-core, is most relevant for this project.

Example cs.android.com search for "Banana": https://cs.android.com/search?q=Banana&ss=androidx%2Fplatform%2Fframeworks%2Fsupport:compose%2Fremote%2F

Example github search for "Banana": https://github.com/search?q=repo%3Aandroidx%2Fandroidx+path%3A%2F%5Ecompose%5C%2Fremote%5C%2F%2F+Banana&type=code

Use searches of either cs.android.com directly, or the github AndroidX project https://github.com/androidx/androidx, rather than searching the web. Searching the web will yield mostly irrelevant results.

## Project Structure
The project is organized as a Cargo workspace with the following crates:

- **`crates/core`**: The core library containing the main logic for parsing and representing Remote Compose documents.
    - **`operations/`**: Contains the definitions and binary parsing logic for various Remote Compose operations.
        - **`mod.rs`**: Defines the `OpCode` and `Operations` enums, and the top-level document structure.
        - **`canvas.rs`**: Contains drawing commands (e.g., `DrawLine`, `DrawRect`) and coordinate transformations (e.g., `MatrixTranslate`). These operations typically draw onto the current canvas state using the accumulated paint settings.
        - **`paint.rs`**: Defines `PaintValues` and `PaintChange`, which manage the styling state of the canvas (colors, stroke widths, gradients, etc.). Paint state is usually configured before drawing operations are called.
        - **`primitives.rs`**: Basic building blocks like `Header` and `ComponentStart`.
        - **`state.rs`**: Operations related to component values and state management.
- **`crates/expressions`**: functionality for parsing and evaluating dynamic expressions (e.g., variables, arithmetic) used within the Remote Compose document. These allow properties like coordinates or colors to be computed at runtime.

## Testing Strategy
We use a two-tiered testing approach to ensure the correctness of our Remote Compose implementation:

### 1. Sample Integration Tests
Located in `crates/core/tests/androidx_samples.rs`, these tests use complete, real-world binary dumps from AndroidX to verify the end-to-end parsing of documents.

### 2. Granular Operation Tests
Individual operations (e.g., `DrawRect`, `DrawLine`, `PaintValues`) are tested in isolation using hex sequences extracted from the integration samples.
**Workflow for creating new operation tests:**
1.  Temporarily add debug logging to `Operations::read` in `mod.rs` to output the hex string of successfully parsed operations.
2.  Run the integration tests to capture the desired operation's hex data.
3.  Create a new test file in `crates/core/tests/` (e.g., `draw_line_test.rs`).
4.  Use `hex_to_bytes` to convert the captured hex string into a byte array.
5.  Call `Operations::read` and assert the parsed operation's properties match expectations.
6.  Document the source of the sample data in the test comments.
