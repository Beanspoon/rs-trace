use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Path};

// #[proc_macro_attribute]
// pub fn modify(args: TokenStream, input: TokenStream) -> TokenStream {
//     let ast = parse_macro_input!(input as DeriveInput);

//     let output = quote! {

//       trait New {}

//     };
//     output.into()
// }

// trait Modify<T> {
//     fn modify(self, read_value: T) -> T;
// }

#[derive(FromDeriveInput)]
#[darling(attributes(modify))]
struct Opts {
    width: u8,
    position: u8,
    register_type: Path,
}

#[proc_macro_derive(Modify, attributes(modify))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Missing options");
    let DeriveInput { ident, .. } = input;

    let base: u8 = 2;
    let mask = (base.pow(opts.width.into()) - 1) << opts.position;
    let position = opts.position;
    let register_type = opts.register_type;
    let output = quote! {
        impl #ident {
            fn modify(self, read_value: #register_type) -> #register_type {
                (read_value & #mask) | ((self as #register_type) << #position)
            }
        }
    };

    output.into()
}
