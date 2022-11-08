use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, AngleBracketedGenericArguments, Data, DeriveInput, Fields,
     PathArguments, PathSegment, Type, TypePath,
};

#[proc_macro_derive(MatStruct)]
pub fn derive_matsave(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match get_fields(&input) {
        Ok(fields) =>  {
            let name = input.ident;
            let struct_name = name.to_string();
            let expanded = quote! {
                impl From<&#name> for matio_rs::MatStruct {
                    fn from(data: &#name) -> Self {
                        let mut builder = MatStruct::new(#struct_name);
                        #fields
                        builder.build().expect(&format!("failed to create struct {}",#struct_name))
                    }
                }
            };
            proc_macro::TokenStream::from(expanded)
        },
        Err(e) => e.into_compile_error().into()
    }
}

fn get_fields(input: &DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = input.ident.to_string();
    match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let mat_struct_field = fields
                    .named
                    .iter()
                    .map(|f| {
                        let name = &f.ident;
                        let field_name = name.as_ref().map(|x| x.to_string());
                        let ty = &f.ty;
                        let type_ident = get_type(ty)?;
                        Ok(match type_ident.to_string().as_str() {
                            "f64" | "f32" | "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" =>                         
                            quote_spanned! {f.span()=>
                                builder = <matio_rs::MatStructBuilder as matio_rs::Field<#ty>>::field(builder,#field_name,&data.#name)
                                        .expect(&format!("failed to create field {} in {}",#field_name,#struct_name));
                            },
                            _ =>                         
                            quote_spanned! {f.span()=>
                                let mat: matio_rs::MatStruct = (&data.#name).into();
                                builder = <matio_rs::MatStructBuilder as matio_rs::FieldMatObject<MatStruct>>::field(builder,#field_name,mat)
                                        .expect(&format!("failed to create field {} in {}",#field_name,#struct_name));
                            }
                        })

                    })
                    .collect::<syn::Result<Vec<_>>>()?;
                Ok(quote! {#(#mat_struct_field)*})
            }
            _ => Err(syn::Error::new_spanned(input, "expected named fields")),
        },
        _ => Err(syn::Error::new_spanned(input, "expected struct with named fields")),
    }
}

fn get_type(ty: &Type) -> syn::Result<Ident> {
    if let Type::Path(TypePath { path, .. }) = ty {
        match path.segments.first() {
            Some(PathSegment { ident, arguments }) => {
                if ident.to_string() == "Vec" {
                    match arguments {
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            args,
                            ..
                        }) => {
                            if let Some(syn::GenericArgument::Type(Type::Path(TypePath {
                                path,
                                ..
                            }))) = args.first()
                            {
                                if let Some(PathSegment { ident, .. }) = path.segments.first() {
                                    Ok(ident.clone())
                                } else {
                                    Err(syn::Error::new_spanned(ty, "Vec type arguments type is empty"))
                                }
                            } else {
                                Err(syn::Error::new_spanned(ty, "Vec type arguments is empty"))
                            }
                        }
                        _ => Err(syn::Error::new_spanned(ty, "Vec type is empty")),
                    }
                } else {
                    Ok(ident.clone())
                }
            }
            _ => Err(syn::Error::new_spanned(ty, "type path is empty")),
        }
    } else {
        Err(syn::Error::new_spanned(ty, "unsupported type is empty"))
    }
}
