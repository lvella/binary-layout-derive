extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

fn find_repr(attrs: &[syn::Attribute]) -> Option<syn::Type> {
    let mut res = None;

    for attr in attrs {
        if matches!(attr.style, syn::AttrStyle::Outer) {
            if let syn::Meta::List(meta_list) = &attr.meta {
                if meta_list.path.is_ident("repr") {
                    let attr: syn::Type = syn::parse2(meta_list.tokens.clone()).unwrap();

                    if res.is_some() {
                        panic!("Only ony repr attribute is allowed");
                    }
                    res = Some(attr);
                }
            }
        }
    }

    res
}

#[proc_macro_derive(LayoutAsEnumRepr)]
pub fn layout_as_enum_repr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let repr_type = find_repr(&input.attrs).expect("No repr attribute found");

    match input.data {
        Data::Enum(_) => {
            let expanded = quote! {
                impl #impl_generics binary_layout::LayoutAs<#repr_type> for #name #ty_generics #where_clause {
                    type ReadError = <Self as std::convert::TryFrom<#repr_type>>::Error;
                    type WriteError = std::convert::Infallible;

                    fn try_read(v: #repr_type) -> Result<Self, Self::ReadError> {
                        Self::try_from(v)
                    }

                    fn try_write(v: Self) -> Result<#repr_type, Self::WriteError> {
                        Ok(v.into())
                    }
                }
            };

            TokenStream::from(expanded)
        },
        _ => panic!("LayoutAsEnumRepr can only be applied to enums"),
    }
}
