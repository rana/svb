# svb
Stream variable byte compression in Rust.

Compresses 32-bit unsigned integers to an array of bytes.

A 32-bit integer compresses to 1-byte, 2-bytes, 3-bytes, or 4-bytes.

Four integers are compressed at a time with SIMD instructions.

## Byte Layout

Bytes are organized as `total integer count`, followed by `control headers`, followed by the `compressed data`.

| Total Integer Count | Control Headers | Compressed Data |
|---------------------|-----------------|-----------------|
| `usize bytes`       | `bytes`         | `bytes`         |

> Byte layout for svb compression.

## Control header

`Two bits` indicate how much compression occurs in a 4-byte integer. 

The two bits are called a control header.

| Compression Size | 1 byte | 2 bytes | 3 bytes | 4 bytes |
|------------------|--------|---------|---------|---------|
| Bit value        | `00`   | `01`    | `10`    | `11`    |
| Integer value    | 0      | 1       | 2       | 3       |

> Compression size represented as two bits. 



A header byte holds four control headers.

Within the header byte, bit values are indexed from right-to-left.

| Header Byte Index  | 3    | 2    | 1    | 0    |
|--------------------|------|------|------|------|
| Example bit values | `00` | `00` | `11` | `01` |

> A header byte containing four header values. The right-most two bits indicate compression size for the first integer.

## Development notes

Lemire blog: [Stream VByte: breaking new speed records for integer compression](https://lemire.me/blog/2017/09/27/stream-vbyte-breaking-new-speed-records-for-integer-compression/)

asXiv article: [Stream VByte: Faster Byte-Oriented Integer Compression](https://arxiv.org/abs/1709.08990)

Lemire C code: [streamvbyte](https://github.com/lemire/streamvbyte)
* Good overview of format in README.

Pierce Rust code: [stream-vbyte-rust](https://bitbucket.org/marshallpierce/stream-vbyte-rust/src/master/)

