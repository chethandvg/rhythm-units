/-
SPDX-License-Identifier: Apache-2.0
SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama
-/
import Mathlib.Algebra.Field.Periodic
import Mathlib.Basic.Real.Basic
import Mathlib.Tactic.FieldSimp
import Mathlib.Tactic.NormNum
import Mathlib.Tactic.Positivity
import Mathlib.Tactic.Ring

/-!
# Rhythm units

Definitions mirror the Rust functions of the same names (`hz_from_period` is
`hzFromPeriod`, and so on). The theorems are about exact real numbers. The Rust
code computes the same formulas in floating point, and the golden vectors check
that its rounding stays within a few units in the last place.
-/

namespace RhythmUnits

/-- Frequency of a rhythm with period `T`: `f = 1 / T`. -/
noncomputable def hzFromPeriod (T : ℝ) : ℝ := 1 / T

/-- Period of a rhythm with frequency `f`: `T = 1 / f`. -/
noncomputable def periodFromHz (f : ℝ) : ℝ := 1 / f

/-- Frequency of a rhythm whose period is `m` minutes. -/
noncomputable def hzFromPeriodMinutes (m : ℝ) : ℝ := 1 / (60 * m)

/-- Beats per minute of a rhythm with frequency `f`. -/
def bpmFromHz (f : ℝ) : ℝ := 60 * f

/-- Frequency of a rhythm at `b` beats per minute. -/
noncomputable def hzFromBpm (b : ℝ) : ℝ := b / 60

/-- The frequency after speeding time up by the factor `k`. -/
def scaleFrequency (f k : ℝ) : ℝ := f * k

/-- The period after speeding time up by the factor `k`. -/
noncomputable def scalePeriod (T k : ℝ) : ℝ := T / k

/-- The speed-up factor that turns frequency `f₁` into `f₂`. -/
noncomputable def speedupFor (f₁ f₂ : ℝ) : ℝ := f₂ / f₁

/-- The ratio of two frequencies. -/
noncomputable def ratio (f₁ f₂ : ℝ) : ℝ := f₁ / f₂

/-- A positive period has a positive frequency. -/
theorem hzFromPeriod_pos {T : ℝ} (hT : 0 < T) : 0 < hzFromPeriod T := by
  unfold hzFromPeriod; positivity

/-- Converting a period to a frequency and back gives the period. -/
theorem period_hz_roundtrip (T : ℝ) : periodFromHz (hzFromPeriod T) = T := by
  simp [periodFromHz, hzFromPeriod]

/-- Converting a frequency to a period and back gives the frequency. -/
theorem hz_period_roundtrip (f : ℝ) : hzFromPeriod (periodFromHz f) = f := by
  simp [periodFromHz, hzFromPeriod]

/-- Converting hertz to bpm and back gives the original frequency. -/
theorem bpm_hz_roundtrip (f : ℝ) : hzFromBpm (bpmFromHz f) = f := by
  unfold hzFromBpm bpmFromHz; ring

/-- Converting bpm to hertz and back gives the original bpm. -/
theorem hz_bpm_roundtrip (b : ℝ) : bpmFromHz (hzFromBpm b) = b := by
  unfold hzFromBpm bpmFromHz; ring

/-- A period of `m` minutes is the same rhythm as a period of `60 * m` seconds. -/
theorem minutes_as_seconds (m : ℝ) : hzFromPeriodMinutes m = hzFromPeriod (60 * m) := rfl

/-- **Time scaling.** If a signal `x` repeats every `T`, then the sped-up signal
`t ↦ x (k * t)` repeats every `T / k`. -/
theorem time_scaling_periodic {α : Type*} {x : ℝ → α} {T : ℝ} (hx : Function.Periodic x T)
    (k : ℝ) : Function.Periodic (fun t => x (k * t)) (scalePeriod T k) := by
  simpa [scalePeriod, div_eq_inv_mul] using hx.const_mul k

/-- **Time scaling.** Speeding time up by `k` turns the frequency `f` into `f * k`. -/
theorem time_scaling_frequency (T k : ℝ) :
    hzFromPeriod (scalePeriod T k) = scaleFrequency (hzFromPeriod T) k := by
  unfold hzFromPeriod scalePeriod scaleFrequency
  rw [one_div_div]; ring

/-- **Ratios survive time scaling.** For any non-zero speed-up `k`, the ratio of two
sped-up frequencies equals the ratio of the original frequencies. -/
theorem scale_ratio_invariant {f₁ f₂ k : ℝ} (hk : k ≠ 0) :
    ratio (scaleFrequency f₁ k) (scaleFrequency f₂ k) = ratio f₁ f₂ := by
  unfold ratio scaleFrequency; exact mul_div_mul_right f₁ f₂ hk

/-- `speedupFor f₁ f₂` is the factor that turns `f₁` into `f₂`. -/
theorem speedupFor_spec {f₁ : ℝ} (f₂ : ℝ) (h : f₁ ≠ 0) :
    scaleFrequency f₁ (speedupFor f₁ f₂) = f₂ := by
  unfold scaleFrequency speedupFor; field_simp

/-- A rhythm with a 10-minute period has frequency 1/600 Hz (about 1.67 mHz). -/
theorem ten_minutes_hz : hzFromPeriodMinutes 10 = 1 / 600 := by
  unfold hzFromPeriodMinutes; norm_num

/-- A 10-minute rhythm sped up 180 times has frequency 3/10 Hz: once every 3⅓ seconds. -/
theorem ten_minutes_sped_up_180 : scaleFrequency (hzFromPeriodMinutes 10) 180 = 3 / 10 := by
  unfold scaleFrequency hzFromPeriodMinutes; norm_num

end RhythmUnits
