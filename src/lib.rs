extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod macros;

/// Derives an `impl From<T>` for an enum that has an `Unknown` variant.
///
/// This macro provides an infallible conversion from an integer type `T`
/// by falling back to an `Enum::Unknown` variant if the integer does not
/// match any other variant.
///
/// # Requirements
///
/// 1. The enum must have a `#[repr(T)]` attribute with an integer type `T`.
/// 2. The enum MUST have a variant named `Unknown`.
///
/// # Example
///
/// ```rust
/// use macrokit::FromReprWithUnkown;
/// 
/// #[derive(Debug, PartialEq, FromReprWithUnkown)]
/// #[repr(u8)]
/// pub enum LegacyStatus {
///     Active = 0,
///     Inactive = 1,
///     Unknown, // Required fallback variant
/// }
///
/// // The macro generates an `impl From<u8> for LegacyStatus` block.
///
/// // Now you can use `.into()` or `From::from()` for conversion:
/// let status: LegacyStatus = 255u8.into();
/// assert_eq!(status, LegacyStatus::Unknown);
///
/// let status2 = LegacyStatus::from(0u8);
/// assert_eq!(status2, LegacyStatus::Active);
/// ```
#[proc_macro_derive(FromReprWithUnkown)]
pub fn from_repr_with_unknown_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input);
    macros::from_repr_with_unknown_derive_impl(&ast)
}

/// Derives a `from_repr` function for an enum.
///
/// This procedural macro generates an implementation block for the enum
/// that provides a `from_repr(value: T) -> Option<Self>` function,
/// where `T` is the integer type specified in the `#[repr(T)]` attribute.
///
/// # Requirements
///
/// The enum must have a `#[repr(...)]` attribute with an integer type,
/// for example `#[repr(u8)]`.
///
/// # Example
///
/// ```rust
/// use macrokit::FromReprAsOption;
///
/// #[derive(Debug, PartialEq, FromReprAsOption)] // We derive FromRepr here
/// #[repr(u16)]
/// pub enum Command {
///     Reset = 0x0100,
///     Read = 0x0200,
///     Write = 0x0300,
/// }
///
/// // The macro generates this implementation for you:
/// /*
/// impl Command {
///     pub fn from_repr(value: u16) -> Option<Self> {
///         match value {
///             v if v == Command::Reset as u16 => Some(Command::Reset),
///             v if v == Command::Read as u16 => Some(Command::Read),
///             v if v == Command::Write as u16 => Some(Command::Write),
///             _ => None,
///         }
///     }
/// }
/// */
///
/// // Now you can use the generated function:
/// assert_eq!(Command::from_repr(0x0200), Some(Command::Read));
/// assert_eq!(Command::from_repr(0x9999), None);
/// ```
#[proc_macro_derive(FromReprAsOption)]
pub fn from_repr_as_option_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input);
    macros::from_repr_as_option_derive_impl(&ast)
}