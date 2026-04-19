
use proc_macro::{TokenStream};



mod pretty;
mod primitive;
mod bit;
use bit::{bitstruct, bitenum};

mod guid;

#[proc_macro_derive(FromGuid, attributes(guid))]
pub fn derive_from_guid(input: TokenStream) -> TokenStream {
    guid::expand(input)
}

/// Implements `Display` for a struct with named fields.
///
/// This function:
/// 1. Iterates over each field
/// 2. Converts field names into a human-readable format
/// 3. Detects whether the field is:
///    - Option<T>
///    - Vec<T>
///    - Struct vs primitive
/// 4. Emits different formatting logic depending on the type
#[proc_macro_derive(DisplayPretty)]
pub fn derive_display_pretty(input: TokenStream) -> TokenStream {
    pretty::expand(input)
}







/// Derives bitflag-style behavior for an enum.
///
/// This macro maps each enum variant to a single bit and generates a companion
/// `<EnumName>Set` type for working with combinations of flags.
///
/// # Attributes
///
/// - `#[bitenum(<int>)]`  
///   *(optional)* Specifies the backing integer type (currently informational; defaults to `u32` in generated code).
///
/// - `#[bit(n)]`  
///   *(optional)* Explicitly assigns a bit position to a variant.
///
/// - `= n` (discriminant)  
///   Also sets the bit position and advances subsequent auto-assigned bits.
///
/// # Generated Items
///
/// For an enum `Foo`, this macro generates:
///
/// - `struct FooSet(u32)`
/// - `impl Foo { fn bit(self) -> u32 }`
/// - `impl FooSet` with:
///   - `empty()`, `all()`, `from_bits()`, `bits()`
///   - `contains()`, `insert()`
/// - Bitwise operators (`|`, `&`)
///
/// # Example
///
/// ```rust
/// use remata_macros::BitEnum;
///
/// #[derive(BitEnum, Debug, Clone, Copy)]
/// pub enum Flag {
///     A = 0,
///     B,
///     #[bit(5)]
///     C,
/// }
///
/// let flags = Flag::A | Flag::C;
///
/// assert!(flags.contains(Flag::A));
/// assert_eq!(flags.bits(), (1 << 0) | (1 << 5));
/// ```
///
/// # Notes
///
/// - Only **unit variants** are supported.
/// - Bit positions must not overlap.
/// - Generated code uses `u32` internally.
#[proc_macro_derive(BitEnum, attributes(bitenum, bit))]
pub fn derive_bitenum(input: TokenStream) -> TokenStream {
    bitenum::expand(input)
}


/// Derives bitfield packing/unpacking for structs of boolean fields.
///
/// This macro maps each `bool` field to a single bit in an integer type,
/// generating conversions to and from that integer.
///
/// # Attributes
///
/// - `#[bitstruct(<int>)]`  
///   *(optional)* Specifies the backing integer type (default: `u8`).
///
/// - `#[bit(n)]`  
///   *(optional)* Explicitly assigns a bit position to a field.
///
/// # Generated Implementations
///
/// - `impl From<Backing> for Struct`
/// - `impl From<Struct> for Backing`
/// - `fn from_bits(value) -> Self`
/// - `fn bits(self) -> Backing`
///
/// # Example
///
/// ```rust
/// use your_crate::BitStruct;
///
/// #[derive(BitStruct, Debug, Default, Clone, Copy)]
/// #[bitstruct(u8)]
/// pub struct LensType {
///     pub mf: bool,
///     pub d: bool,
///     pub g: bool,
///     pub vr: bool,
/// }
///
/// let raw: u8 = 0b1010;
/// let lens = LensType::from(raw);
///
/// assert!(lens.d);
/// assert!(lens.vr);
/// assert_eq!(lens.bits(), raw);
/// ```
///
/// # Custom Bit Positions
///
/// ```rust
/// #[derive(BitStruct)]
/// #[bitstruct(u8)]
/// struct Example {
///     #[bit(0)] a: bool,
///     #[bit(7)] b: bool,
/// }
/// ```
///
/// # Notes
///
/// - Only `bool` fields are supported.
/// - Field order determines bit order unless overridden.
/// - No validation is performed for overlapping bits.
#[proc_macro_derive(BitStruct, attributes(bitstruct, bit))]
pub fn derive_bitstruct(input: TokenStream) -> TokenStream {
    bitstruct::expand(input)
}

/// Derives conversion from primitive integers to enum variants.
///
/// This macro generates `TryFrom<primitive>` implementations for an enum,
/// using `#[value = N]` attributes to map integer values to variants.
///
/// # Attributes
///
/// - `#[value = N]`  
///   Assigns a numeric value to a variant.
///
/// # Unknown Handling
///
/// If the enum contains a tuple variant (e.g. `Unknown(u8)`), it will be used
/// as a fallback for unmatched values.
///
/// Otherwise, conversion will return `Err(())`.
///
/// # Generated Implementations
///
/// - `impl TryFrom<u8/u16/u32/...> for Enum`
///
/// # Example (strict)
///
/// ```rust
/// use your_crate::FromPrimitive;
/// use core::convert::TryFrom;
///
/// #[derive(FromPrimitive, Debug, PartialEq)]
/// enum Mode {
///     #[value = 0]
///     Off,
///     #[value = 1]
///     On,
/// }
///
/// assert_eq!(Mode::try_from(1), Ok(Mode::On));
/// assert!(Mode::try_from(2).is_err());
/// ```
///
/// # Example (with unknown fallback)
///
/// ```rust
/// #[derive(FromPrimitive, Debug, PartialEq)]
/// enum Mode {
///     #[value = 0]
///     Off,
///     #[value = 1]
///     On,
///     Unknown(u8),
/// }
///
/// assert_eq!(Mode::try_from(2), Ok(Mode::Unknown(2)));
/// ```
///
/// # Notes
///
/// - Only variants with `#[value = N]` are matched explicitly.
/// - A single tuple variant is treated as the fallback.
/// - The tuple type should match the primitive type for best results.
/// - Matching is performed using `v as i128`.
#[proc_macro_derive(FromPrimitive, attributes(value))]
pub fn derive_from_primitive(input: TokenStream) -> TokenStream {
    primitive::expand(input)
}