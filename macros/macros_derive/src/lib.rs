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

/// This is a demo macro to showcase custom derive macro
///
/// This implements a dummy MacroDemo trait for the specific struct or enum provided
/// in this case it will print "Hello, macro! My name is <TypeName>"
/// where <TypeName> is the name of the provided struct or enum
#[proc_macro_derive(MacroDemo)]
pub fn derive_demo_macro(input: TokenStream) -> TokenStream {
    // For more info on why this is used instead of syn::parse().unwrapt()
    // see https://docs.rs/syn/2.0.94/syn/macro.parse_macro_input.html
    let ast = syn::parse_macro_input!(input);

    impl_demo_macro(&ast)
}

/// This is a demo macro to showcase a Attribute-like macro
///
/// This is a dummy implementation but irl it could for example
/// 1) Parse attr and its args to get route info
/// 2) Parse item and make sure it's a function
/// 3) Generate implementation for a http handle that call the item fn when triggered
#[proc_macro_attribute]
pub fn route(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// This is a demo macro showcase a Function-like macro
///
/// This is a dummy implementation but irl it could for example
/// 1) Parse a raw SQL query in the _input
/// 2) Generate implementation for the sql query from our lib
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    input
}
