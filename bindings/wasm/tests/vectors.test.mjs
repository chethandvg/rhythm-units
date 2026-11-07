// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama
// The shared golden vectors, checked against the WebAssembly build.

import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { test } from "node:test";
import { checkDir } from "golden-vectors";

const impl = createRequire(import.meta.url)("../pkg/node/rhythm_units_wasm.js");

test("golden vectors", () => {
  assert.ok(checkDir(new URL("../../../vectors/", import.meta.url), impl) > 0);
});
