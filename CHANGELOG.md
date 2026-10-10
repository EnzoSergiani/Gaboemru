# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-10-10

### Added

- `IE`/`IF` registers with real storage, full interrupt dispatch
  (`GameBoy::handle_interrupts`), priority ordering, and `HALT` wake-up
  independent of `IME`
- `Timer` peripheral (`DIV`/`TIMA`/`TMA`/`TAC`), modeled as a single 16-bit
  counter with falling-edge detection on the selected bit, matching real
  hardware behavior rather than an independent countdown
- `Serial` peripheral (`SB`/`SC`), internal-clock transfers complete and
  request an interrupt; external-clock transfers remain pending indefinitely
  (no link cable emulation)
- The HALT bug: `HALT` executed with `IME=false` and a pending interrupt no
  longer actually halts the CPU; the following byte is read twice, matching
  documented hardware behavior
- `Cpu` wired into `GameBoy`, with a working `run()` loop
- Headless Mooneye ROM runner (`LD B,B` success protocol, cycle timeout)

### Changed

- M3's exit criterion no longer requires passing Mooneye ROMs directly:
  validation against real ROMs is deferred to M4, since Mooneye's shared
  test harness depends on PPU/VBlank synchronization that doesn't exist yet

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
