//! Window system
//!
//! Thin wrapper around winit for cross-platform window creation and event handling.

use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window as WinitWindow, WindowAttributes};
use std::num::NonZeroU32;

/// A VulpID window wrapping winit
pub struct Window {
    window: WinitWindow,
}

impl Window {
    /// Create a new window with default settings
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self, Box<dyn std::error::Error>> {
        let attributes = WindowAttributes::default()
            .with_title("VulpID-tech")
            .with_inner_size(LogicalSize::new(1280.0, 720.0));

        let window = event_loop.create_window(attributes)?;

        Ok(Window { window })
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
