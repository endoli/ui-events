// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(missing_docs)]

use dpi::PhysicalPosition;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScrollDelta {
    PageDelta(f32, f32),

    LineDelta(f32, f32),

    PixelDelta(PhysicalPosition<f64>),
}
