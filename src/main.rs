use std::{thread::sleep, time::Duration};

use inputbot::{KeybdKey::*, MouseCursor};

/// This example demonstrates moving the mouse around on the screen both relative to its position,
/// and absolute. To use these functions effectively, you would ideally combine it with a library
/// which exposes your system display resolution, as well as separates different monitors.

let X:int = 3840;
let Y:int = 2160;

fn main() {
    // Bind our 1 key to a function that moves the mouse absolute to your monitors. Note: if you
    // have multiple monitors, 0, 0 might be not where you're expecting. If we wanted to get the
    // absolute position of your primary (or a specific) monitor, we would need to bring in extra
    // libraries.
    Numpad1Key.bind(|| {
        MouseCursor::move_abs(X/4, Y/4);
        sleep(Duration::from_millis(1));
    });
    Numpad3Key.bind(|| {
        MouseCursor::move_abs(3*X/4, Y/4);
        sleep(Duration::from_millis(1));
    });
    Numpad7Key.bind(|| {
        MouseCursor::move_abs(X/4, 3*Y/4);
        sleep(Duration::from_millis(1));
    });
    Numpad9Key.bind(|| {
        MouseCursor::move_abs(3*X/4, 3*Y/4);
        sleep(Duration::from_millis(1));
    });

    // Bind our 2 key to a function that moves the mouse relative to its current position.
    // This will be 100 pixels over and 100 pixels down.
    Numrow2Key.bind(|| {
        MouseCursor::move_rel(0, -100);
        sleep(Duration::from_millis(1));
    });
    Numrow4Key.bind(|| {
        MouseCursor::move_rel(-100, 0);
        sleep(Duration::from_millis(1));
    });
    Numrow6Key.bind(|| {
        MouseCursor::move_rel(100, 0);
        sleep(Duration::from_millis(1));
    });
    Numrow2Key.bind(|| {
        MouseCursor::move_rel(0, 100);
        sleep(Duration::from_millis(1));
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events(false);
}