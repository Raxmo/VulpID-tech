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

- Inspired by IDtech engine architecture
- Focus on performance and extensibility
- Clean separation of concerns
- Rust's type system for safety guarantees

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

### Physics System

### Input System

### Entity-Component System

### Scripting/Logic System

## Module Organization

```
src/
├── lib.rs
├── main.rs
├── render/      # Rendering pipeline
├── physics/     # Physics simulation
├── input/       # Input handling
├── ecs/         # Entity-Component System
└── utils/       # Utility functions
```

## Data Flow

*(To be documented as architecture solidifies)*

## API Reference

*(To be documented as public APIs stabilize)*

## Performance Considerations

- Leveraging Rust's zero-cost abstractions
- Cache-friendly data structures
- Minimal allocations in hot paths
- Data-oriented design for rendering and physics

---

**Last Updated:** December 5, 2025
**Status:** Project initialization phase

## Design Notes

### Rendering Philosophy

One of VulpID-tech's core strengths is proper handling of both 2D and 3D rendering. Unlike many engines that either force everything into 3D or treat 2D as a second-class citizen, VulpID-tech gives both dimensions first-class support through its layer-based architecture. Users can work with 2D and 3D natively within a single, unified system.
