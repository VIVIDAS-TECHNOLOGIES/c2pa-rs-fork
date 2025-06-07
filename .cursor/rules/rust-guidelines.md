# Rust Coding Guidelines

## Project Structure

### Module Organization
```
sdk/src/
├── lib.rs              # Public API and module declarations
├── assertions/         # C2PA assertion implementations
├── asset_handlers/     # Asset format handlers
├── utils/             # Utility functions
├── wasm/              # WebAssembly specific code
└── [domain].rs        # Core domain implementations
```

### Module Rules
1. Public modules should be declared in `lib.rs`
2. Internal modules should use `pub(crate)` visibility
3. Group related functionality in subdirectories
4. Use feature flags for optional functionality
5. Keep file sizes manageable (prefer < 1000 lines)

### Feature Organization
```rust
// In lib.rs
#[cfg(feature = "feature_name")]
pub mod feature_module;

// In code
#[cfg(feature = "feature_name")]
pub fn feature_function() { ... }
```

## Code Formatting

### Rustfmt Configuration
The project uses nightly rustfmt with the following configuration:

```toml
blank_lines_upper_bound = 1
edition = "2018"
format_code_in_doc_comments = true
group_imports = "StdExternalCrate"
hex_literal_case = "Lower"
imports_granularity = "Crate"
reorder_impl_items = true
unstable_features = true
```

To format your code:
```bash
rustup toolchain add nightly
cargo +nightly fmt
```

### Import Organization
```rust
// Standard library imports
use std::path::Path;

// External crate imports
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Internal crate imports
use crate::error::Result;
use crate::utils::hash_utils;
```

## Error Handling

### Error Type Definition
```rust
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    // Domain-specific errors
    #[error("error message: {0}")]
    DomainError(String),

    // Feature-specific errors
    #[cfg(feature = "feature_name")]
    #[error("feature error: {0}")]
    FeatureError(String),

    // Third-party errors
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
```

### Error Usage
```rust
// Prefer
fn process() -> Result<()> {
    let data = read_file()?;
    validate_data(&data)?;
    Ok(())
}

// Avoid
fn process() -> Result<()> {
    let data = read_file().unwrap();
    validate_data(&data).expect("validation failed");
    Ok(())
}
```

## Documentation

### Module Documentation
```rust
//! Module description
//!
//! # Examples
//!
//! ```rust
//! use crate::module::function;
//! let result = function()?;
//! ```
```

### Function Documentation
```rust
/// Function description
///
/// # Arguments
///
/// * `param` - Parameter description
///
/// # Returns
///
/// Returns `Result<T>` where T is the return type
///
/// # Errors
///
/// * `Error::Variant` - When something goes wrong
///
/// # Examples
///
/// ```rust
/// let result = function(param)?;
/// ```
```

## Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_success() {
        let result = function("valid input");
        assert!(result.is_ok());
    }

    #[test]
    fn test_function_error() {
        let result = function("invalid input");
        assert!(result.is_err());
    }
}
```

### Integration Tests
- Place in `tests/` directory
- Test public API
- Use test fixtures
- Test feature-gated functionality

## Performance

### Guidelines
1. Use appropriate data structures
2. Avoid unnecessary allocations
3. Use references when possible
4. Profile before optimizing
5. Use `#[inline]` judiciously

### Common Optimizations
```rust
// Prefer
let mut vec = Vec::with_capacity(size);

// Over
let mut vec = Vec::new();
```

## Security

### Best Practices
1. Validate all input data
2. Use secure random number generators
3. Handle sensitive data carefully
4. Follow the principle of least privilege
5. Use appropriate cryptographic libraries

## Dependencies

### Guidelines
1. Keep dependencies up to date
2. Use specific versions in Cargo.toml
3. Review security advisories
4. Minimize dependencies
5. Use feature flags for optional dependencies

## Commit Messages

Follow conventional commits format:
- `fix`: Bug fixes
- `feat`: New features
- `chore`: Maintenance tasks
- `update`: Updates to existing features
- `doc`: Documentation changes

Format:
```
type(scope): description

[optional body]
[optional footer]
```

Example:
```
feat(api): add new validation method

Adds support for validating C2PA v2 claims with improved error handling.
```

## Platform Support

The project supports:
- x86_64-unknown-linux-gnu
- x86_64-apple-darwin
- x86_64-pc-windows-msvc
- aarch64-apple-darwin
- wasm32-unknown-unknown

Ensure code works across all supported platforms.

## Code Generation

When generating code, follow these patterns:

1. **Module Structure**:
   - Place public API in `lib.rs`
   - Use `pub(crate)` for internal modules
   - Group related functionality in subdirectories

2. **Error Handling**:
   - Use `thiserror` for error types
   - Implement `From` for error conversions
   - Use `?` operator for error propagation

3. **Feature Flags**:
   - Gate optional functionality
   - Document feature requirements
   - Test feature combinations

4. **Documentation**:
   - Include examples
   - Document error conditions
   - Use markdown formatting

5. **Testing**:
   - Write unit tests
   - Include integration tests
   - Test error cases 