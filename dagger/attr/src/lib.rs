// this is a procedural macro
// see my Cargo.toml

// read and manipulate Rust code from our code
extern crate proc_macro;

use crate::proc_macro::TokenStream;
// use quote::quote;
// use syn;

// this fn is mostly boilerplate
#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("#[component]");
    implem(attr, item)
}

#[proc_macro_attribute]
pub fn inject(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("#[inject]");
    implem(attr, item)
}

fn implem(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    // let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    // impl_hello_macro(&ast)
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"\n", item.to_string());
    item
}

// this fn contains the actual logic
// fn impl_component_attribute(ast: &syn::DeriveInput) -> TokenStream {
//     let name = &ast.ident;
//     let gen = quote! {
//         impl HelloMacro for #name {
//             fn hello_macro() {
//                 println!("Hello, Macro! My name is {}", stringify!(#name));
//             }
//         }
//     };
//     gen.into()
// }
