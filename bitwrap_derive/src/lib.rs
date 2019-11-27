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
    pack_list: TokenStream,
    unpack_list: TokenStream,
    bits: usize,
}


impl BitWrapMacro {
    fn new(ident: &Ident) -> Self {
        Self {
            struct_id: ident.clone(),
            pack_list: TokenStream::default(),
            unpack_list: TokenStream::default(),
            bits: 0,
        }
    }

    fn build_field_bits(&mut self, field: &syn::Field, meta_list: &syn::MetaList) {
        let mut bits = match meta_list.nested.first() {
            Some(syn::NestedMeta::Lit(syn::Lit::Int(v))) => v.base10_parse::<usize>().unwrap(),
            _ => panic!("bits value should be a number"),
        };

        let ty = &field.ty;
        let ident = &field.ident;

        if self.bits == 8 {
            self.pack_list.extend(quote! {
                let b = 0;
            });

            self.unpack_list.extend(quote! {
                debug_assert!(src.len() >= ((#bits + 7) / 8 + offset),
                    "array length is to short for BitWrap");
            });
        }

        self.unpack_list.extend(quote! {
            self.#ident =
        });

        while bits > self.bits {
            let shift = bits - self.bits; // value left shift
            let mask = 0xFFu8 >> (8 - self.bits);

            self.pack_list.extend(quote! {
                let b = b | (((self.#ident >> #shift) as u8) & #mask);
                dst.push(b);
                let b = 0;
            });

            self.unpack_list.extend(quote! {
                (((src[{ let x = offset; offset += 1; x }] & #mask) as #ty) << #shift) |
            });

            bits -= self.bits;
            self.bits = 8;
        }

        let shift = self.bits - bits; // byte right shift
        let mask = 0xFFu8 >> (8 - bits);

        if shift != 0 {
            self.pack_list.extend(quote! {
                let b = b | (((self.#ident as u8) & #mask) << #shift);
            });
        } else {
            self.pack_list.extend(quote! {
                let b = b | ((self.#ident as u8) & #mask);
            });
        }

        if shift != 0 {
            self.unpack_list.extend(quote! {
                (((src[offset] >> #shift) & #mask) as #ty);
            });
        } else {
            self.unpack_list.extend(quote! {
                ((src[offset] & #mask) as #ty);
            });
        }

        self.bits -= bits;
        if self.bits == 0 {
            self.bits = 8;

            self.pack_list.extend(quote! {
                dst.push(b);
            });

            self.unpack_list.extend(quote! {
                offset += 1;
            });
        }
    }

    fn build_unpack_bitfield(&mut self, field: &syn::Field) {
        assert_eq!(self.bits, 8, "bitwrap not aligned");

        let ident = &field.ident;

        self.unpack_list.extend(quote! {
            offset += self.#ident.unpack(&src[offset ..]);
        });
    }

    fn build_field(&mut self, field: &syn::Field) {
        for attr in field.attrs.iter().filter(|v| v.path.segments.len() == 1) {
            match attr.path.segments[0].ident.to_string().as_str() {
                "bits" => {
                    let meta = attr.parse_meta().unwrap();
                    match &meta {
                        syn::Meta::List(v) => self.build_field_bits(field, v),
                        _ => panic!("bits meta format mismatch"),
                    }
                }
                "bitfield" => {
                    self.build_unpack_bitfield(field)
                }
                _ => {}
            };
        }
    }

    fn build(&mut self, data: &syn::DataStruct) -> TokenStream {
        self.bits = 8;

        let fields = match &data.fields {
            syn::Fields::Named(v) => &v.named,
            syn::Fields::Unnamed(_v) => unimplemented!(),
            syn::Fields::Unit => unimplemented!(),
        };

        for field in fields {
            self.build_field(field);
        }

        assert_eq!(self.bits, 8, "bitwrap not aligned");

        let struct_id = &self.struct_id;
        let pack_list = &self.pack_list;
        let unpack_list = &self.unpack_list;

        quote! {
            impl BitWrap for #struct_id {
                fn pack(&self, dst: &mut Vec<u8>) -> usize {
                    let len = dst.len();
                    #pack_list
                    dst.len() - len
                }

                fn unpack(&mut self, src: &[u8]) -> usize {
                    let mut offset: usize = 0;
                    #unpack_list
                    offset
                }
            }
        }
    }
}


#[proc_macro_derive(BitWrap, attributes(bits, bitfield))]
pub fn bitwrap_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    if let syn::Data::Struct(ref s) = input.data {
        let mut bitwrap = BitWrapMacro::new(&input.ident);
        bitwrap.build(s).into()
    } else {
        panic!("struct required")
    }
}
