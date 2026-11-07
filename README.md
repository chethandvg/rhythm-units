# Rhythm units

**Period, frequency and bpm conversions, with Lean 4 proofs that speeding time up scales every frequency by the same factor and keeps every ratio.**

[![CI](https://github.com/chethandvg/rhythm-units/actions/workflows/ci.yml/badge.svg)](https://github.com/chethandvg/rhythm-units/actions/workflows/ci.yml)
[![Proofs](https://github.com/chethandvg/rhythm-units/actions/workflows/proofs.yml/badge.svg)](https://github.com/chethandvg/rhythm-units/actions/workflows/proofs.yml)
[![Licence](https://img.shields.io/badge/licence-Apache--2.0-blue)](LICENSE)

Part of **[Frequencies of Life](https://chethandvg.github.io/frequencies-of-life/)**, open code for the rhythms of life in which every claim says how it is known: proved, verified, tested, referenced or assumed.

## What is proved

<!-- claims-table:start -->
| Claim | Level | Statement | Evidence |
|---|---|---|---|
| `rhythm:FREQ-PERIOD-INVERSE` | P | Frequency and period are inverses: converting a period to a frequency and back returns the original period. | [`RhythmUnits.period_hz_roundtrip`](proofs/RhythmUnits/Basic.lean) |
| `rhythm:FREQ-POSITIVE` | P | A positive period always has a positive frequency. | [`RhythmUnits.hzFromPeriod_pos`](proofs/RhythmUnits/Basic.lean) |
| `rhythm:BPM-HZ-INVERSE` | P | Beats per minute and hertz convert into each other without loss (bpm = 60 × f). | [`RhythmUnits.bpm_hz_roundtrip`](proofs/RhythmUnits/Basic.lean) |
| `rhythm:MINUTES-AS-SECONDS` | P | A period of m minutes is the same rhythm as a period of 60 × m seconds. | [`RhythmUnits.minutes_as_seconds`](proofs/RhythmUnits/Basic.lean) |
| `rhythm:TIME-SCALING-PERIODIC` | P | If a signal repeats every T, the same signal sped up by a factor k repeats every T / k. | [`RhythmUnits.time_scaling_periodic`](proofs/RhythmUnits/Basic.lean) |
| `rhythm:TIME-SCALING-FREQUENCY` | P | Speeding time up by a factor k multiplies every frequency by k. | [`RhythmUnits.time_scaling_frequency`](proofs/RhythmUnits/Basic.lean) |
| `rhythm:SCALE-RATIO-INVARIANT` | P | Speeding time up by any non-zero factor leaves the ratio between any two frequencies unchanged. | [`RhythmUnits.scale_ratio_invariant`](proofs/RhythmUnits/Basic.lean) |
| `rhythm:SPEEDUP-FOR` | P | For any non-zero starting frequency, speedup_for gives exactly the factor that turns it into the target frequency. | [`RhythmUnits.speedupFor_spec`](proofs/RhythmUnits/Basic.lean) |
| `rhythm:TEN-MINUTES-HZ` | P | A rhythm with a 10-minute period has a frequency of exactly 1/600 Hz (about 1.67 mHz). | [`RhythmUnits.ten_minutes_hz`](proofs/RhythmUnits/Basic.lean) |
| `rhythm:TEN-MINUTES-SPED-UP-180` | P | Sped up 180 times, a 10-minute rhythm has a frequency of exactly 0.3 Hz: one beat every 3⅓ seconds. | [`RhythmUnits.ten_minutes_sped_up_180`](proofs/RhythmUnits/Basic.lean) |
| `rhythm:IMPLEMENTATIONS-AGREE` | T | The Rust crate, the Python bindings and the WebAssembly bindings all match the exactly computed golden vectors to within a few units in the last place. | [`vectors/conversions.json`](vectors/conversions.json), [`tests/vectors.rs`](tests/vectors.rs), [`bindings/python/tests/test_vectors.py`](bindings/python/tests/test_vectors.py), [`bindings/wasm/tests/vectors.test.mjs`](bindings/wasm/tests/vectors.test.mjs) |
| `rhythm:INPUT-VALIDATION` | T | All three implementations reject inputs that are zero, negative or not finite, and results that would overflow. | [`vectors/conversions.json`](vectors/conversions.json), [`src/lib.rs`](src/lib.rs) |
<!-- claims-table:end -->

Levels: **P** machine-checked proof (Lean 4) · **V** verified numerics · **T** tested · **R** referenced (DOI or PMID) · **M** model assumption.

## Use

**Rust**

```toml
[dependencies]
rhythm-units = { git = "https://github.com/chethandvg/rhythm-units", tag = "v0.1.0" }
```

**Python** (builds the Rust core with maturin)

```bash
pip install "git+https://github.com/chethandvg/rhythm-units@v0.1.0#subdirectory=bindings/python"
```

**JavaScript / WebAssembly**: see [bindings/wasm](bindings/wasm).

## How it is verified

- **Proofs:** the Lean 4 files in [`proofs/`](proofs) prove the P-level claims about the mathematical functions. CI rebuilds them with Mathlib, rejects `sorry`, and allows only the standard axioms (`propext`, `Classical.choice`, `Quot.sound`).
- **Implementation:** one Rust implementation, compiled to Python (PyO3) and WebAssembly. The same golden vectors in [`vectors/`](vectors) are checked in all three, on Linux and Windows.
- **What the proofs don't cover:** they are about exact real-number mathematics. The Rust code uses 64-bit floating point, so its rounding is checked by the golden-vector tolerances, not by the proofs.

## Cite

If you use this work, please cite it. GitHub's **Cite this repository** button gives APA and BibTeX from [CITATION.cff](CITATION.cff).

## Licence

Code: Apache-2.0 ([LICENSE](LICENSE)). Copyright notices are in [NOTICE](NOTICE).
