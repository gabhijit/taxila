//! Functions related to resolving schema components as Rust 'struct's or 'enum's

#[allow(unused)]
use openapiv3::*;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

// Returns a TokenStream corresponding to the schema component.
//
// Typically this function will be called by `Generator`.
pub(crate) fn resolve_schema_component(
    name: &str,
    schema: &Schema,
) -> std::io::Result<TokenStream> {
    match &schema.schema_kind {
        SchemaKind::Type(_) => resolve_schema_type_component(name, schema, false),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not implemented yet!",
        )),
    }
}

// Resolves the `Type(Type)` variant of `SchemaKind`
//
// Basically calls the resolver for each of the variant of the `Type`
//
// TODO: AnyOf, AllOf, OneOf
fn resolve_schema_type_component(
    name: &str,
    schema: &Schema,
    inner: bool,
) -> std::io::Result<TokenStream> {
    if let SchemaKind::Type(ref t) = schema.schema_kind {
        match t {
            Type::String(ref s) => {
                resolve_schema_component_kind_string(name, &schema.schema_data, s)
            }
            Type::Object(ref o) => {
                resolve_schema_component_kind_object(name, &schema.schema_data, o, inner)
            }
            Type::Number(ref n) => {
                resolve_schema_component_kind_number(name, &schema.schema_data, n)
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Not implemented yet!",
            )),
        }
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Schema: {:#?} is not of Kind Type", schema),
        ))
    }
}

// Resolves the `Type::String(StringType)` component
//
// TODO: Handling `schema_data`
fn resolve_schema_component_kind_string(
    name: &str,
    _data: &SchemaData,
    s: &StringType,
) -> std::io::Result<TokenStream> {
    let ident = Ident::new(&sanitize_str_for_ident(name), Span::call_site());
    let tokens = if s.enumeration.is_empty() {
        quote! {
            pub struct #ident(String);
        }
    } else {
        let enum_variants = s
            .enumeration
            .iter()
            .map(|s| s.as_ref().unwrap())
            .collect::<Vec<_>>();
        let mut enum_variant_tokens = TokenStream::new();
        for var in enum_variants {
            let var_ident = Ident::new(&var, Span::call_site());
            enum_variant_tokens.extend(quote! { #var_ident, });
        }
        quote! {
            pub enum #ident {
                #enum_variant_tokens
            }
        }
    };

    Ok(tokens)
}

fn resolve_reference_or_box_schema_component(
    name: &str,
    _data: &SchemaData,
    ref_or_schema: &ReferenceOr<Box<Schema>>,
) -> std::io::Result<TokenStream> {
    match ref_or_schema {
        ReferenceOr::Reference { reference } => {
            let field_ident = Ident::new(&sanitize_str_for_ident(&name), Span::call_site());

            let referred_type = reference.split('#').last().unwrap();
            let referred_type = referred_type.split("/").last().unwrap();
            let field_ty_ident =
                Ident::new(&sanitize_str_for_ident(referred_type), Span::call_site());
            Ok(quote! { #field_ident: #field_ty_ident , })
        }
        ReferenceOr::Item(ref s) => resolve_schema_type_component(name, s, true),
    }
}

// Resolves the `ObjectType`
//
// If `additional_properties` is set, it's an `inner` object and it's resolved as
// `field: HashMap<String, ReferredObject>`
fn resolve_schema_component_kind_object(
    name: &str,
    data: &SchemaData,
    object: &ObjectType,
    inner: bool,
) -> std::io::Result<TokenStream> {
    let ident = Ident::new(&sanitize_str_for_ident(name), Span::call_site());
    let tokens = if object.additional_properties.is_some() {
        let additional = object.additional_properties.as_ref().unwrap();
        assert!(inner);
        if let AdditionalProperties::Schema(s) = additional {
            if let ReferenceOr::Reference { reference } = &**s {
                let referred_type = reference.split('#').last().unwrap();
                let referred_type = referred_type.split("/").last().unwrap();
                let value_ident =
                    Ident::new(&sanitize_str_for_ident(referred_type), Span::call_site());
                quote! { #ident: std::collections::HashMap<String, #value_ident> , }
            } else {
                // TODO: Ideally we should not reach here, but let's keep it for now. Later make
                // this an Err Return.
                quote! { () }
            }
        } else {
            // TODO: Ideally we should not reach here, but let's keep it for now. Later make
            // this an Err Return.
            quote! { () }
        }
    } else {
        // This is an Outer object and is resolved as a `struct`.
        let mut obj_tokens = TokenStream::new();
        for (prop_name, prop_value) in &object.properties {
            let property_toks =
                resolve_reference_or_box_schema_component(prop_name, data, prop_value);
            obj_tokens.extend(property_toks);
        }
        quote! {
            pub struct #ident {
                #obj_tokens
            }
        }
    };
    Ok(tokens)
}

// Resolves the `NumberType`
fn resolve_schema_component_kind_number(
    name: &str,
    _data: &SchemaData,
    _num: &NumberType,
) -> std::io::Result<TokenStream> {
    let ident = Ident::new(&sanitize_str_for_ident(name), Span::call_site());
    Ok(quote! {
        #ident(f64);
    })
}

fn sanitize_str_for_ident(name: &str) -> String {
    if name.starts_with("5g") {
        name.replace("5g", "Fiveg")
    } else if name.starts_with("5G") {
        name.replace("5G", "FiveG")
    } else {
        name.to_string()
    }
}