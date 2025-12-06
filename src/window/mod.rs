//! Window system
//!
//! Thin wrapper around winit for cross-platform window creation and event handling.

use winit::window::{Window as WinitWindow};

/// A VulpID window wrapping winit
pub struct Window {
    window: WinitWindow,
}

impl Window {
    /// Create a window from an existing winit::Window
    /// 
    /// This is the primary constructor for the new winit 0.30+ API
    pub fn new_from_raw(window: WinitWindow) -> Self {
        Window { window }
    }

    /// Get a reference to the underlying winit window
    pub fn inner(&self) -> &WinitWindow {
        &self.window
    }

    /// Get a mutable reference to the underlying winit window
    pub fn inner_mut(&mut self) -> &mut WinitWindow {
        &mut self.window
    }

    /// Request a redraw
    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    /// Check if window should close
    pub fn should_close(&self) -> bool {
        // Will be set by the application's event loop
        false
    }
}
