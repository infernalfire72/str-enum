use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote_spanned, quote};
use syn::{parse_macro_input, ItemEnum, spanned::Spanned};

#[proc_macro_attribute]
pub fn str_enum(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);
    let ident = &input.ident;
    let vis = &input.vis;
    let values = input.variants.iter().map(|field| {
        let ident = &field.ident;
        let span = field.span();
        let discrim = field.discriminant.as_ref().unwrap();
        let literal = &discrim.1;
        quote_spanned!(span => 
            pub const #ident: ValueType = #literal;
        )
    });
    
    let rule: TokenStream2 = "#[allow(non_snake_case)]".parse().unwrap();
    TokenStream::from(quote! {
        #rule
        #vis mod #ident {
            pub type ValueType = &'static str;
            #(#values)*
        }
    })
}