use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Name)]
pub fn tag_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_tag(&ast)
}

fn impl_tag(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Name for #name {
            fn name(&self) -> &'static str {
                stringify!(#name)
            }
        }
    };
    gen.into()
}
