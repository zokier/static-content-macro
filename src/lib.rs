extern crate proc_macro;

use std::{env, path::Path};
use proc_macro2::Literal;
use quote::quote;

#[proc_macro]
pub fn create_static(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let static_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("static");
    let mut file_list = vec![];
    for dent_result in static_dir.read_dir().unwrap() {
        let dent = dent_result.unwrap();
        if let Ok(file_contents) = std::fs::read(dent.path()) {
            let file_name = dent.file_name().into_string().unwrap();
            let file_name_literal = Literal::string(&file_name);
            let file_contents_literal = Literal::byte_string(&file_contents);
            file_list.push(quote! {
                #file_name_literal => Some(#file_contents_literal),
            });
        }
    }
    let tokens = quote! {
        mod static_content {
            static CONTENT: &[&[u8]] = &[
                include_bytes!("index-top.html")
            ];

            pub fn get_content(path: &str) -> Option<&[u8]> {
                match path {
                    "index-top.html" => Some(CONTENT[0]),
                    #(#file_list.iter())*
                    _ => None,
                }
            }
        }
    };
    return tokens.into();
}
