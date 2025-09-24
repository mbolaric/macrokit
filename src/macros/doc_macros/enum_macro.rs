use proc_macro::Span;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Ident};
use syn::{Expr, ItemEnum};

/// Extracts the integer type from a `#[repr(...)]` attribute.
///
/// This function iterates through the attributes of an enum and looks for `#[repr(T)]`,
/// where `T` is a primitive integer type (e.g., `u8`, `i32`).
///
/// # Arguments
///
/// * `attrs` - A slice of `syn::Attribute` to search.
///
/// # Returns
///
/// An `Option<Ident>` containing the identifier of the integer type if found, otherwise `None`.
fn extract_repr(attrs: &[Attribute]) -> Option<Ident> {
    for attr in attrs {
        if attr.path().is_ident("repr") {
            let mut found: Option<Ident> = None;
            // Walk through #[repr(...)] inner items
            let _ = attr.parse_nested_meta(|meta| {
                let path = &meta.path;
                if path.is_ident("u8")
                    || path.is_ident("u16")
                    || path.is_ident("u32")
                    || path.is_ident("u64")
                    || path.is_ident("u128")
                    || path.is_ident("i8")
                    || path.is_ident("i16")
                    || path.is_ident("i32")
                    || path.is_ident("i64")
                    || path.is_ident("i128")
                    || path.is_ident("usize")
                    || path.is_ident("isize")
                {
                    // make a new Ident using the same span
                    found = Some(Ident::new(
                        &path.segments[0].ident.to_string(),
                        Span::call_site().into(),
                    ));
                }
                Ok(())
            });
            if found.is_some() {
                return found;
            }
        }
    }
    None
}

/// Generates an enum with documentation comments that include the hexadecimal and decimal
/// values of each variant.
///
/// This function processes an `ItemEnum` and reconstructs it with added `#[doc = "..."]`
/// attributes for each variant. The documentation format is `VariantName = 0x... (...)`.
///
/// # Arguments
///
/// * `input` - The input `ItemEnum` to process.
///
/// # Returns
///
/// A `TokenStream` representing the modified enum.
pub fn generate_enum_with_docs(input: ItemEnum) -> TokenStream {
    let enum_ident = &input.ident;
    let vis = &input.vis;
    let generics = &input.generics;
    let attrs = &input.attrs;

    // Extract the repr type or default to isize.
    let repr_ty = extract_repr(&input.attrs)
        .unwrap_or_else(|| syn::Ident::new("isize", proc_macro::Span::call_site().into()));

    let mut next_val: Option<i128> = Some(0);
    let mut variants = Vec::new();

    for variant in input.variants {
        let ident = &variant.ident;
        let mut val: Option<i128> = None;

        // If the variant has an explicit discriminant, parse it.
        if let Some((_, expr)) = &variant.discriminant {
            if let Expr::Lit(expr_lit) = expr {
                if let syn::Lit::Int(lit_int) = &expr_lit.lit {
                    if let Ok(v) = lit_int.base10_parse::<i128>() {
                        next_val = Some(v + 1);
                        val = Some(v);
                    }
                }
            }
        }

        // If no explicit discriminant, use the auto-incremented value.
        if val.is_none() {
            if let Some(v) = next_val {
                val = Some(v);
                next_val = Some(v + 1);
            }
        }

        // Build the new doc string with hex and decimal values.
        let doc = if let Some(v) = val {
            let hex = format!("{:#X}", v);
            let dec = format!("{}", v);
            format!("{} = {} ({})", ident, hex, dec)
        } else {
            format!("{} (unknown)", ident)
        };

        let attrs = &variant.attrs;
        if let Some(v) = val {
            variants.push(quote! {
                #( #attrs )*
                #[doc = #doc]
                #ident = #v as #repr_ty
            });
        } else {
            variants.push(quote! {
                #( #attrs )*
                #[doc = #doc]
                #ident
            });
        }
    }

    // Reconstruct the enum with the new variants.
    let expanded = quote! {
        #( #attrs )*
        #vis enum #enum_ident #generics {
            #( #variants, )*
        }
    };

    expanded.into()
}
