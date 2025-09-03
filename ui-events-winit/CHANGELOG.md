<!-- Instructions

This changelog follows the patterns described here: <https://keepachangelog.com/en/>.

Subheadings to categorize changes are `added, changed, deprecated, removed, fixed, security`.

-->

# Changelog

The latest published UI Events Winit release is [0.1.0](#010-2025-05-08) which was released on 2025-05-08.
You can find its changes [documented below](#010-2025-05-08).

## [Unreleased]

This release has an [MSRV][] of 1.82.

### Added

* `PointerGesture` and `PointerGestureEvent` types, with `Gesture` variant added to `PointerEvent`. ([#80][] by [@xorgy][] and [@arthur-fontaine][])
* `scale_factor` parameter to `WindowEventReducer::reduce` for device-independent slop in tap detection. ([#78][] by [@xorgy][])

### Changed

* Convert `PointerEvent` struct variants (`Down`, `Up`, `Scroll`) to separate structs. ([#63][] by [@nicoburns][])
* Reduce allocations in `TapCounter`. ([#61][] by [@nicoburns][])

## [0.1.0][] - 2025-05-08

This release has an [MSRV][] of 1.73.

This is the initial release.


[@arthur-fontaine]: https://github.com/arthur-fontaine
[@nicoburns]: https://github.com/nicoburns
[@xorgy]: https://github.com/xorgy

[#61]: https://github.com/endoli/ui-events/pull/61
[#63]: https://github.com/endoli/ui-events/pull/63
[#78]: https://github.com/endoli/ui-events/pull/78
[#80]: https://github.com/endoli/ui-events/pull/80

[Unreleased]: https://github.com/endoli/ui-events/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/endoli/ui-events/releases/tag/v0.1.0

[MSRV]: README.md#minimum-supported-rust-version-msrv
