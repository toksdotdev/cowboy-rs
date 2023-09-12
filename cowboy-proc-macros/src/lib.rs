use std::vec;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use quote::quote_spanned;
use syn::parse_macro_input;
use syn::spanned::Spanned;
use syn::Attribute;
use syn::Data;
use syn::DeriveInput;
use syn::Error;
use syn::Expr;
use syn::ExprLit;
use syn::Fields;
use syn::Lit;
use syn::Variant;

#[proc_macro_derive(Characteristic, attributes(characteristic))]
pub fn derive_characteristic(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match extract_variant_and_uuid_value("characteristic", &input.data, parse_string_attribute) {
        Err(err) => err,
        Ok((variants, characteristics)) => {
            let enum_name = &input.ident;
            let left_side_for_match = prepare_left_side_for_match_statement(&variants);
            TokenStream::from(quote! {
                impl #enum_name {
                    /// Get the characteristic UUID for a given characteristic.
                    pub fn characteristic(&self) -> uuid::Uuid {
                        match self {
                            #(Self::#left_side_for_match => uuid::uuid!(#characteristics)),*
                        }
                    }
                }
            })
        }
    }
}

#[proc_macro_derive(Service, attributes(service))]
pub fn derive_service(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match extract_variant_and_uuid_value("service", &input.data, parse_string_attribute) {
        Err(err) => err,
        Ok((variants, services)) => {
            let enum_name = &input.ident;
            let variant_name = variants.iter().map(|variant| &variant.ident);
            let left_side_for_match = prepare_left_side_for_match_statement(&variants);

            TokenStream::from(quote! {
                impl #enum_name {
                    /// Get the service UUID for a given service.
                    pub fn service(&self) -> uuid::Uuid {
                        match self {
                            #(Self::#left_side_for_match => uuid::uuid!(#services)),*
                        }
                    }

                    /// Get the characteristic UUID for a given service.
                    pub fn characteristic(&self) -> uuid::Uuid {
                        match self {
                            #(Self::#variant_name(characteristic) => characteristic.characteristic()),*
                        }
                    }
                }
            })
        }
    }
}

#[proc_macro_derive(Command, attributes(mode))]
pub fn derive_command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match extract_variant_and_uuid_value("mode", &input.data, parse_mode_attribute) {
        Err(err) => err,
        Ok((variants, mode)) => {
            let enum_name = &input.ident;
            let left_side_for_match = prepare_left_side_for_match_statement(&variants);
            TokenStream::from(quote! {
                impl #enum_name {
                    /// Get the write mode for a given command.
                    pub fn mode(&self) -> crate::CmdMode {
                        match self {
                            #(Self::#left_side_for_match => crate::CmdMode::from_str(#mode).unwrap()),*
                        }
                    }
                }
            })
        }
    }
}

fn extract_variant_and_uuid_value<T>(
    property: &str,
    data: &Data,
    parse_attribute: impl FnOnce(&Attribute) -> Result<Option<T>, Error> + Copy,
) -> Result<(Vec<Variant>, Vec<T>), TokenStream> {
    let mut variants = vec![];
    let mut values = vec![];
    let mut errs = TokenStream::new();
    let Data::Enum(enum_data) = data else {
        return Err(TokenStream::from(
            Error::new(Span::call_site(), "expected enum").into_compile_error(),
        ));
    };

    for variant in &enum_data.variants {
        let Some(attr) = variant
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident(property))
        else {
            let err = Error::new(
                Span::call_site(),
                format!("`{property}` attribute not found"),
            );
            errs.extend(TokenStream::from(err.into_compile_error()));
            continue;
        };

        match parse_attribute(attr) {
            Err(e) => errs.extend(TokenStream::from(e.to_compile_error())),
            Ok(Some(value)) => {
                variants.push(variant.clone());
                values.push(value);
            }
            Ok(None) => {}
        }
    }

    if !errs.is_empty() {
        return Err(errs);
    }

    Ok((variants, values))
}

fn parse_string_attribute(attribute: &Attribute) -> Result<Option<String>, Error> {
    Ok(match attribute.parse_args()? {
        Expr::Lit(ExprLit {
            lit: Lit::Str(ref s),
            ..
        }) => Some(s.value()),
        _ => None,
    })
}

fn parse_mode_attribute(attribute: &Attribute) -> Result<Option<&'static str>, Error> {
    let mut mode = None;
    attribute.parse_nested_meta(|meta| {
        mode = match meta.path {
            path if path.is_ident("write") => Some("write"),
            path if path.is_ident("write_with_response") => Some("write_with_response"),
            _ => None,
        };

        Ok(())
    })?;

    Ok(mode)
}

fn prepare_left_side_for_match_statement(variants: &[Variant]) -> Vec<proc_macro2::TokenStream> {
    variants
        .into_iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            match &variant.fields {
                Fields::Unnamed(_) => {
                    quote_spanned! {variant.span() => #variant_ident (..) }
                }
                Fields::Unit => quote_spanned! { variant.span() => #variant_ident },
                Fields::Named(_) => quote_spanned! {variant.span() => #variant_ident {..} },
            }
        })
        .collect()
}
