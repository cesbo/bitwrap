extern crate proc_macro;

use proc_macro2::{
    TokenStream,
    Ident,
};

use quote::quote;

use syn::{
    self,
    parse_macro_input,
};


struct BitWrapMacro {
    struct_id: Ident,
}


impl BitWrapMacro {
    fn new(ident: &Ident) -> Self {
        Self {
            struct_id: ident.clone(),
        }
    }

    fn build(&mut self, data: &syn::DataStruct) -> TokenStream {
        let struct_id = &self.struct_id;

        quote! {
            impl BitWrap for #struct_id {
                fn unpack(&mut self, src: &[u8]) {
                    unimplemented!()
                }
            }
        }
    }
}


#[proc_macro_derive(BitWrap, attributes(bits))]
pub fn bitwrap_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    if let syn::Data::Struct(ref s) = input.data {
        let mut bitwrap = BitWrapMacro::new(&input.ident);
        bitwrap.build(s).into()
    } else {
        panic!("struct required")
    }
}
