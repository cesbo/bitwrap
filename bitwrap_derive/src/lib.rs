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

    fn get_bits(iter: &mut syn::punctuated::Iter<'_, syn::NestedMeta>) -> usize {
        let bits = match iter.next() {
            Some(syn::NestedMeta::Lit(syn::Lit::Int(v))) => v.base10_parse::<usize>().unwrap(),
            _ => 0,
        };

        if bits == 0 || bits > 64 {
            panic!("argument #1 should be a number in range 1 .. 64");
        }

        bits
    }

    fn get_type(bits: usize) -> Ident {
        let convert_ty = if bits <= 8 {
            "u8"
        } else if bits <= 16 {
            "u16"
        } else if bits <= 32 {
            "u32"
        } else {
            "u64"
        };

        Ident::new(convert_ty, proc_macro2::Span::call_site())
    }

    fn macro_make_check(&mut self, bits: usize) {
        if self.bits == 8 {
            let bytes = (bits + 7) / 8;

            self.pack_list.extend(quote! {
                if #bytes + offset > dst.len() {
                    return Err(bitwrap::BitWrapError);
                }

                dst[offset] = 0;
            });

            self.unpack_list.extend(quote! {
                if #bytes + offset > src.len() {
                    return Err(bitwrap::BitWrapError);
                }
            });
        }
    }

    fn macro_make_bits(&mut self, ty: &Ident, bits: usize) {
        let mut bits = bits;

        self.unpack_list.extend(quote! {
            let mut value: #ty = 0;
        });

        while bits > self.bits {
            let shift = bits - self.bits; // value left shift
            let mask = 0xFFu8 >> (8 - self.bits);

            self.pack_list.extend(quote! {
                dst[offset] |= ((value >> #shift) as u8) & #mask;
                offset += 1;
                dst[offset] = 0;
            });

            self.unpack_list.extend(quote! {
                value |= ((src[offset] & #mask) as #ty) << #shift;
                offset += 1;
            });

            bits -= self.bits;
            self.bits = 8;
        }

        self.bits -= bits;

        let shift = self.bits; // byte right shift
        let mask = 0xFFu8 >> (8 - bits);

        if shift == 0 {
            self.pack_list.extend(quote! {
                dst[offset] |= (value as u8) & #mask;
                offset += 1;
            });

            self.unpack_list.extend(quote! {
                value |= (src[offset] & #mask) as #ty;
                offset += 1;
            });

            self.bits = 8;
        } else {
            self.pack_list.extend(quote! {
                dst[offset] |= ((value as u8) & #mask) << #shift;
            });

            self.unpack_list.extend(quote! {
                value |= ((src[offset] >> #shift) & #mask) as #ty;
            });
        }
    }

    fn build_field_bits(&mut self, field: &syn::Field, meta_list: &syn::MetaList) {
        let mut iter = meta_list.nested.iter();

        let field_ty = &field.ty;
        let field_ident = &field.ident;

        let bits = Self::get_bits(&mut iter);
        let ty = Self::get_type(bits);

        self.macro_make_check(bits);

        match field_ty {
            syn::Type::Path(v) if v.path.is_ident("bool") => {
                self.pack_list.extend(quote! {
                    let value: #ty = if self.#field_ident { 1 } else { 0 };
                });
            }
            _ => {
                self.pack_list.extend(quote! {
                    let value = self.#field_ident as #ty;
                });
            }
        }

        self.macro_make_bits(&ty, bits);

        match field_ty {
            syn::Type::Path(v) if v.path.is_ident("bool") => {
                self.unpack_list.extend(quote! {
                    self.#field_ident = value != 0;
                });
            }
            _ => {
                self.unpack_list.extend(quote! {
                    self.#field_ident = value as #field_ty;
                });
            }
        }
    }

    fn build_field_bits_convert(&mut self, field: &syn::Field, meta_list: &syn::MetaList) {
        let mut iter = meta_list.nested.iter();

        let field_ty = &field.ty;
        let field_ident = &field.ident;

        let bits = Self::get_bits(&mut iter);
        let ty = Self::get_type(bits);

        let mut convert_from = TokenStream::new();
        let mut convert_into = TokenStream::new();

        if meta_list.nested.len() == 1 {
            convert_from.extend(quote! { #field_ty::from });
            convert_into.extend(quote! { #field_ty::into });
        } else if meta_list.nested.len() == 3 {
            match iter.next() {
                Some(syn::NestedMeta::Meta(syn::Meta::Path(v))) => {
                    convert_from.extend(quote! { #v });
                }
                _ => panic!("bits_convert argument #2 should be a function"),
            };

            match iter.next() {
                Some(syn::NestedMeta::Meta(syn::Meta::Path(v))) => {
                    convert_into.extend(quote! { #v });
                }
                _ => panic!("bits_convert argument #3 should be a function"),
            };
        } else {
            panic!("bits_convert has wrong arguments format");
        }

        self.macro_make_check(bits);

        self.pack_list.extend(quote! {
            let value: #ty = #convert_into ( self.#field_ident );
        });

        self.macro_make_bits(&ty, bits);

        self.unpack_list.extend(quote! {
            self.#field_ident = #convert_from ( value );
        });
    }

    fn build_field_bits_skip(&mut self, meta_list: &syn::MetaList) {
        let mut iter = meta_list.nested.iter();

        let mut bits = Self::get_bits(&mut iter);

        let value = match iter.next() {
            Some(syn::NestedMeta::Lit(syn::Lit::Int(v))) => v.base10_parse::<usize>().unwrap(),
            _ => 0usize,
        };

        self.macro_make_check(bits);

        while bits > self.bits {
            let shift = bits - self.bits; // value left shift
            let mask = 0xFFu8 >> (8 - self.bits);
            let v = ((value >> shift) as u8) & mask;

            self.pack_list.extend(quote! {
                dst[offset] |= #v;
                offset += 1;
                dst[offset] = 0;
            });

            self.unpack_list.extend(quote! {
                offset += 1;
            });

            bits -= self.bits;
            self.bits = 8;
        }

        self.bits -= bits;

        let shift = self.bits; // byte right shift
        let mask = 0xFFu8 >> (8 - bits);
        let v = ((value as u8) & mask) << shift;

        self.pack_list.extend(quote! {
            dst[offset] |= #v;
        });

        if shift == 0 {
            self.pack_list.extend(quote! {
                offset += 1;
            });

            self.unpack_list.extend(quote! {
                offset += 1;
            });

            self.bits = 8;
        }
    }

    fn build_field_bits_nested(&mut self, field: &syn::Field) {
        assert_eq!(self.bits, 8, "bitwrap not aligned");

        let ident = &field.ident;

        self.pack_list.extend(quote! {
            offset += self.#ident.pack(&mut dst[offset ..])?;
        });

        self.unpack_list.extend(quote! {
            offset += self.#ident.unpack(&src[offset ..])?;
        });
    }

    fn build_field(&mut self, field: &syn::Field) {
        for attr in field.attrs.iter().filter(|v| v.path.segments.len() == 1) {
            match attr.path.segments[0].ident.to_string().as_str() {
                "bits" => {
                    let meta = attr.parse_meta().unwrap();
                    match &meta {
                        syn::Meta::Path(_) => self.build_field_bits_nested(field),
                        syn::Meta::List(v) => self.build_field_bits(field, v),
                        _ => panic!("bits format mismatch"),
                    }
                }
                "bits_skip" => {
                    let meta = attr.parse_meta().unwrap();
                    match &meta {
                        syn::Meta::List(v) => self.build_field_bits_skip(v),
                        _ => panic!("bits_skip format mismatch"),
                    }
                }
                "bits_convert" => {
                    let meta = attr.parse_meta().unwrap();
                    match &meta {
                        syn::Meta::List(v) => self.build_field_bits_convert(field, v),
                        _ => panic!("bits_convert format mismatch"),
                    }
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
            impl bitwrap::BitWrap for #struct_id {
                fn pack(&self, dst: &mut [u8]) -> Result<usize, BitWrapError> {
                    let mut offset: usize = 0;
                    #pack_list
                    Ok(offset)
                }

                fn unpack(&mut self, src: &[u8]) -> Result<usize, BitWrapError> {
                    let mut offset: usize = 0;
                    #unpack_list
                    Ok(offset)
                }
            }
        }
    }
}


#[proc_macro_derive(BitWrap, attributes(bits, bits_skip, bits_convert))]
pub fn bitwrap_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    if let syn::Data::Struct(ref s) = input.data {
        let mut bitwrap = BitWrapMacro::new(&input.ident);
        bitwrap.build(s).into()
    } else {
        panic!("struct required")
    }
}
