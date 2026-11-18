# Kiwinx

Kiwinx is an experimental, headless-first Nintendo Switch emulator written in Rust.
The project starts with a small, testable ARM64 interpreter; it does not currently run homebrew or commercial games.

## Brand

The name combines **kiwi** with the **NX** codename associated with Nintendo Switch.
The visual identity should use an original kiwi-fruit mark: kiwi green, seed black and skin brown.
The final logo is a separate design deliverable; no Nintendo or Ryujinx artwork is part of this project.

## Workspace

- `kiwinx-common`: guest address types shared by all subsystems.
- `kiwinx-memory`: bounded guest memory.
- `kiwinx-cpu`: ARM64 CPU state, decoder and interpreter.
- `kiwinx-core`: headless system composition and execution.
- `kiwinx-testkit`: synthetic guest programs used by tests and demos.
- `kiwinx-cli`: command-line entry point.

## Requirements

Install Rust with [rustup](https://rustup.rs/), then use the stable toolchain.
No C or C++ compiler is required for this initial slice.

On Windows, the default MSVC target needs the Visual Studio C++ Build Tools to link.
When they are absent, select the GNU target for this directory only:

```sh
rustup toolchain install stable-x86_64-pc-windows-gnu --profile minimal
rustup override set stable-x86_64-pc-windows-gnu   # run inside the repository root
```

The override is written to `rust-toolchain` and applies only to this checkout.

## Run

```sh
cargo run -p kiwinx-cli -- demo
cargo test --workspace
```

The demo executes a synthetic three-instruction ARM64 program and prints its guest exit code.

## Scope

The first slice implements only 64-bit `MOVZ`, 64-bit `ADD (immediate)` and `SVC #0` as a test exit.
It is not yet a Switch loader, kernel or hardware emulator. Future work must be validated with synthetic programs and differential tests before adding a JIT, Vulkan or a desktop UI.
