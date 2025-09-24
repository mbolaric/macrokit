mod doc_macros;
mod from_repr;

pub use from_repr::{from_repr_as_option_derive_impl, from_repr_with_unknown_derive_impl};

pub mod doc {
    pub use super::doc_macros::generate_enum_with_docs;
}
