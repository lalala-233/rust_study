use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Name)]
pub fn struct_name_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_struct_name(&ast)
}

fn impl_struct_name(ast: &syn::DeriveInput) -> TokenStream {
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
