# simple_web

Minimal web example that wires DOM events to `ui-events` using `ui-events-web`.

## What it does

- Attaches listeners for pointer (Pointer Events), wheel, and keyboard
- Converts DOM events into `ui-events` types via `ui-events-web`
- Visualizes pointer input on a full-window canvas (current, coalesced, predicted)
- Shows a small HUD with pressed keys and the last key event; includes Clear and coalesced/predicted toggles
- Logs converted events to the browser console for inspection

## Build and run (with trunk)

1. Install prerequisites:
   - `rustup target add wasm32-unknown-unknown`
   - `cargo install trunk`
2. From this directory:
   - `trunk serve --open`
3. Open the devtools console to see the logs.

## Build with wasm-pack (alternative)

1. `cargo install wasm-pack`
2. `wasm-pack build --target web`
3. Serve the `pkg/` folder with any static server, open `index.html` that imports `pkg/simple_web.js`.

## Notes

- This is intentionally lightweight: a single canvas and small HUD meant to
  demonstrate the conversion helpers and ergonomics (not a full UI toolkit).
- The coalesced/predicted toggles drive both visualization and conversion via
  `ui-events-web::pointer::Options` to avoid unnecessary allocations in hot
  paths when disabled.
- The example uses W3C Pointer Events exclusively (plus wheel), which unify
  mouse, touch, and pen input. This avoids duplicate synthetic `MouseEvent`s and
  ensures touch/pen are handled without extra code. Add mouse listeners only as
  a fallback when targeting very old browsers without Pointer Events support.
