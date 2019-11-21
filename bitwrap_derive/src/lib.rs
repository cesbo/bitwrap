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
    unpack_list: TokenStream,

    skip: usize,
    remain: usize,
}


impl BitWrapMacro {
    fn new(ident: &Ident) -> Self {
        Self {
            struct_id: ident.clone(),
            unpack_list: TokenStream::default(),

            skip: 0,
            remain: 0,
        }
    }

    fn build_unpack_bits(&mut self, field: &syn::Field, meta_list: &syn::MetaList) {
        let mut bits = match meta_list.nested.first() {
            Some(syn::NestedMeta::Lit(syn::Lit::Int(v))) => v.base10_parse::<usize>().unwrap(),
            _ => panic!("bits value should be a number"),
        };

        let ty = &field.ty;
        let ident = &field.ident;

        let bytes_check = (bits + 7) / 8 + self.skip;

        self.unpack_list.extend(quote! {
            debug_assert!(src.len() >= #bytes_check, "array length is to short for BitWrap");
            self.#ident =
        });

        while bits > self.remain {
            let skip = self.skip;
            let shift = bits - self.remain; // value left shift
            let mask = 0xFFu8 >> (8 - self.remain);

            self.unpack_list.extend(quote! {
                (((src[#skip] & #mask) as #ty) << #shift) |
            });

            bits -= self.remain;
            self.remain = 8;
            self.skip += 1;
        }

        let skip = self.skip;
        let shift = self.remain - bits; // byte right shift
        let mask = 0xFFu8 >> (8 - bits);

        if shift != 0 {
            self.unpack_list.extend(quote! {
                (((src[#skip] >> #shift) & #mask) as #ty);
            });
        } else {
            self.unpack_list.extend(quote! {
                ((src[#skip] & #mask) as #ty);
            });
        }

        self.remain -= bits;
        if self.remain == 0 {
            self.remain = 8;
            self.skip += 1;
        }
    }

    fn build_unpack(&mut self, field: &syn::Field) {
        for attr in field.attrs.iter().filter(|v| v.path.segments.len() == 1) {
            match attr.path.segments[0].ident.to_string().as_str() {
                "bits" => {
                    let meta = attr.parse_meta().unwrap();
                    match &meta {
                        syn::Meta::List(v) => self.build_unpack_bits(field, v),
                        _ => panic!("bits meta format mismatch"),
                    }
                }
                _ => {}
            };
        }
    }

    fn build(&mut self, data: &syn::DataStruct) -> TokenStream {
        self.skip = 0;
        self.remain = 8;

        let fields = match &data.fields {
            syn::Fields::Named(v) => &v.named,
            syn::Fields::Unnamed(_v) => unimplemented!(),
            syn::Fields::Unit => unimplemented!(),
        };

        for field in fields {
            self.build_unpack(field);
        }

        let struct_id = &self.struct_id;
        let unpack_list = &self.unpack_list;

        quote! {
            impl BitWrap for #struct_id {
                fn unpack(&mut self, src: &[u8]) {
                    #unpack_list
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
