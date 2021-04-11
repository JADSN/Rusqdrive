// * Struct to generate code base on fields(syn::Field)

use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::Field;
#[derive(Debug)]
pub struct FieldStruct {
    ident: Ident,
}

impl FieldStruct {
    pub fn new(field: Field) -> FieldStruct {
        FieldStruct {
            ident: field.ident.clone().expect(&format!(
                "Line: {}, Column: {}, File: {}",
                line!(),
                column!(),
                file!(),
            )),
        }
    }

    pub fn only_names(&self) -> TokenStream2 {
        let f_name = &self.ident;
        quote!(#f_name)
    }

    pub fn as_row_read_all(&self, idx: usize) -> TokenStream2 {
        let f_name = &self.ident;
        quote!(#f_name: row.get(#idx)?,)
    }

    pub fn as_params(&self) -> TokenStream2 {
        let field_name = &self.ident;
        quote!(data.#field_name,)
    }
}
