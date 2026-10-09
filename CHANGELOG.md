# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-10-09

### Added

- Full CPU instruction set: all 245 valid base opcodes and all 256 `0xCB`-prefixed opcodes
- Fetch/decode/execute loop generic over the `Bus` trait
- SingleStepTests/sm83 integration test runner (500,000 cases, all passing)
- `tracing`-based instruction-level logging (`trace!`/`debug!`/`error!`)

### Fixed

- `STOP` (`0x10`) no longer consumes a non-existent second byte; the opcode is
  effectively 1 byte on real hardware, with a fixed 12-cycle cost, confirmed
  against SingleStepTests/sm83 and cross-referenced with the official
  encoding note (`Cycles: -` in the RGBDS opcode reference)
- `INC HL`/`DEC HL` (register) and `INC (HL)`/`DEC (HL)` (memory) dispatch
  targets were swapped in the opcode table
- `0xCB`/`0xCD` dispatch targets were swapped (prefix vs. `CALL nn`)
- `0x7C` (`LD A,H`) was incorrectly dispatched as `LD A,B`

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
