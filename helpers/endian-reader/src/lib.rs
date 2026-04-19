//! # Endian Utilities Crate
//!
//! This crate provides a lightweight, zero-dependency set of utilities for
//! reading multi-byte numeric values from raw byte buffers with explicit
//! control over byte order (endianness).
//!
//! It is primarily designed for low-level binary parsing tasks such as:
//!
//! - Image formats (e.g., EXIF/TIFF parsing)
//! - File format decoders
//! - Network protocol parsing
//! - Embedded systems or binary data inspection tools
//!
//! ## 📦 Core Concept
//!
//! The central type in this crate is [`Endian`], which represents the byte
//! order used when interpreting raw bytes:
//!
//! - [`Endian::Little`] → least significant byte first
//! - [`Endian::Big`] → most significant byte first
//!
//! All numeric reading operations are performed relative to a chosen
//! endianness, ensuring consistent interpretation across platforms.
//!
//! ## 🔢 Supported Data Types
//!
//! This crate provides direct helpers for reading primitive numeric types:
//!
//! ### Unsigned integers
//! - `u16`, `u32`, `u64`, `u128`
//!
//! ### Signed integers
//! - `i16`, `i32`, `i64`, `i128`
//!
//! ### Floating-point values
//! - `f32`, `f64` (IEEE 754)
//!
//! All reads operate on a byte slice (`&[u8]`) and an explicit offset.
//!
//! ## ⚙️ Design Goals
//!
//! - **Zero dependencies**
//! - **Explicit correctness over abstraction**
//! - **Minimal API surface**
//! - **Fast, direct byte interpretation**
//!
//! This crate intentionally avoids:
//!
//! - Streaming abstractions
//! - Unsafe pointer casting
//! - Heap allocation
//!
//! ## ⚠️ Safety Notes
//!
//! - All read methods assume the buffer contains enough bytes
//!   at the specified offset.
//! - Out-of-bounds access will cause a panic.
//! - This design prioritizes performance and simplicity.
//!
//! For untrusted input, wrap calls with bounds checks or extend the API
//! with safe `Option` / `Result`-based variants.
//!
//! ## 📌 Example
//!
//! ```rust
//! use endian_reader::Endian;
//!
//! let buf = [0x01, 0x00, 0x00, 0x00];
//! let endian = Endian::Little;
//!
//! let value = endian.read_u32(&buf, 0);
//! assert_eq!(value, 1);
//! ```
//!
//! ## 🧠 Use Cases
//!
//! This crate is especially useful in:
//!
//! - EXIF / TIFF metadata parsing
//! - Binary file inspection tools
//! - Custom serialization formats
//! - Reverse engineering file structures
//!
//! ## 🔮 Possible Extensions
//!
//! Future versions may include:
//!
//! - Safe read APIs (`Result` / `Option`)
//! - Slice-based reading helpers
//! - Bit-level utilities
//! - Iterator-based parsers
//! - Alignment-aware reads


/// Represents byte order (endianness) used when interpreting raw binary data.
///
/// Endianness determines how multi-byte values are stored in memory:
///
/// - [`Endian::Little`] – Least significant byte first
/// - [`Endian::Big`] – Most significant byte first
///
/// This enum provides helper methods to read numeric values from byte slices
/// according to the specified byte order.
///
/// ## Example
///
/// ```rust
/// let buf = [0x01, 0x00];
/// let value = Endian::Little.read_u16(&buf, 0);
/// assert_eq!(value, 1);
/// ```
pub enum Endian {
    /// Little-endian byte order (least significant byte first)
    Little,

    /// Big-endian byte order (most significant byte first)
    Big,
}

impl Endian {
    /// Reads an unsigned 16-bit integer (`u16`) from the buffer at the given offset.
    pub fn read_u16(&self, buf: &[u8], offset: usize) -> u16 {
        let bytes = [buf[offset], buf[offset + 1]];
        match self {
            Self::Little => u16::from_le_bytes(bytes),
            Self::Big => u16::from_be_bytes(bytes),
        }
    }

    /// Reads an unsigned 32-bit integer (`u32`) from the buffer at the given offset.
    pub fn read_u32(&self, buf: &[u8], offset: usize) -> u32 {
        let bytes = [
            buf[offset],
            buf[offset + 1],
            buf[offset + 2],
            buf[offset + 3],
        ];
        match self {
            Self::Little => u32::from_le_bytes(bytes),
            Self::Big => u32::from_be_bytes(bytes),
        }
    }

    /// Reads an unsigned 64-bit integer (`u64`) from the buffer at the given offset.
    pub fn read_u64(&self, buf: &[u8], offset: usize) -> u64 {
        let bytes = [
            buf[offset],
            buf[offset + 1],
            buf[offset + 2],
            buf[offset + 3],
            buf[offset + 4],
            buf[offset + 5],
            buf[offset + 6],
            buf[offset + 7],
        ];
        match self {
            Self::Little => u64::from_le_bytes(bytes),
            Self::Big => u64::from_be_bytes(bytes),
        }
    }

    /// Reads an unsigned 128-bit integer (`u128`) from the buffer at the given offset.
    pub fn read_u128(&self, buf: &[u8], offset: usize) -> u128 {
        let bytes = [
            buf[offset],
            buf[offset + 1],
            buf[offset + 2],
            buf[offset + 3],
            buf[offset + 4],
            buf[offset + 5],
            buf[offset + 6],
            buf[offset + 7],
            buf[offset + 8],
            buf[offset + 9],
            buf[offset + 10],
            buf[offset + 11],
            buf[offset + 12],
            buf[offset + 13],
            buf[offset + 14],
            buf[offset + 15],
        ];
        match self {
            Self::Little => u128::from_le_bytes(bytes),
            Self::Big => u128::from_be_bytes(bytes),
        }
    }

    /// Reads a signed 16-bit integer (`i16`) from the buffer at the given offset.
    pub fn read_i16(&self, buf: &[u8], offset: usize) -> i16 {
        let bytes = [buf[offset], buf[offset + 1]];
        match self {
            Self::Little => i16::from_le_bytes(bytes),
            Self::Big => i16::from_be_bytes(bytes),
        }
    }

    /// Reads a signed 32-bit integer (`i32`) from the buffer at the given offset.
    pub fn read_i32(&self, buf: &[u8], offset: usize) -> i32 {
        let bytes = [
            buf[offset],
            buf[offset + 1],
            buf[offset + 2],
            buf[offset + 3],
        ];
        match self {
            Self::Little => i32::from_le_bytes(bytes),
            Self::Big => i32::from_be_bytes(bytes),
        }
    }

    /// Reads a signed 64-bit integer (`i64`) from the buffer at the given offset.
    pub fn read_i64(&self, buf: &[u8], offset: usize) -> i64 {
        let bytes = [
            buf[offset],
            buf[offset + 1],
            buf[offset + 2],
            buf[offset + 3],
            buf[offset + 4],
            buf[offset + 5],
            buf[offset + 6],
            buf[offset + 7],
        ];
        match self {
            Self::Little => i64::from_le_bytes(bytes),
            Self::Big => i64::from_be_bytes(bytes),
        }
    }

    /// Reads a signed 128-bit integer (`i128`) from the buffer at the given offset.
    pub fn read_i128(&self, buf: &[u8], offset: usize) -> i128 {
        let bytes = [
            buf[offset],
            buf[offset + 1],
            buf[offset + 2],
            buf[offset + 3],
            buf[offset + 4],
            buf[offset + 5],
            buf[offset + 6],
            buf[offset + 7],
            buf[offset + 8],
            buf[offset + 9],
            buf[offset + 10],
            buf[offset + 11],
            buf[offset + 12],
            buf[offset + 13],
            buf[offset + 14],
            buf[offset + 15],
        ];
        match self {
            Self::Little => i128::from_le_bytes(bytes),
            Self::Big => i128::from_be_bytes(bytes),
        }
    }

    /// Reads a 32-bit floating point value (`f32`) from the buffer at the given offset.
    ///
    /// The bytes are interpreted according to IEEE 754.
    pub fn read_f32(&self, buf: &[u8], offset: usize) -> f32 {
        let bytes = [
            buf[offset],
            buf[offset + 1],
            buf[offset + 2],
            buf[offset + 3],
        ];
        match self {
            Self::Little => f32::from_le_bytes(bytes),
            Self::Big => f32::from_be_bytes(bytes),
        }
    }

    /// Reads a 64-bit floating point value (`f64`) from the buffer at the given offset.
    ///
    /// The bytes are interpreted according to IEEE 754.
    pub fn read_f64(&self, buf: &[u8], offset: usize) -> f64 {
        let bytes = [
            buf[offset],
            buf[offset + 1],
            buf[offset + 2],
            buf[offset + 3],
            buf[offset + 4],
            buf[offset + 5],
            buf[offset + 6],
            buf[offset + 7],
        ];
        match self {
            Self::Little => f64::from_le_bytes(bytes),
            Self::Big => f64::from_be_bytes(bytes),
        }
    }
}