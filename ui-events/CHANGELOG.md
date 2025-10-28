<!-- Instructions

This changelog follows the patterns described here: <https://keepachangelog.com/en/>.

Subheadings to categorize changes are `added, changed, deprecated, removed, fixed, security`.

-->

# Changelog

The latest published UI Events release is [0.1.0](#010-2025-05-08) which was released on 2025-05-08.
You can find its changes [documented below](#010-2025-05-08).

## [Unreleased]

This release has an [MSRV][] of 1.82.

### Added

* `PointerId`, `PointerInfo`, `PointerUpdate`, and `PointerEvent` now have an `is_primary_pointer` method. ([#54][] by [@waywardmonkeys][])
* `PointerGesture` and `PointerGestureEvent` types, with `Gesture` variant added to `PointerEvent`. ([#80][] by [@xorgy][] and [@arthur-fontaine][])
* `scale_factor` field to `EventState` for convenient conversion between logical and device pixels ([#82][] by [@jrmoulton][] and [@xorgy][]).
  * Optional `kurbo` integration behind the `kurbo` Cargo feature with convenience helpers for converting DPI position to `kurbo::Point`.

### Changed

* Convert `PointerEvent` struct variants (`Down`, `Up`, `Scroll`) to separate structs. ([#63][] by [@nicoburns][])

## [0.1.0][] - 2025-05-08

This release has an [MSRV][] of 1.73.

This is the initial release.


[@arthur-fontaine]: https://github.com/arthur-fontaine
[@jrmoulton]: https://github.com/jrmoulton
[@nicoburns]: https://github.com/nicoburns
[@waywardmonkeys]: https://github.com/waywardmonkeys
[@xorgy]: https://github.com/xorgy

[#54]: https://github.com/endoli/ui-events/pull/54
[#63]: https://github.com/endoli/ui-events/pull/63
[#80]: https://github.com/endoli/ui-events/pull/80
[#82]: https://github.com/endoli/ui-events/pull/82

[Unreleased]: https://github.com/endoli/ui-events/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/endoli/ui-events/releases/tag/v0.1.0

[MSRV]: README.md#minimum-supported-rust-version-msrv
