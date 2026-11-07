# Changelog

All notable changes are listed here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-11-07

### Added

- Rust functions for period, frequency and bpm conversions and for time scaling: `hz_from_period`, `period_from_hz`, `hz_from_period_minutes`, `bpm_from_hz`, `hz_from_bpm`, `scale_frequency`, `scale_period`, `speedup_for` and `ratio`. Every input and result is checked to be finite and positive.
- Python (PyO3) and WebAssembly bindings with the same function names.
- 38 golden vectors, computed exactly with rational arithmetic, checked in all three implementations.
- Lean 4 proofs of 10 claims, including the time-scaling theorem and the invariance of frequency ratios under any speed-up.
