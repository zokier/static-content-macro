#![feature(track_path)]
extern crate proc_macro;

use proc_macro2::Literal;
use quote::quote;
use std::{env, path::Path};

#[proc_macro]
pub fn create_static(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let output = create_static_inner(input);
    proc_macro::TokenStream::from(output)
}

fn create_static_inner(_input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let static_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("static");
    let mut file_list = vec![];
    for dent_result in static_dir.read_dir().unwrap() {
        let dent = dent_result.unwrap();
        if let Ok(file_contents) = std::fs::read(dent.path()) {
            let file_name = dent.file_name().into_string().unwrap();
            proc_macro::tracked_path::path(dent.path().into_os_string().into_string().unwrap());
            let file_name_literal = Literal::string(&file_name);
            let file_contents_literal = Literal::byte_string(&file_contents);
            file_list.push(quote! {
                #file_name_literal => Some(#file_contents_literal),
            });
        }
    }
    let tokens = quote! {
        mod static_content {
            pub fn get_content(path: &str) -> Option<&'static [u8]> {
                match path {
                    #(#file_list)*
                    _ => None,
                }
            }
        }
    };
    return tokens.into();
}

#[test]
fn test_create_static() {
    let input = quote!("static");
    let input = proc_macro2::TokenStream::from(input);
    let output = create_static_inner(input);
    println!("{}", output);
}
