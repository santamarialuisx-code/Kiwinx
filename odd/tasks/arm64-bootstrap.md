# Kiwinx ARM64 bootstrap

Create the Kiwinx repository and its first headless ARM64 vertical slice.
Delivery strategy: ask-on-risk; forecast about 400 authored lines in one work unit.

## Specs

S1. "quiero que empecemos con el proyecto, como lo hacemos?"
S2. "quiero que el nombre y logo esten basados en la fruta kiwi, podria llamarse kiwix o kiwinx, como ryujinx"
S3. "en rust claro, como los proyectos de los que estabamos hablando"
S4. The human selected "Kiwinx (recomendado)" with preview "Carpeta: D:/Proyectos/Kiwinx\nMarca: kiwi + emulación\nPrimer crate ejecutable: kiwinx-cli".
S5. "instala engram entonces, ya que yo solo instale engram pi"

## Tasks

- T1 | S1-S4 | parent | done: created the Kiwinx workspace, feature branch, base crates and kiwi brand direction | commit: pending
- T2 | S1,S3 | parent | done: RED exited 101 only for missing public APIs after fixing the testkit dev-dependency | commit: pending
- T3 | S1,S3 | parent | in progress: implement the minimum memory, CPU, system and kiwinx-cli demo behavior | commit: pending
- T4 | S1,S3 | parent | Run available checks, document exact limitations and commit the work unit | commit: pending
- T5 | S1,S3,S5 | parent | done: installed Engram core 3.7.2 and Rust 1.100.0; verified the Kiwinx-only stable GNU override | commit: pending

## Log

L1. "quiero que empecemos con el proyecto, como lo hacemos? quiero que el nombre y logo esten basados en la fruta kiwi, podria llamarse kiwix o kiwinx, como ryujinx, por donde empezamos? si quieres crea la carpeta en donde va a estar el proyecto"
L2. The human selected Kiwinx and the project location D:/Proyectos/Kiwinx.
L3. Environment evidence: git 2.56.1.windows.1 is available; rustc, cargo and rustup are absent from PATH. Engram is unavailable because its binary is not installed, so the odd/arm64-bootstrap/tasks mirror could not be created.
L4. "instala engram entonces, ya que yo solo instale engram pi"
L5. The human authorized installing Rust stable. The first rustup invocation failed before downloading anything because Bash expanded PowerShell variables; the corrected invocation preserves them with single quotes.
L6. A renamed rustup multicall binary also failed before installation. The successful invocation used the official rustup-init.exe filename in a session-owned temporary directory.
L7. Engram core 3.7.2 and its Pi integration are verified. The Engram mirror is observation 1 under project kiwinx. Rust 1.100.0 is installed; stable-x86_64-pc-windows-gnu is verified as a directory override only for D:/Proyectos/Kiwinx because MSVC build tools are absent.
L8. First cargo test --workspace exited 101 with unresolved public APIs and a missing kiwinx-testkit dev-dependency. The dependency was added to kiwinx-core before repeating RED.
L9. Clean RED: cargo test --workspace exited 101 only because GuestAddress, Memory, MemoryError, System, SystemError and CpuError were not implemented.
