use proc_macro::TokenStream;
use quote::quote;

fn impl_demo_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    TokenStream::from(quote! {
        impl MacroDemo for #name {
            fn demo_macro() {
                println!("Hello, macro! My name is {}", stringify!(#name));
            }
        }
    })
}

#[proc_macro_derive(MacroDemo)]
pub fn derive_demo_macro(input: TokenStream) -> TokenStream {
    // For more info on why this is used instead of syn::parse().unwrapt()
    // see https://docs.rs/syn/2.0.94/syn/macro.parse_macro_input.html
    let ast = syn::parse_macro_input!(input);

    impl_demo_macro(&ast)
}
