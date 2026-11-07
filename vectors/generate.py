# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama
"""Write the golden vectors for rhythm-units from the reference implementation.

Every result here is rational, so the oracle computes it exactly with
fractions.Fraction from the double-precision inputs, then rounds once to the
nearest double. Run:

    python vectors/generate.py
"""

import json
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent

# The reference implementation: exact rational arithmetic.
ORACLE = {
    "hz_from_period": lambda t: 1 / t,
    "period_from_hz": lambda f: 1 / f,
    "hz_from_period_minutes": lambda m: 1 / (60 * m),
    "bpm_from_hz": lambda f: 60 * f,
    "hz_from_bpm": lambda b: b / 60,
    "scale_frequency": lambda f, k: f * k,
    "scale_period": lambda t, k: t / k,
    "speedup_for": lambda a, b: b / a,
    "ratio": lambda a, b: a / b,
}


def write(
    name: str,
    description: str,
    cases: list[dict],
    rel: float = 1e-12,
    abs_: float = 0.0,
) -> None:
    doc = {
        "schema": "golden-vectors@1",
        "description": description,
        "tolerance": {"rel": rel, "abs": abs_},
        "cases": cases,
    }
    (HERE / f"{name}.json").write_text(
        json.dumps(doc, indent=2) + "\n", encoding="utf-8"
    )


def case(fn: str, *args: F | float, note: str = "") -> dict:
    """The arguments as doubles; the expected value computed exactly from those doubles."""
    doubles = [float(a) for a in args]
    out = {"claim": "rhythm:IMPLEMENTATIONS-AGREE", "fn": fn, "args": doubles}
    out["expected"] = float(ORACLE[fn](*map(F, doubles)))
    return {**out, "note": note} if note else out


def error(fn: str, *args: float, note: str) -> dict:
    return {
        "claim": "rhythm:INPUT-VALIDATION",
        "fn": fn,
        "args": [float(a) for a in args],
        "error": True,
        "note": note,
    }


CASES = [
    case("hz_from_period", 1, note="one cycle per second is 1 Hz"),
    case("hz_from_period", F(1, 2)),
    case("hz_from_period", 600, note="a 10-minute period, in seconds"),
    case("hz_from_period", 86400, note="one day"),
    case("period_from_hz", 2),
    case("period_from_hz", 440, note="concert A"),
    case("period_from_hz", F(1, 600)),
    case("hz_from_period_minutes", 10, note="10-minute period"),
    case("hz_from_period_minutes", 35, note="35-minute period"),
    case("hz_from_period_minutes", F(1, 60), note="one second, expressed in minutes"),
    case("bpm_from_hz", 1),
    case("bpm_from_hz", 19, note="19 beats a second"),
    case("bpm_from_hz", F(1, 600), note="a 10-minute rhythm is 0.1 beats per minute"),
    case("hz_from_bpm", 60),
    case("hz_from_bpm", 72),
    case("hz_from_bpm", 176),
    case(
        "scale_frequency",
        F(1, 600),
        180,
        note="a 10-minute rhythm sped up 180 times: 0.3 Hz",
    ),
    case("scale_frequency", 19, 16),
    case("scale_frequency", F(1, 600), 2**18, note="18 octaves up: 436.91 Hz"),
    case("scale_frequency", 440, F(1, 2), note="k below 1 slows time down"),
    case("scale_period", 600, 180),
    case("scale_period", 1, 1),
    case("speedup_for", F(1, 600), F(3, 10)),
    case("speedup_for", 440, 220),
    case("ratio", 12.2, 4.3),
    case("ratio", 880, 440),
    error("hz_from_period", 0, note="a period must be positive"),
    error("hz_from_period", -600, note="negative period"),
    error("period_from_hz", 0, note="zero frequency"),
    error("hz_from_period_minutes", -1, note="negative minutes"),
    error("bpm_from_hz", -1, note="negative frequency"),
    error("hz_from_bpm", 0, note="zero bpm"),
    error("scale_frequency", 440, 0, note="k must be positive"),
    error("scale_frequency", 440, -2, note="negative k"),
    error("scale_period", 600, 0, note="k must be positive"),
    error("speedup_for", 0, 440, note="cannot speed up from 0 Hz"),
    error("ratio", 440, 0, note="division by zero"),
    error("hz_from_period", 1e-310, note="the result would overflow to infinity"),
]

if __name__ == "__main__":
    # Each function does one or two correctly rounded operations, so results are within a few ulp.
    write(
        "conversions",
        "Period, frequency and bpm conversions and time scaling, computed exactly.",
        CASES,
        rel=1e-15,
    )
    print(f"wrote {len(CASES)} cases to vectors/conversions.json")
