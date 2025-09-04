use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident};

/// The core implementation logic for the `FromWithUnknown` derive macro.
///
/// This function takes the parsed abstract syntax tree (AST) of an enum
/// and generates the token stream for an `impl From<T> for Enum` block.
pub fn from_repr_with_unknown_derive_impl(ast: &DeriveInput) -> TokenStream {
    // Get the name of the enum we're implementing the trait for
    let enum_name = &ast.ident;

    // Find the integer type from the `#[repr(...)]` attribute (e.g., u8, u16)
    let repr_type = match get_repr_type(ast) {
        Some(t) => t,
        None => panic!("FromWithUnknown requires a #[repr(...)] attribute (e.g., #[repr(u8)])"),
    };

    // Ensure the input is an enum
    let variants = match &ast.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("FromWithUnknown can only be used on enums"),
    };

    // Generate a match arm for each variant, e.g., `v if v == MyEnum::Variant as u8 => MyEnum::Variant,`
    // We explicitly skip the `Unknown` variant to let it be the fallback case.
    let match_arms = variants.iter().filter_map(|variant| {
        let variant_name = &variant.ident;
        if variant_name == "Unknown" {
            None // Skip the `Unknown` variant
        } else {
            Some(quote! {
                v if v == #enum_name::#variant_name as #repr_type => #enum_name::#variant_name,
            })
        }
    });

    // Build the `impl From<...>` block
    let generated_impl = quote! {
        impl From<#repr_type> for #enum_name {
            fn from(value: #repr_type) -> Self {
                match value {
                    #( #match_arms )*
                    _ => #enum_name::Unknown,
                }
            }
        }
    };

    // Return the generated code
    generated_impl.into()
}

/// The core implementation logic for the `FromReprAsOption` derive macro.
///
/// This function takes the parsed abstract syntax tree (AST) of an enum
/// and generates the token stream for an `impl` block containing the
/// `from_repr` function.
pub fn from_repr_as_option_derive_impl(ast: &DeriveInput) -> TokenStream {
    let enum_name = &ast.ident;

    let repr_type = match get_repr_type(ast) {
        Some(t) => t,
        None => panic!("FromReprAsOption requires a #[repr(...)] attribute (e.g., #[repr(u8)])"),
    };

    let variants = match &ast.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("FromReprAsOption can only be used on enums"),
    };

    // Generate a match arm for each variant that returns Some(Self::Variant)
    let match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        quote! {
            // This creates arms like: `v if v == MyEnum::VariantA as u8 => Some(MyEnum::VariantA),`
            v if v == #enum_name::#variant_name as #repr_type => Some(#enum_name::#variant_name),
        }
    });

    // Build an `impl` block with a `from_repr` function that returns an Option<Self>
    let generated_impl = quote! {
        impl #enum_name {
            /// Creates an enum from its integer representation.
            ///
            /// Returns `None` if the integer does not match any variant.
            pub fn from_repr(value: #repr_type) -> Option<Self> {
                match value {
                    #( #match_arms )*
                    _ => None,
                }
            }
        }
    };

    generated_impl.into()
}

/// A helper function to find and parse the `#[repr(...)]` attribute from an enum.
///
/// It iterates through the attributes of the given `DeriveInput` (the AST of the enum)
/// and looks for one named `repr`. If found, it parses the argument inside the
/// parentheses as an identifier (e.g., `u8`, `u16`).
///
/// # Returns
///
/// `Some(Ident)` containing the representation type if found, otherwise `None`.
fn get_repr_type(ast: &DeriveInput) -> Option<Ident> {
    for attr in &ast.attrs {
        if attr.path().is_ident("repr") {
            return attr.parse_args().ok();
        }
    }
    None
}
