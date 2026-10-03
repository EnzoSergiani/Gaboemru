# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-10-03

### Added

- `CartridgeHeader::parse`: reads title, cartridge type, ROM size and RAM
  size from the header (`0x0134`–`0x0149`), with header checksum
  validation (`0x014D`).
- `CartridgeType` enum covering the full MBC table (MBC1 through MBC7,
  MMM01, HuC1/HuC3, Pocket Camera, Bandai TAMA5), with an `Unknown(Byte)`
  fallback for unrecognised codes.
- `Mbc` trait with a ROM-only implementation (`RomOnly`).
- `Cartridge`: assembles the parsed header with the matching MBC,
  exposes `read`/`write` and a `header()` getter.
- `Bus`: full address decoding — cartridge ROM/RAM, WRAM, echo RAM
  (mirrored to WRAM), OAM, the prohibited range, I/O registers, HRAM and
  the IE register. Ranges without an implemented component yet (VRAM,
  OAM, I/O, IE) are stubbed and logged at `debug` level; the prohibited
  range logs at `error`.
- ROM path as a command-line argument.
- 17 unit tests covering header parsing, the ROM-only MBC, the
  `Cartridge` getter and bus address decoding.

## [0.0.0] - 2026-09-28

### Added

- Project scaffolding with a library/binary split: emulation core in
  `src/lib.rs` and its modules, CLI glue in `src/main.rs`.
- `common::types` module with core type aliases (`Byte`).
- `emulator` module with a minimal `GameBoy` struct.
- Structured logging via `tracing`, written to `logs/emulator.log` and
  overwritten on every run.
- Nix flake providing a development shell (`rustc`, `rust-analyzer`,
  `clippy`, `rustfmt`, `cargo-watch`, `cargo-expand`, `cargo-audit`).
- `rust-version = "1.85.0"` and `edition = "2024"` pinned in `Cargo.toml`.

[Unreleased]: https://github.com/EnzoSergiani/Gaboemru/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/EnzoSergiani/Gaboemru/compare/v0.0.0...v0.1.0
[0.0.0]: https://github.com/EnzoSergiani/Gaboemru/releases/tag/v0.0.0
