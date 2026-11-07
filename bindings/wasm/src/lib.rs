// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama

//! WebAssembly bindings for `rhythm-units`. Every function keeps the Rust name,
//! so the golden vectors can call it by name; invalid input throws a JS `Error`.

use rhythm_units_rs as imp;
use wasm_bindgen::prelude::*;

/// The crate version.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

/// Declares one exported JS function per Rust function.
macro_rules! bind {
    ($($doc:literal $name:ident($($arg:ident),+);)+) => {
        $(
            #[doc = $doc]
            #[wasm_bindgen]
            pub fn $name($($arg: f64),+) -> Result<f64, JsError> {
                imp::$name($($arg),+).map_err(|e| JsError::new(&e.to_string()))
            }
        )+
    };
}

bind! {
    "Frequency in Hz of a rhythm with period `period_s` seconds: f = 1 / T." hz_from_period(period_s);
    "Period in seconds of a rhythm with frequency `hz`: T = 1 / f." period_from_hz(hz);
    "Frequency in Hz of a rhythm with a period of `minutes` minutes." hz_from_period_minutes(minutes);
    "Beats per minute of a rhythm with frequency `hz`: 60 * f." bpm_from_hz(hz);
    "Frequency in Hz of a rhythm at `bpm` beats per minute: bpm / 60." hz_from_bpm(bpm);
    "The frequency after speeding time up by the factor `k`: f * k." scale_frequency(hz, k);
    "The period after speeding time up by the factor `k`: T / k." scale_period(period_s, k);
    "The speed-up factor k that turns `from_hz` into `to_hz`: to / from." speedup_for(from_hz, to_hz);
    "The ratio of two frequencies, f1 / f2." ratio(f1, f2);
}
