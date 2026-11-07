// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama

//! Python bindings for `rhythm-units`. Every function keeps the Rust name, so
//! the golden vectors can call it by name; invalid input raises `ValueError`.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rhythm_units_rs as imp;

/// Declares one Python function per Rust function, then a `register` that adds them all.
macro_rules! bind {
    ($($doc:literal $name:ident($($arg:ident),+);)+) => {
        $(
            #[doc = $doc]
            #[pyfunction]
            fn $name($($arg: f64),+) -> PyResult<f64> {
                imp::$name($($arg),+).map_err(|e| PyValueError::new_err(e.to_string()))
            }
        )+

        fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
            $(m.add_function(wrap_pyfunction!($name, m)?)?;)+
            Ok(())
        }
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

#[pymodule]
fn rhythm_units(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    register(m)
}
