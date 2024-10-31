# SVB - Stream Variable Byte Compression

A high-performance integer compression library implemented in Rust that uses SIMD instructions to compress 32-bit unsigned integers into variable-length byte sequences. This implementation follows the Stream VByte algorithm described in ["Stream VByte: Faster Byte-Oriented Integer Compression"](https://arxiv.org/abs/1709.08990).

## Key Features

- **Variable-length Compression**: Efficiently compresses 32-bit integers into 1-4 bytes based on value magnitude
- **SIMD Optimization**: Uses x86_64 SIMD instructions for parallel processing of integer blocks
- **Dual Implementation**: Provides both scalar and SIMD variants for maximum compatibility
- **Zero-Copy Design**: Employs unsafe Rust for direct memory manipulation without unnecessary copying
- **Memory-efficient**: Uses compact control headers (2 bits per integer) to track compression ratios

## Technical Implementation

### Compression Format

The compressed data format consists of three sections:
1. Total Integer Count (usize bytes)
2. Control Headers (compressed size indicators)
3. Compressed Data (variable-length encoded integers)

### Control Headers

Each control header uses 2 bits to indicate compression level:
- `00` (0): 1-byte compression
- `01` (1): 2-byte compression
- `10` (2): 3-byte compression
- `11` (3): 4-byte compression (uncompressed)

Headers are packed four per byte, with bits ordered right-to-left within each byte.

### Performance Optimizations

1. **SIMD Processing**
   - Processes 8 integers simultaneously using 128-bit SIMD registers
   - Uses specialized x86_64 instructions for parallel comparisons and bit manipulation
   - Includes lookup tables for rapid compression length calculation

2. **Memory Management**
   - Direct memory manipulation using unsafe Rust for zero-copy operations
   - Efficient slice manipulation without unnecessary allocations
   - Careful pointer arithmetic for optimal performance

3. **Error Handling**
   - Comprehensive validation of input data
   - Robust error handling using the `anyhow` crate
   - Proper bounds checking during compression/decompression

## Implementation Details

### Core Components

1. **Scalar Implementation (`scl.rs`)**
   - Traditional single-integer processing
   - Fallback implementation for non-SIMD platforms
   - Clear, maintainable code for reference

2. **SIMD Implementation (`smd.rs`)**
   - Leverages x86_64 SIMD instructions
   - Processes multiple integers in parallel
   - Uses lookup tables for optimization

3. **Common Utilities (`lib.rs`)**
   - Shared constants and utilities
   - Header calculation functions
   - Type definitions and common traits

### Testing

- Comprehensive unit tests for both implementations
- Property-based testing with random input data
- Edge case validation
- Performance benchmarking comparisons

## Technical Achievements

1. **Memory Efficiency**
   - Optimal compression ratios for different integer ranges
   - Minimal memory overhead for control structures
   - Efficient handling of large datasets

2. **Performance**
   - SIMD parallelization for up to 8x throughput
   - Minimal branching in critical paths
   - Efficient bit manipulation techniques

3. **Code Quality**
   - Type-safe Rust implementation
   - Clear separation of concerns
   - Well-documented interfaces
   - Comprehensive error handling

## Usage

```rust
use svb::{smd, scl};

// SIMD-accelerated compression
let compressed = smd::enc(&integers)?;

// SIMD-accelerated decompression
let decompressed = smd::dec(&compressed)?;

// Scalar fallback compression
let compressed = scl::enc(&integers)?;

// Scalar fallback decompression
let decompressed = scl::dec(&compressed)?;
```

## Skills Demonstrated

- Advanced Rust programming
- SIMD optimization
- Low-level memory management
- Algorithm implementation
- Performance optimization
- Systems programming
- Technical documentation
- Test-driven development

## References

- [Stream VByte: Faster Byte-Oriented Integer Compression](https://arxiv.org/abs/1709.08990)
- [Original C Implementation](https://github.com/lemire/streamvbyte)


## Byte Layout

Bytes are organized as `total integer count`, followed by `control headers`, followed by the `compressed data`.

| Total Integer Count | Control Headers | Compressed Data |
|---------------------|-----------------|-----------------|
| `usize bytes`       | `bytes`         | `bytes`         |

> Byte layout for svb compression.

## Control header

`Two bits` indicate how much compression occurs in a 4-byte integer. 

The two bits are called a control header.

| Compression Size      | 1 byte | 2 bytes | 3 bytes | 4 bytes |
|-----------------------|--------|---------|---------|---------|
| Bit value             | `00`   | `01`    | `10`    | `11`    |
| Integer value of bits | 0      | 1       | 2       | 3       |

> Compression size represented as two bits. 



A header byte holds four control headers.

Within the header byte, bit values are indexed from right-to-left.

| Header Byte Index  | 3    | 2    | 1    | 0    |
|--------------------|------|------|------|------|
| Example bit values | `00` | `00` | `11` | `01` |

> A header byte containing four header values. The right-most two bits indicate compression size for the first integer.

## Development notes

Lemire blog: [Stream VByte: breaking new speed records for integer compression](https://lemire.me/blog/2017/09/27/stream-vbyte-breaking-new-speed-records-for-integer-compression/)

arXiv article: [Stream VByte: Faster Byte-Oriented Integer Compression](https://arxiv.org/abs/1709.08990)

Lemire C code: [streamvbyte](https://github.com/lemire/streamvbyte)
* Good overview of format in README.

Pierce Rust code: [stream-vbyte-rust](https://bitbucket.org/marshallpierce/stream-vbyte-rust/src/master/)


## File Tree
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
└── svb
    ├── Cargo.toml
    └── src
        ├── lib.rs
        ├── scl.rs
        └── smd.rs