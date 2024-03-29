//! Handling of `ArrayType` Schema objects

use openapiv3::{ArrayType, ReferenceOr, SchemaData, SchemaKind, Type};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::{sanitize_str_for_ident, ResolvedSchemaComponent};

// Resolves `Array` type Schema component
pub(super) fn resolve_schema_component_kind_array(
    _data: &SchemaData,
    array: &ArrayType,
) -> std::io::Result<ResolvedArrayType> {
    let tokens = if array.items.is_some() {
        let mut tokens = TokenStream::new();
        let items_schema = array.items.as_ref().unwrap();
        match items_schema {
            ReferenceOr::Reference { reference } => {
                let referred_type = reference.split('#').last().unwrap();
                let referred_type = referred_type.split("/").last().unwrap();
                let value_ident =
                    Ident::new(&sanitize_str_for_ident(referred_type), Span::call_site());

                tokens.extend(quote! { Vec<#value_ident> })
            }
            ReferenceOr::Item(s) => match &s.schema_kind {
                SchemaKind::Type(t) => match t {
                    Type::String(_) => tokens.extend(quote! { Vec<String> }),
                    Type::Integer(_) => tokens.extend(quote! { Vec<String> }),
                    _ => {
                        eprintln!("item_schema: {:#?}", items_schema);
                        todo!()
                    }
                },
                _ => {
                    eprintln!("item_schema: {:#?}", items_schema);
                    todo!()
                }
            },
        }
        tokens
    } else {
        TokenStream::new()
    };

    Ok(ResolvedArrayType { tokens })
}

pub(super) struct ResolvedArrayType {
    tokens: TokenStream,
}

impl ResolvedArrayType {
    pub(super) fn generate(
        self,
        ident: Ident,
        inner: bool,
    ) -> std::io::Result<ResolvedSchemaComponent> {
        let arr_tokens = self.tokens;
        let tokens = if inner {
            quote! {#arr_tokens }
        } else {
            quote! { pub struct #ident(#arr_tokens); }
        };

        Ok(ResolvedSchemaComponent {
            tokens,
            aux_tokens: TokenStream::new(),
        })
    }
}
