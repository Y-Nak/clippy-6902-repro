use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(serialize)]
pub fn use_self(input: TokenStream) -> TokenStream {
    let item: syn::ItemEnum = parse_macro_input!(input);
    let ident = item.ident;
    let var0 = item.variants[0].ident.clone();
    TokenStream::from(quote! {
        #[automatically_derived]
        impl #ident {
            fn foo() -> Self {
                #ident::#var0
            }
        }
    })
}
