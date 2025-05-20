// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Pointer Event Types

mod buttons;

pub use buttons::{PointerButton, PointerButtons};

extern crate alloc;
use alloc::vec::Vec;

use core::num::NonZeroU64;

use dpi::{PhysicalPosition, PhysicalSize};
use keyboard_types::Modifiers;

use crate::ScrollDelta;

/// A unique identifier for the pointer.
///
/// PointerId(1) is reserved for the primary pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PointerId(NonZeroU64);

impl PointerId {
    /// The id of the primary pointer.
    pub const PRIMARY: Self = Self(NonZeroU64::MIN);

    /// Make a new `PointerId` from a `u64`.
    #[inline(always)]
    pub fn new(n: u64) -> Option<Self> {
        NonZeroU64::new(n).map(PointerId)
    }

    /// Return `true` if this is the primary `PointerId`.
    #[inline(always)]
    pub fn is_primary_pointer(self) -> bool {
        self == Self::PRIMARY
    }
}

/// An identifier for the pointing device that is stable across the session.
///
/// PointerId(1) is reserved for the primary pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PersistentDeviceId(NonZeroU64);

impl PersistentDeviceId {
    /// Make a new `PersistentDeviceId` from a `u64`.
    #[inline(always)]
    pub fn new(n: u64) -> Option<Self> {
        NonZeroU64::new(n).map(PersistentDeviceId)
    }
}

/// The type of device that has generated a pointer event.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum PointerType {
    /// The type of device could not be determined.
    #[default]
    Unknown,
    /// A mouse.
    Mouse,
    /// A pen or stylus.
    Pen,
    /// A touch contact.
    Touch,
}

/// Identifying information about a pointer, stable across states.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PointerInfo {
    /// Pointer ID.
    ///
    /// [`PointerId::PRIMARY`] is reserved for the primary pointer,
    /// so when converting platform pointer IDs on a platform that
    /// does not reserve this value, add an offset to avoid collision.
    ///
    /// `None` is for events not originating from a pointing device.
    pub pointer_id: Option<PointerId>,
    /// Persistent device ID.
    ///
    /// This should be set when the platform can identify a given pointing
    /// device during the whole session, and associate it with events.
    /// If this is not possible for the given event, it should be `None`.
    pub persistent_device_id: Option<PersistentDeviceId>,
    /// Pointer type.
    pub pointer_type: PointerType,
}

impl PointerInfo {
    /// Returns `true` if this is the primary pointer.
    #[inline(always)]
    pub fn is_primary_pointer(&self) -> bool {
        self.pointer_id.is_some_and(PointerId::is_primary_pointer)
    }
}

/// Orientation of a pointer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointerOrientation {
    /// Spherical altitude.
    ///
    /// 0 is parallel to the surface, π/2 is perpendicular.
    pub altitude: f32,
    /// Spherical azimuth.
    ///
    /// 0 is the positive x axis, π/2 is positive y.
    pub azimuth: f32,
}

impl Default for PointerOrientation {
    fn default() -> Self {
        Self {
            altitude: core::f32::consts::FRAC_PI_2,
            azimuth: core::f32::consts::FRAC_PI_2,
        }
    }
}

/// The size of an input, usually touch.
///
/// If this is not provided by the underlying API, platform, or device,
/// then it should be a single pixel.
pub type ContactGeometry = PhysicalSize<f64>;

/// A single pointer state.
#[derive(Clone, Debug, PartialEq)]
pub struct PointerState {
    /// `u64` nanoseconds real time.
    ///
    /// The base time is not important, except by convention, and should
    /// generally be the same at least for states originating from the
    /// same device.
    pub time: u64,
    /// Position.
    pub position: PhysicalPosition<f64>,
    /// Pressed buttons.
    pub buttons: PointerButtons,
    /// Modifiers state.
    pub modifiers: Modifiers,
    /// Click or tap count associated with the pointer.
    pub count: u8,
    /// The size of an input, usually touch.
    ///
    /// If this is not provided by the underlying API, platform, or device,
    /// then it should be a single pixel.
    pub contact_geometry: ContactGeometry,
    /// Orientation.
    pub orientation: PointerOrientation,
    /// Normalized pressure in range 0..1.
    ///
    /// Where pressure is not reported by the platform, it
    /// is always 0.5 when activated and 0.0 when not.
    pub pressure: f32,
    /// Normalized ‘tangential pressure’ in range -1..1.
    ///
    /// This is an arbitrary parameter and, despite its name,
    /// it may not originate from a pressure sensitive control.
    /// This is often controlled by something like a wheel on the
    /// barrel of an ‘airbrush’ style pen.
    pub tangential_pressure: f32,
}

impl Default for PointerState {
    fn default() -> Self {
        Self {
            time: 0,
            position: PhysicalPosition::<f64>::default(),
            buttons: PointerButtons::default(),
            modifiers: Modifiers::default(),
            count: 0,
            contact_geometry: ContactGeometry {
                width: 1.0,
                height: 1.0,
            },
            orientation: PointerOrientation::default(),
            // No buttons pressed, therefore no pressure.
            pressure: 0.0,
            tangential_pressure: 0.0,
        }
    }
}

/// A pointer update, along with coalesced and predicted states.
#[derive(Clone, Debug, PartialEq)]
pub struct PointerUpdate {
    /// Identifying information about pointer.
    pub pointer: PointerInfo,
    /// Current state.
    pub current: PointerState,
    /// Coalesced states, ordered by `time`.
    ///
    /// Coalescing is application-specific.
    /// On the web, the browser does its own coalescing, whereas
    /// on other platforms you may do your own, or forego it
    /// altogether, delivering every state.
    pub coalesced: Vec<PointerState>,
    /// Predicted states, ordered by `time`.
    ///
    /// Some platforms provide predicted states directly,
    /// and you may choose to add your own predictor.
    pub predicted: Vec<PointerState>,
}

impl PointerUpdate {
    /// Returns `true` if this is the primary pointer.
    #[inline(always)]
    pub fn is_primary_pointer(&self) -> bool {
        self.pointer.is_primary_pointer()
    }
}

/// A standard `PointerEvent`.
///
/// This is intentionally limited to standard pointer events,
/// and it is expected that applications and frameworks that
/// support more event types will use this as a base and add
/// what they need in a conversion.
#[derive(Clone, Debug)]
pub enum PointerEvent {
    /// A [`PointerButton`] was pressed.
    Down {
        /// The [`PointerButton`] that was pressed..
        button: Option<PointerButton>,
        /// Identity of the pointer.
        pointer: PointerInfo,
        /// The state of the pointer (i.e. position, pressure, etc.).
        state: PointerState,
    },
    /// A [`PointerButton`] was released.
    Up {
        /// The [`PointerButton`] that was released.
        button: Option<PointerButton>,
        /// Identity of the pointer.
        pointer: PointerInfo,
        /// The state of the pointer (i.e. position, pressure, etc.).
        state: PointerState,
    },
    /// Pointer moved.
    Move(PointerUpdate),
    /// Pointer motion was cancelled.
    ///
    /// Usually this is a touch which was taken over somewhere else.
    /// You should try to undo the effect of the gesture when you receive this.
    Cancel(PointerInfo),
    /// Pointer entered the area that receives this event.
    Enter(PointerInfo),
    /// Pointer left the area that receives these events.
    Leave(PointerInfo),
    /// A scroll was requested at the pointer location.
    ///
    /// Usually this is caused by a mouse wheel or a touchpad.
    Scroll {
        /// Identity of the pointer.
        pointer: PointerInfo,
        /// The delta of the scroll.
        delta: ScrollDelta,
        /// The state of the pointer (i.e. position, pressure, etc.).
        state: PointerState,
    },
}

impl PointerEvent {
    /// Returns `true` if this event is for the primary pointer.
    #[inline(always)]
    pub fn is_primary_pointer(&self) -> bool {
        match self {
            Self::Down { pointer, .. }
            | Self::Up { pointer, .. }
            | Self::Move(PointerUpdate { pointer, .. })
            | Self::Cancel(pointer)
            | Self::Enter(pointer)
            | Self::Leave(pointer)
            | Self::Scroll { pointer, .. } => pointer.is_primary_pointer(),
        }
    }
}
