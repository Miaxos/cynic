use proc_macro2::TokenStream;

use super::Ident;

/// A GenericParameter, which struct fields may or may not require.
///
/// We use these for input struct or enum arguments. We're expecting users
/// to define these types but we need to take them as arguements so we make
/// our argument structs generic, and apply constraints to make sure the
/// correct types are passed in.
pub struct GenericParameter {
    pub name: Ident,
    pub constraint: GenericConstraint,
}

impl GenericParameter {
    pub fn to_tokens(&self, path_to_markers: syn::Path) -> TokenStream {
        use quote::quote;

        let name = &self.name;
        let constraint = self.constraint.to_tokens(path_to_markers);

        quote! {
            #name: #constraint
        }
    }
}

/// Our generic parameters need constraints - this enum specifies what they
/// should be.
pub enum GenericConstraint {
    /// An enum type constraint: `where T: Enum<SomeTypeLockMarkerStruct>
    Enum(Ident),
    /// An input object constraint: `where T: InputObject<SomeInputObjectMarkerStruct>
    InputObject(Ident),
}

impl GenericConstraint {
    fn to_tokens(&self, path_to_markers: syn::Path) -> TokenStream {
        use quote::quote;

        match self {
            GenericConstraint::Enum(ident) => {
                quote! { ::cynic::Enum<#path_to_markers::#ident> }
            }
            GenericConstraint::InputObject(ident) => {
                quote! { ::cynic::InputObject<#path_to_markers::#ident> }
            }
        }
    }
}
