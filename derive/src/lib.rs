use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, Type, TypePath};

/// Derive macro that implements for a structure of type `T` the traits `MayBeFrom<&T>` for `Mat` and `MayBeInto<T>` for `Mat` and `&Mat`
#[proc_macro_derive(MatIO)]
pub fn derive_matio(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = &input.ident;
    match get_fields(&input) {
        Ok((maybe_from, maybe_into)) => {
            let expanded = quote! {

               impl<'a> matio_rs::MayBeFrom<&'a #struct_ident> for matio_rs::Mat<'a> {
                   fn maybe_from<S: Into<String>>(name: S, data: &'a #struct_ident) -> matio_rs::Result<Self> {
                       let mats: Vec<matio_rs::Mat> = vec![#(#maybe_from),*];
                       matio_rs::MayBeFrom::maybe_from(name, mats)
                   }
               }
               impl<'a> matio_rs::MayBeInto<#struct_ident> for matio_rs::Mat<'a> {
                   fn maybe_into(self) -> matio_rs::Result<#struct_ident> {
                       Ok(#struct_ident {
                           #(#maybe_into),*
                       })
                   }
               }
               impl<'a> matio_rs::MayBeInto<#struct_ident> for &matio_rs::Mat<'a> {
                   fn maybe_into(self) -> matio_rs::Result<#struct_ident> {
                       Ok(#struct_ident {
                           #(#maybe_into),*
                       })
                   }
               }
            };
            proc_macro::TokenStream::from(expanded)
        }
        Err(e) => e.into_compile_error().into(),
    }
}

fn get_fields(input: &DeriveInput) -> syn::Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let mut maybe_from = vec![];
                let mut maybe_into = vec![];
                for f in &fields.named {
                    let field_ident = &f.ident;
                    let field_name = field_ident.as_ref().map(|ident| ident.to_string());
                    let Type::Path(TypePath { path, .. }) = &f.ty else {
                         return Err(syn::Error::new_spanned(&f.ty, "unsupported type"))
                    };
                    if let Some(ty_ident) = path.get_ident() {
                        match ty_ident.to_string().as_str() {
                            "f64" | "f32" | "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32"
                            | "u64" => maybe_from.push(quote_spanned! {f.span()=>
                               matio_rs::Mat::maybe_from(#field_name, data.#field_ident)?
                            }),
                            _ => maybe_from.push(quote_spanned! {f.span()=>
                               matio_rs::Mat::maybe_from(#field_name, &data.#field_ident)?
                            }),
                        }
                    } else {
                        maybe_from.push(quote_spanned! {f.span()=>
                           matio_rs::Mat::maybe_from(#field_name, &data.#field_ident)?
                        });
                    }
                    maybe_into.push(quote_spanned! {f.span()=>
                       #field_ident: self.field(#field_name)?.get(0).unwrap().maybe_into()?
                    });
                }
                Ok((maybe_from, maybe_into))
            }

            _ => Err(syn::Error::new_spanned(input, "expected named fields")),
        },
        _ => Err(syn::Error::new_spanned(
            input,
            "expected struct with named fields",
        )),
    }
}
