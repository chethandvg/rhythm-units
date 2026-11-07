// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama

//! Golden-vector tests. The Python and WebAssembly bindings check the same
//! `vectors/*.json` cases, so all three implementations give the same answers.

use golden_vectors::{Outcome, f64_arg};
use rhythm_units as r;
use serde_json::Value;

/// Maps each vector's `fn` name to this crate's implementation.
fn dispatch(name: &str, args: &[Value]) -> Outcome {
    let one = |f: fn(f64) -> r::Result<f64>| Outcome::from(f(f64_arg(args, 0)));
    let two =
        |f: fn(f64, f64) -> r::Result<f64>| Outcome::from(f(f64_arg(args, 0), f64_arg(args, 1)));
    match name {
        "hz_from_period" => one(r::hz_from_period),
        "period_from_hz" => one(r::period_from_hz),
        "hz_from_period_minutes" => one(r::hz_from_period_minutes),
        "bpm_from_hz" => one(r::bpm_from_hz),
        "hz_from_bpm" => one(r::hz_from_bpm),
        "scale_frequency" => two(r::scale_frequency),
        "scale_period" => two(r::scale_period),
        "speedup_for" => two(r::speedup_for),
        "ratio" => two(r::ratio),
        _ => Outcome::unknown(name, args),
    }
}

#[test]
fn golden_vectors() {
    golden_vectors::check_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/vectors"), dispatch);
}
