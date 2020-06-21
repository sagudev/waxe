#[macro_use]
extern crate mozjs;
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
use proc_macro::TokenStream;
mod backend;
use backend::get_namespace;
use syn::{parse, parse_macro_input, AttributeArgs, NestedMeta};
use syn::MetaNameValue;

#[proc_macro_attribute]
pub fn jsfn(metadata: TokenStream, input: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", input.to_string());
    println!("item: \"{}\"", metadata.to_string());
    let item: syn::Item = syn::parse(input).expect("failed to parse input");
    let attr: Vec<NestedMeta> = parse_macro_input!(metadata as AttributeArgs);
    /* if metadata.is_empty() { // normal function

    } else {
        match get_namespace(attr) {
            Some(x) => {

            }
            None => panic!("Wrong attribute in jsfn"),
        }
    }; */
    for a in attr {
        match a {
            NestedMeta::Meta(x) => match x {
                syn::Meta::Path(_) => println!("1"),
                syn::Meta::List(_) => println!("2"),
                syn::Meta::NameValue(y) => println!("this"),
            },
            NestedMeta::Lit(_) => println!("lit"),
        }
    }
    let output = quote! { #item };
    output.into()
}
