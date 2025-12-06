# VulpID-tech Technical Documentation

## Overview

VulpID-tech is a Rust game engine inspired by the IDtech engines that powered classic games like Quake and Doom. This document covers the architecture, design decisions, and technical implementation details.

## Table of Contents

1. [Architecture](#architecture)
2. [Core Systems](#core-systems)
3. [Module Organization](#module-organization)
4. [Data Flow](#data-flow)
5. [API Reference](#api-reference)
6. [Performance Considerations](#performance-considerations)

## Architecture

*(To be filled as the project develops)*

### High-Level Design

The engine is structured around core systems that handle rendering, physics, input, and game logic.

### Design Philosophy

VulpID-tech is built on a **thin layers of abstraction** principle:

- **Composable, not hierarchical**: Systems are thin wrappers that build on each other, not nested pyramids
- **Breakable at every level**: Users can "jump off" at any layer and rewrite their own system
- **Multiple entry points**: Use high-level convenience APIs or drop down to lower levels
- **Rust-first**: Leverage Rust's type system for safety while maintaining performance
- **Inspired by IDtech**: Learn from proven 90s engine architecture but modernize for today

**Example Layer Stack:**

```
Game (optional convenience wrapper)
  ↓
Window (thin winit wrapper)
  ↓
Input (processes events, exposes raw winit::Event)
  ↓
Keyboard/Mouse (typed input, but winit::Event still accessible)
  ↓
winit (raw library)
```

Users can use `Game`, skip to `Window`, use `Input` standalone, or access `winit` directly. Each layer is optional and rewritable internally.

**Benefits:**
- Experimentation at every level
- No forced abstractions
- Performance: remove layers you don't need
- Educational: learn engine architecture layer by layer
- Flexible: pick and choose which abstractions help you

## Core Systems

*(To be expanded as systems are implemented)*

### Rendering System

#### Layer-Based Architecture

The rendering system uses a flexible, user-defined layer pipeline where users can compose rendering layers in any order. Each layer can be independently typed as 2D or 3D (or custom), with its own rendering logic and event system.

**Core Concepts:**

- **Layers**: Generic rendering units that can be composed into a pipeline
- **Layer Types**: 2D, 3D, or custom (user-defined)
- **Meta-Pipeline**: User-defined ordering and composition of layers
- **Events**: Layers can pass and consume events for inter-layer communication

**Example Pipelines:**

Standard 3D game with UI overlay:
```
3D World Scene → Physical UI → Post-Processing → 2D UI
```

2D-primary with 3D elements:
```
2D Background → 3D Game World → 2D Foreground
```

Custom compositing:
```
3D Pass 1 → Custom Filter → 3D Pass 2 → 2D Overlay
```

**Benefits:**

- Native support for both 2D and 3D without forcing one into the other
- Extensible: users define their own rendering pipeline
- Type-safe: layer types ensure correct rendering logic
- Event-driven communication between layers
- Flexible composition for various game genres and visual styles

### Game Architecture (Token-Based ECS)

The optional high-level `Game` layer uses a **token-based entity-component-system** inspired by the Nominal Token Pattern. This provides organizational structure for larger games without enforcing it on all users.

**Core Concepts:**

- **Tokens/IDs**: Unique identifiers for entities, windows, scenes, renderers
- **Deferred Processing**: Components store tokens, not direct references
- **Central Ownership**: `Game` owns all collections, other components reference by token

**Architecture:**

```rust
struct Game {
    windows: HashMap<WindowId, Window>,
    scenes: HashMap<SceneId, Scene>,
    entities: HashMap<EntityId, Entity>,
    renderers: HashMap<RendererId, Renderer>,
    input: InputState,
}

struct Scene {
    window_id: WindowId,        // token, not owned
    entity_ids: Vec<EntityId>,  // tokens, not owned
}

struct Renderer {
    window_id: WindowId,  // token
}
```

**Processing Pattern:**

```rust
for (scene_id, scene) in &game.scenes {
    for entity_id in &scene.entity_ids {
        let entity = &game.entities[entity_id];  // lookup by token
        // process entity
    }
}
```

**Benefits:**

- No circular ownership or borrow checker conflicts
- Easy to swap, add, or remove components at runtime
- Decouple systems via token lookups
- Natural mapping from Token Pattern (user's C++ design)
- Enables complex game architectures cleanly

**Important:** This is optional. Users can ignore the `Game` layer entirely and use lower-level primitives directly.

### Physics System

### Input System

### Scripting/Logic System

## Module Organization

Modules are organized as thin, composable layers:

```
src/
├── lib.rs
├── main.rs (example application)
│
├── window/          # Thin winit wrapper
│   ├── mod.rs       # Window struct, event loop abstraction
│   └── event.rs     # Event handling (exposes raw winit::Event)
│
├── input/           # Input system built on window events
│   ├── mod.rs       # Input state management
│   ├── keyboard.rs  # Keyboard-specific handling
│   └── mouse.rs     # Mouse-specific handling
│
├── render/          # Rendering pipeline
│   ├── mod.rs       # Renderer abstraction
│   ├── layer.rs     # Layer system
│   └── context.rs   # glow context wrapper
│
├── audio/           # Audio system
│   └── mod.rs       # alto wrapper
│
├── physics/         # Physics simulation (future)
│   └── mod.rs
│
├── ecs/             # Entity-Component System (future)
│   └── mod.rs
│
└── utils/           # Utility functions
    └── mod.rs
```

**Key Principle:** Each module exposes both high-level convenience APIs and low-level primitives, allowing users to bypass any layer they don't need.

## Data Flow

*(To be documented as architecture solidifies)*

## API Reference

### Window Module (`vulpid_tech::window`)

The window module provides a thin wrapper around `winit` for cross-platform window creation and management.

**Main Types:**
- `Window`: Wraps a `winit::Window` and provides convenient methods for window operations

**Example Usage:**
```rust
use vulpid_tech::window::Window;
use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use std::sync::Arc;

struct App {
    window: Option<Arc<Window>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = winit::window::Window::default_attributes()
            .with_title("VulpID-tech")
            .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));
        
        let raw_window = event_loop.create_window(attributes)
            .expect("Failed to create window");
        let window = Arc::new(Window::new_from_raw(raw_window));
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                self.window = None;
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App { window: None };
    let _ = event_loop.run_app(&mut app);
}
```

**Notes:**
- The window module uses the winit 0.30 API with `ApplicationHandler` trait
- Users can access the underlying `winit::Window` via `inner()` or `inner_mut()`
- Window creation occurs in the `resumed()` event handler to ensure proper initialization

*(To be documented as public APIs stabilize)*

## Performance Considerations

- Leveraging Rust's zero-cost abstractions
- Cache-friendly data structures
- Minimal allocations in hot paths
- Data-oriented design for rendering and physics

---

**Last Updated:** December 6, 2025
**Status:** Window system implemented and tested

## Design Notes

### Rendering Philosophy

One of VulpID-tech's core strengths is proper handling of both 2D and 3D rendering. Unlike many engines that either force everything into 3D or treat 2D as a second-class citizen, VulpID-tech gives both dimensions first-class support through its layer-based architecture. Users can work with 2D and 3D natively within a single, unified system.
