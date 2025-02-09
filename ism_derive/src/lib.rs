// Note: Generated by AI assistant
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let expanded = quote! {
        impl ism::Serialize for #name {
            fn serialize(&self) -> ism::text::Message {
                serde_json::to_value(self)
                    .map(|v| serde_json::from_value(v).unwrap())
                    .unwrap()
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl ism::Deserialize for #name {
            fn deserialize(msg: ism::text::Message) -> Result<Self, ism::text::Error> {
                let value = serde_json::to_value(msg)?;
                Ok(serde_json::from_value(value)?)
            }
        }
    };

    TokenStream::from(expanded)
} 