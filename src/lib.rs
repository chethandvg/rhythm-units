// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama

//! # Rhythm units
//!
//! Every rhythm of life is a frequency: a sperm tail beating 10–19 times a
//! second, a calcium spike in a fertilised egg every 10–35 minutes, a heartbeat.
//! This crate converts between periods, frequencies and beats per minute, and
//! describes what happens when time is sped up by a factor `k`: every frequency
//! is multiplied by `k`, so every ratio between two frequencies stays the same.
//! The Lean proofs in `proofs/` establish these facts for the underlying
//! mathematics; `claims.yaml` lists each one.
//!
//! Part of [Frequencies of Life](https://chethandvg.github.io/frequencies-of-life/).
//!
//! Every input must be finite and strictly positive, and so must every result;
//! anything else is a [`RhythmError`].
//!
//! ```
//! use rhythm_units::{hz_from_period_minutes, scale_frequency};
//!
//! // A rhythm with a 10-minute period, sped up 180 times, repeats 0.3 times a second.
//! let f = hz_from_period_minutes(10.0)?;
//! assert!((scale_frequency(f, 180.0)? - 0.3).abs() < 1e-15);
//! # Ok::<(), rhythm_units::RhythmError>(())
//! ```

use std::fmt;

/// Seconds in a minute.
pub const SECONDS_PER_MINUTE: f64 = 60.0;

/// A value that is not a finite number greater than zero.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RhythmError {
    name: &'static str,
    value: f64,
}

impl RhythmError {
    /// The parameter (or `"result"`) that was out of range.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// The offending value.
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl fmt::Display for RhythmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} must be a finite number greater than 0, got {}",
            self.name, self.value
        )
    }
}

impl std::error::Error for RhythmError {}

/// The result type of this crate.
pub type Result<T> = std::result::Result<T, RhythmError>;

/// Accepts `value` only if it is finite and strictly positive.
fn positive(name: &'static str, value: f64) -> Result<f64> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(RhythmError { name, value })
    }
}

/// Frequency in hertz of a rhythm whose period is `period_s` seconds: f = 1 / T.
pub fn hz_from_period(period_s: f64) -> Result<f64> {
    positive("result", 1.0 / positive("period_s", period_s)?)
}

/// Period in seconds of a rhythm with frequency `hz`: T = 1 / f.
pub fn period_from_hz(hz: f64) -> Result<f64> {
    positive("result", 1.0 / positive("hz", hz)?)
}

/// Frequency in hertz of a rhythm whose period is `minutes` minutes: f = 1 / (60 · minutes).
pub fn hz_from_period_minutes(minutes: f64) -> Result<f64> {
    positive(
        "result",
        1.0 / (SECONDS_PER_MINUTE * positive("minutes", minutes)?),
    )
}

/// Beats per minute of a rhythm with frequency `hz`: 60 · f.
pub fn bpm_from_hz(hz: f64) -> Result<f64> {
    positive("result", SECONDS_PER_MINUTE * positive("hz", hz)?)
}

/// Frequency in hertz of a rhythm at `bpm` beats per minute: bpm / 60.
pub fn hz_from_bpm(bpm: f64) -> Result<f64> {
    positive("result", positive("bpm", bpm)? / SECONDS_PER_MINUTE)
}

/// The frequency after speeding time up by the factor `k`: f · k.
///
/// `k > 1` compresses time (a slow biological rhythm becomes audible);
/// `k < 1` stretches it.
pub fn scale_frequency(hz: f64, k: f64) -> Result<f64> {
    positive("result", positive("hz", hz)? * positive("k", k)?)
}

/// The period after speeding time up by the factor `k`: T / k.
pub fn scale_period(period_s: f64, k: f64) -> Result<f64> {
    positive(
        "result",
        positive("period_s", period_s)? / positive("k", k)?,
    )
}

/// The speed-up factor `k` that turns `from_hz` into `to_hz`: to / from.
pub fn speedup_for(from_hz: f64, to_hz: f64) -> Result<f64> {
    positive(
        "result",
        positive("to_hz", to_hz)? / positive("from_hz", from_hz)?,
    )
}

/// The ratio of two frequencies, f1 / f2. Unchanged by any speed-up (see [`scale_frequency`]).
pub fn ratio(f1: f64, f2: f64) -> Result<f64> {
    positive("result", positive("f1", f1)? / positive("f2", f2)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_values_that_are_not_finite_and_positive() {
        for bad in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(hz_from_period(bad).is_err(), "{bad}");
            assert!(scale_frequency(1.0, bad).is_err(), "{bad}");
        }
        let err = hz_from_bpm(-5.0).unwrap_err();
        assert_eq!((err.name(), err.value()), ("bpm", -5.0));
        assert_eq!(
            err.to_string(),
            "bpm must be a finite number greater than 0, got -5"
        );
    }

    #[test]
    fn rejects_results_that_overflow() {
        assert_eq!(hz_from_period(1e-310).unwrap_err().name(), "result");
        assert_eq!(scale_frequency(1e300, 1e300).unwrap_err().name(), "result");
    }

    #[test]
    fn speeding_up_keeps_ratios() {
        let (a, b, k) = (12.2, 4.3, 180.0);
        let scaled = ratio(
            scale_frequency(a, k).unwrap(),
            scale_frequency(b, k).unwrap(),
        )
        .unwrap();
        assert!((scaled - ratio(a, b).unwrap()).abs() < 1e-15);
    }
}
