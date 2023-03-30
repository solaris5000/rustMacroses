extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput, Data};
use quote::quote;

/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(Counte)]
pub fn count(input: TokenStream) -> TokenStream {
    let macro_input = parse_macro_input!(input as DeriveInput);

     let res = &macro_input.ident;
     let data = &macro_input.data;

    let count = match  *data {

        Data::Enum(ref dat) => dat.variants.len(),
        Data::Struct(_) | Data::Union(_) => unimplemented!(),
    };

    quote!(
        impl #res {
            fn count() -> usize {
                3 as usize
            }
        }
    ).into()
}

#[proc_macro_derive(Identy)]
pub fn identy(input: TokenStream) -> TokenStream {
    let macro_input = parse_macro_input!(input as DeriveInput);

    let var = &macro_input.ident;
    let raw_data = &macro_input.data;

    let id = match *raw_data {
        Data::Enum(ref data) => { data.enum_token },
        Data::Struct(_) | Data::Union(_) => unimplemented!(),
    };

    quote!(
        impl #var {
            fn identy() -> usize {
                #id
            }
        }
    ).into()
}


#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
