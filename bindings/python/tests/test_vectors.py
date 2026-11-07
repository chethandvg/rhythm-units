# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama
"""The shared golden vectors, checked against the Python bindings."""

from pathlib import Path

import rhythm_units
from evidence_kernel.vectors import check_dir

VECTORS = Path(__file__).resolve().parents[3] / "vectors"


def test_golden_vectors():
    assert check_dir(VECTORS, rhythm_units) > 0
