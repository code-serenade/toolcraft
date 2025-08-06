# toolcraft-utils

Common utilities and helper functions for the toolcraft ecosystem.

[![Crates.io](https://img.shields.io/crates/v/toolcraft-utils.svg)](https://crates.io/crates/toolcraft-utils)
[![Documentation](https://docs.rs/toolcraft-utils/badge.svg)](https://docs.rs/toolcraft-utils)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Features

- 🚀 Common utility functions for everyday use
- 🔧 Helper functions for the toolcraft ecosystem
- 🎯 Zero dependencies for core functionality
- ⚡ Performance-focused implementations
- 🧩 Modular design - use only what you need

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
toolcraft-utils = "*"
```

Check the [crates.io page](https://crates.io/crates/toolcraft-utils) for the latest version.

## Quick Start

```rust
use toolcraft_utils::*;

fn main() {
    // Utility functions will be available here
    // This is a placeholder for future utilities
}
```

## Planned Features

This crate is currently in early development. Planned utilities include:

### String Utilities
- String manipulation helpers
- Case conversion utilities
- Text processing functions

### File System Utilities
- Path manipulation helpers
- File reading/writing utilities
- Directory traversal helpers

### Time Utilities
- Duration formatting
- Time calculation helpers
- Timestamp utilities

### Data Structure Utilities
- Collection helpers
- Data transformation utilities
- Common algorithms

### Error Handling Utilities
- Result combinators
- Error conversion helpers
- Retry mechanisms

### Encoding Utilities
- Base64 encoding/decoding
- Hex encoding/decoding
- Hash utilities

### Network Utilities
- URL parsing helpers
- IP address utilities
- Port validation

### Validation Utilities
- Input validation helpers
- Format validators
- Range checks

## Contributing

This crate is in early development and we welcome contributions! If you have ideas for useful utilities that would benefit the toolcraft ecosystem, please feel free to:

1. Open an issue to discuss the utility
2. Submit a pull request with your implementation
3. Add comprehensive tests and documentation

### Guidelines for New Utilities

When adding new utilities, please ensure:

1. **Zero or minimal dependencies**: Keep the crate lightweight
2. **Comprehensive documentation**: Include examples for each function
3. **Thorough testing**: Add unit tests for all edge cases
4. **Performance considerations**: Optimize for common use cases
5. **Ergonomic API**: Make functions easy to use and discover

## Example Structure (Future)

```rust
// String utilities
use toolcraft_utils::string::{to_snake_case, to_camel_case};

let snake = to_snake_case("HelloWorld"); // "hello_world"
let camel = to_camel_case("hello_world"); // "helloWorld"

// File utilities
use toolcraft_utils::fs::{ensure_dir, read_json};

ensure_dir("./config")?;
let config: MyConfig = read_json("./config/app.json")?;

// Time utilities
use toolcraft_utils::time::{format_duration, parse_duration};

let duration = parse_duration("1h 30m")?;
let formatted = format_duration(duration); // "1 hour 30 minutes"

// Validation utilities
use toolcraft_utils::validate::{is_valid_email, is_valid_url};

if is_valid_email("user@example.com") {
    // Process email
}
```

## Design Philosophy

1. **Simplicity**: Each utility should do one thing well
2. **Composability**: Utilities should work well together
3. **Performance**: Optimize for the common case
4. **Safety**: Prefer safe APIs, use unsafe only when necessary
5. **Clarity**: Clear naming and obvious behavior

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/code-serenade/toolcraft/blob/main/LICENSE) file for details.

## Links

- [Repository](https://github.com/code-serenade/toolcraft)
- [Documentation](https://docs.rs/toolcraft-utils)
- [Crates.io](https://crates.io/crates/toolcraft-utils)