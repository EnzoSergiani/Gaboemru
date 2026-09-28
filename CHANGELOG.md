# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Project scaffolding with a library/binary split: emulation core in `src/lib.rs` and its modules, CLI glue in `src/main.rs`.
- `emulator` module with a minimal `GameBoy` struct.
- Structured logging via `tracing`, written to `logs/emulator.log` and overwritten on every run.
- Nix flake providing a development shell (`rustc`, `rust-analyzer`, `clippy`, `rustfmt`, `cargo-watch`, `cargo-expand`, `cargo-audit`).
- `rust-version = "1.85.0"` and `edition = "2024"` pinned in `Cargo.toml`.

### Planned

See the [Roadmap](README.md#roadmap) for upcoming milestones (M1–M8): cartridge
and bus, CPU, interrupts and timer, PPU, frontend, memory bank controllers, APU,
and cycle-accurate timing.

[Unreleased]: https://github.com/EnzoSergiani/Gaboemru/commits/main
