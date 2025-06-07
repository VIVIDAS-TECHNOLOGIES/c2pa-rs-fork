# C2PA Rust Library Project Context

## Project Overview
This repository is a fork of the [C2PA Rust library](https://github.com/contentauth/c2pa-rs), with a specific focus on adding support for DASH VOD MBR (Multi-Bitrate) CMAF (Common Media Application Format) files. The original C2PA Rust library is an implementation of the [C2PA technical specification](https://c2pa.org/specifications/specifications/2.1/specs/C2PA_Specification.html) in Rust, part of the [Content Authenticity Initiative](https://contentauthenticity.org) open-source SDK.

## Fork-Specific Objectives
The primary goal of this fork is to extend the library's capabilities to support:
- DASH VOD (Video on Demand) content
- Multi-Bitrate (MBR) streaming
- CMAF (Common Media Application Format) files
- Specifically targeting `.m4s` and CMAF file formats

## Current Development Status
### Implemented Features
- Partial support for `.m4s` files
- Partial support for CMAF files
- Initial segment signing capability (metadata addition)

### Known Issues
- BMFF (Base Media File Format) mismatch errors when signing CMAF files
- This occurs even when the init segment is successfully signed

## Core Functionality
- Creation and signing of C2PA claims and manifests
- Embedding manifests in supported file formats
- Parsing and validating manifests in supported file formats
- Support for common C2PA assertions and hard bindings

## Technical Stack
- **Language**: Rust (MSRV: 1.82.0)
- **Key Dependencies**:
  - `c2pa-crypto`: Custom crypto implementation
  - `c2pa-status-tracker`: Status tracking functionality
  - `serde`: Serialization/deserialization
  - `image`: Image processing (optional)
  - `lopdf`: PDF support (optional)
  - Various cryptographic libraries (sha2, ed25519-dalek, etc.)
- **Media Format Support**:
  - DASH VOD
  - CMAF
  - BMFF
  - MBR streaming

## Project Structure
```
.
├── sdk/               # Main library implementation
├── cli/              # Command-line interface
├── examples/         # Example implementations
├── internal/         # Internal crates
├── docs/            # Documentation
└── make_test_images/ # Test image generation
```

## Key Features
1. **File Format Support**:
   - Multiple image formats
   - PDF support (optional)
   - MP4 support
   - Custom format extensions possible

2. **Platform Support**:
   - Native platforms
   - WebAssembly (WASM)
   - WASI environments

3. **Security Features**:
   - Cryptographic signing
   - Manifest validation
   - X.509 certificate support

## Development Focus
Current development efforts are focused on:
1. Resolving BMFF mismatch errors in CMAF file signing
2. Completing full support for DASH VOD MBR CMAF files
3. Ensuring compatibility with existing C2PA functionality
4. Maintaining alignment with upstream C2PA-RS updates

## Development Status
- Current version: 0.49.5 (Beta)
- Supports C2PA v2 claims (in progress)
- Breaking changes may occur in minor versions (0.x.0)

## Important Considerations

### Performance
- File I/O operations can be optimized using the `no_interleaved_io` feature
- Thumbnail generation is optional (`add_thumbnails` feature)
- Remote manifest fetching is supported (`fetch_remote_manifests` feature)

### Security
- Cryptographic operations are handled by `c2pa-crypto`
- Support for both native and WASM cryptographic implementations
- X.509 certificate validation

### Integration Points
1. **File I/O**:
   - Custom I/O implementations possible
   - Default file I/O available via `file_io` feature

2. **Cryptography**:
   - Native Rust crypto via `rust_native_crypto` feature
   - WASM crypto support for web environments

3. **Serialization**:
   - JSON schema generation support
   - CBOR serialization
   - XML support for specific formats

## Known Limitations
1. Beta status means API stability is not guaranteed
2. C2PA v2 claims support is still in progress
3. Some features are platform-specific (WASM vs native)
4. Partial support for DASH VOD MBR CMAF files
5. BMFF mismatch issues in CMAF signing

## Development Guidelines
1. Follow Rust 1.82.0+ compatibility
2. Use feature flags for optional functionality
3. Maintain backward compatibility where possible
4. Document breaking changes in release notes

## Testing Strategy
- Unit tests in `sdk/tests/`
- Integration tests for file format support
- WASM-specific tests for web functionality
- Platform-specific test suites

## Future Considerations
1. Complete C2PA v2 claims implementation
2. Enhanced platform support
3. Performance optimizations
4. Additional file format support
5. Full DASH VOD MBR CMAF support
6. Resolution of BMFF compatibility issues

## Resources
- [Official Documentation](https://opensource.contentauthenticity.org/docs/rust-sdk/)
- [C2PA Specification](https://c2pa.org/specifications/specifications/2.1/specs/C2PA_Specification.html)
- [GitHub Repository](https://github.com/contentauth/c2pa-rs)
- [Crates.io Page](https://crates.io/crates/c2pa) 