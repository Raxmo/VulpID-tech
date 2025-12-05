# Dependencies and Attribution

VulpID-tech depends on the following open-source libraries. All dependencies use permissive licenses that do not require visible in-app attribution.

## Core Dependencies

### winit
- **License**: Apache License 2.0
- **Repository**: https://github.com/rust-windowing/winit
- **Purpose**: Cross-platform window creation and management
- **Attribution**: Copyright (c) 2014-2024 The winit contributors

### glow
- **License**: Apache License 2.0
- **Repository**: https://github.com/gfx-rs/glow
- **Purpose**: Modern OpenGL bindings for Rust
- **Attribution**: Copyright (c) 2015-2024 The glow contributors

### alto
- **License**: MIT OR Apache License 2.0 (dual licensed)
- **Repository**: https://github.com/jpernst/alto
- **Purpose**: Idiomatic Rust bindings for OpenAL 1.1 and extensions
- **Attribution**: Copyright (c) 2015 Josh Pernst

## Transitive Dependencies

Additional dependencies are pulled in by the libraries above. The full dependency tree can be inspected with:

```bash
cargo tree
```

To view full license information:

```bash
cargo license
```

## License Compliance

All dependencies are compatible with VulpID-tech's dual Apache 2.0 / MIT licensing model. Users of VulpID-tech must include appropriate license notices when distributing the engine or applications built with it.

---

**Last Updated**: December 5, 2025
**Status**: Initial setup
