# Tondi Opcode Utilities

A Rust library providing utilities for working with Tondi blockchain opcodes. This module extends the functionality of the `tondi-txscript` crate with additional helper functions and macros for opcode manipulation and conversion.

## Features

- **Push Opcode Detection**: Functions to identify push bytes and push data opcodes
- **String Conversion**: Macro-generated function for converting string representations to opcode values
- **Comprehensive Opcode Support**: Covers all standard Tondi opcodes including data operations, stack manipulation, arithmetic, and cryptographic functions
- **Alias Support**: Multiple string representations for common opcodes (e.g., "Op0", "FALSE", "TRUE")

## Installation

Add this dependency to your `Cargo.toml`:

```toml
[dependencies]
tondi-opcode-utils = "0.17.0"
```

## Dependencies

This library depends on:
- `tondi-txscript = "0.17.0"` - Core Tondi transaction script functionality

## Usage

### Basic Functions

```rust
use tondi_opcode_utils::{is_push_bytes, is_push_data};

// Check if an opcode is a push bytes opcode (0x01-0x4b)
let is_push = is_push_bytes(0x01); // true
let is_not_push = is_push_bytes(0x61); // false (OpNop)

// Check if an opcode is a push data opcode
let is_push_data = is_push_data(0x4c); // true (OP_PUSH_DATA1)
```

### String to Opcode Conversion

```rust
use tondi_opcode_utils::from_str;

// Convert string representations to opcode values
let opcode = from_str("Op0").unwrap(); // Returns 0x00
let opcode = from_str("TRUE").unwrap(); // Returns 0x51
let opcode = from_str("OpFalse").unwrap(); // Returns 0x00

// Supports both prefixed and non-prefixed formats
let opcode1 = from_str("OpAdd").unwrap(); // Returns 0x93
let opcode2 = from_str("Add").unwrap(); // Returns 0x93
```

### Available Opcode Categories

#### Data Operations
- **Push Operations**: `OpData1` through `OpData75`, `OP_PUSH_DATA1`, `OP_PUSH_DATA2`, `OP_PUSH_DATA4`
- **Constants**: `OpFalse` (0x00), `OpTrue` (0x51), `Op1Negate` (0x4f)

#### Stack Manipulation
- **Duplication**: `OpDup`, `Op2Dup`, `Op3Dup`, `OpIfDup`
- **Movement**: `OpDrop`, `Op2Drop`, `OpNip`, `OpOver`, `OpPick`, `OpRoll`, `OpRot`, `OpSwap`, `OpTuck`
- **Stack Info**: `OpDepth`, `OpSize`

#### Arithmetic Operations
- **Basic Math**: `OpAdd`, `OpSub`, `OpMul`, `OpDiv`, `OpMod`
- **Bitwise**: `OpAnd`, `OpOr`, `OpXor`, `OpInvert`
- **Comparison**: `OpEqual`, `OpNumEqual`, `OpLessThan`, `OpGreaterThan`
- **Increment/Decrement**: `Op1Add`, `Op1Sub`, `Op2Mul`, `Op2Div`

#### Control Flow
- **Conditionals**: `OpIf`, `OpNotIf`, `OpElse`, `OpEndIf`
- **Verification**: `OpVerify`, `OpEqualVerify`, `OpNumEqualVerify`
- **Time Locks**: `OpCheckLockTimeVerify`, `OpCheckSequenceVerify`

#### Cryptographic Functions
- **Hashing**: `OpSHA256`, `OpBlake3`
- **Signatures**: `OpCheckSig`, `OpCheckSigVerify`, `OpCheckMultiSig`, `OpCheckMultiSigVerify`
- **ECDSA**: `OpCheckSigECDSA`, `OpCheckMultiSigECDSA`

## API Reference

### Functions

#### `is_push_bytes(code: u8) -> bool`
Determines if the given opcode is a push bytes opcode (0x01-0x4b).

#### `is_push_data(code: u8) -> bool`
Determines if the given opcode is one of the push data opcodes (0x4c, 0x4d, 0x4e).

#### `from_str(s: &str) -> Result<u8, ()>`
Converts a string representation of an opcode to its byte value. Returns `Err(())` if the string is not recognized.

### Constants

- `OP_PUSH_DATA1`: 0x4c
- `OP_PUSH_DATA2`: 0x4d  
- `OP_PUSH_DATA4`: 0x4e

## Implementation Notes

- The library uses workarounds to determine opcode classes due to limited access to internal `Opcode` struct values
- String conversion supports multiple aliases for common opcodes
- All opcodes are defined as constants with their hexadecimal values
- Reserved and undefined opcodes are marked appropriately

## License

This project is licensed under the terms specified in the LICENSE file.

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## Related Projects

- [Tondi](https://github.com/your-org/Tondi) - Core Tondi blockchain implementation
- [tondi-txscript](https://github.com/your-org/Tondi/tree/main/crypto/txscript) - Tondi transaction script functionality
