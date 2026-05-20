use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, Ident, LitStr, Meta};

/// Parsed `#[moju(...)]` attributes on a type.
#[derive(Default)]
struct TypeAttr {
    kind: String,
    domain: String,
    role: Option<String>,
    identity: Option<String>,
    tag: Option<String>,
}

/// Extract `#[moju(...)]` attributes from the type-level attribute list.
fn parse_type_attrs(attrs: &[syn::Attribute]) -> TypeAttr {
    let mut result = TypeAttr::default();

    for attr in attrs {
        if !attr.path().is_ident("moju") {
            continue;
        }

        if let Meta::List(list) = &attr.meta {
            let _ = list.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    match ident.to_string().as_str() {
                        "kind" => result.kind = meta.value()?.parse::<LitStr>()?.value(),
                        "domain" => result.domain = meta.value()?.parse::<LitStr>()?.value(),
                        "role" => result.role = Some(meta.value()?.parse::<LitStr>()?.value()),
                        "identity" => {
                            result.identity = Some(meta.value()?.parse::<LitStr>()?.value())
                        }
                        "tag" => result.tag = Some(meta.value()?.parse::<LitStr>()?.value()),
                        other => {
                            return Err(syn::Error::new(
                                meta.path.span(),
                                format!("unknown moju attr: `{other}`"),
                            ));
                        }
                    }
                }
                Ok(())
            });
        }
    }

    result
}

/// Collect field idents that have `#[moju(unique)]`.
fn parse_field_unique_attrs(fields: &Fields) -> Vec<Ident> {
    let mut unique = Vec::new();

    let named = match fields {
        Fields::Named(named) => named,
        _ => return unique,
    };

    for field in &named.named {
        for attr in &field.attrs {
            if !attr.path().is_ident("moju") {
                continue;
            }
            if let Meta::List(list) = &attr.meta {
                let _ = list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("unique") {
                        if let Some(ident) = &field.ident {
                            unique.push(ident.clone());
                        }
                    }
                    Ok(())
                });
            }
        }
    }

    unique
}

fn to_lit_strs(idents: &[Ident]) -> Vec<LitStr> {
    idents
        .iter()
        .map(|i| LitStr::new(&i.to_string(), i.span()))
        .collect()
}

#[proc_macro_derive(MoJu, attributes(moju))]
pub fn derive_moju(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let attrs = parse_type_attrs(&input.attrs);

    // syn 2.x: fields are inside `data`.  Enum has per-variant fields so we
    // skip field-level unique collection for enums; struct/union have top-level
    // named fields.
    let unique_fields = match &input.data {
        Data::Struct(s) => parse_field_unique_attrs(&s.fields),
        Data::Enum(_) => {
            // No field-level unique for enums — emit impl and return early.
            let kind = &attrs.kind;
            let domain = &attrs.domain;
            let role = opt_str(&attrs.role);
            let identity = opt_str(&attrs.identity);
            let tag = opt_str(&attrs.tag);

            let expanded = quote! {
                impl ::moju_derive::MoJuItem for #name {
                    fn moju_kind() -> &'static str { #kind }
                    fn moju_domain() -> &'static str { #domain }
                    fn moju_role() -> ::core::option::Option<&'static str> { #role }
                    fn moju_identity() -> ::core::option::Option<&'static str> { #identity }
                    fn moju_tag() -> ::core::option::Option<&'static str> { #tag }
                }
            };
            return TokenStream::from(expanded);
        }
        Data::Union(u) => parse_field_unique_attrs(&Fields::Named(u.fields.clone())),
    };

    let unique_strs = to_lit_strs(&unique_fields);

    let kind = &attrs.kind;
    let domain = &attrs.domain;
    let role = opt_str(&attrs.role);
    let identity = opt_str(&attrs.identity);
    let tag = opt_str(&attrs.tag);

    let unique_tokens = if unique_strs.is_empty() {
        quote! { &[] }
    } else {
        quote! { &[#(#unique_strs),*] }
    };

    let expanded = quote! {
        impl ::moju_derive::MoJuItem for #name {
            fn moju_kind() -> &'static str { #kind }

            fn moju_domain() -> &'static str { #domain }

            fn moju_role() -> ::core::option::Option<&'static str> { #role }

            fn moju_identity() -> ::core::option::Option<&'static str> { #identity }

            fn moju_tag() -> ::core::option::Option<&'static str> { #tag }

            fn moju_unique_fields() -> &'static [&'static str] { #unique_tokens }
        }
    };

    TokenStream::from(expanded)
}

fn opt_str(opt: &Option<String>) -> proc_macro2::TokenStream {
    match opt {
        Some(s) => {
            let lit = LitStr::new(s, proc_macro2::Span::call_site());
            quote! { ::core::option::Option::Some(#lit) }
        }
        None => quote! { ::core::option::Option::None },
    }
}
