use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Fields, GenericArgument, Path, PathArguments, Type, TypePath,
};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut setters = Vec::new();

    let name = input.ident;

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields_named),
            ..
        }) => fields_named.named,

        _ => panic!("this is only for structs!"),
    };

    for field in fields.iter() {
        let name = field.ident.clone();
        let ty = field.ty.clone();

        //check for the cutsom thing
        let mut custom_requested = false; 
        for attr in field.attrs.clone() {
            if attr.path().is_ident("builder") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("custom") {
                        custom_requested = true;
                    };

                    Ok(())
                })
                .expect("failed to parse builder attribute");
            }
        }

        if custom_requested {
            continue;
        }

        let something = match ty {
            Type::Path(tyype) => {
                tyype
            }, 
            _ => panic!("panic1")
        };

        match option_inner_type(&something.path) {
            Some(Type::Path(TypePath { path, .. })) if path.is_ident("String") => {
                setters.push(quote! {

                    pub fn #name<S: Into<String>>(mut self, value: S) -> Self {
                        self.#name = Some(value.into());
                        self
                    }
        
                });
            },

            Some(inner) => {
                setters.push(quote! {

                    pub fn #name(mut self, value: #inner) -> Self {
                        self.#name = Some(value);
                        self
                    }
        
                });
            },
            _ => panic!("panic2")
        };


         
        
    }

    let expanded = quote! {
        #[automatically_derived]
        impl #name {
            #(#setters)*
        }
    };

    TokenStream::from(expanded)
}

fn option_inner_type(path: &Path) -> Option<&Type> {
    if path.leading_colon.is_some() {
        return None;
    }

    if path.segments.len() != 1 || path.segments[0].ident != "Option" {
        return None;
    }

    let ab = match &path.segments[0].arguments {
        PathArguments::AngleBracketed(ab) => ab,
        _ => return None,
    };

    if ab.args.len() != 1 {
        return None;
    }

    match &ab.args[0] {
        GenericArgument::Type(t) => Some(t),
        _ => None,
    }
}

#[proc_macro_attribute]
pub fn experimental(_: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::Item); // Convert to proc_macro2::TokenStream
    let output = quote! {
        #[doc = "⚠️ Experimental: This feature is still experimental in the qbittorrent webAPI."]
        #[doc = ""]
        #[doc = ""]
        #item
    };
    output.into()
}