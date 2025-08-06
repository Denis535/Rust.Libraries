#![no_implicit_prelude]
#![forbid(absolute_paths_not_starting_with_crate)]

extern crate std;
extern crate syn;
extern crate quote;
extern crate proc_macro;

use crate::std::*;
use crate::std::convert::*;
use crate::syn::*;
use crate::quote::quote;
use crate::proc_macro::*;

#[proc_macro_attribute]
pub fn child_of(attr: TokenStream, item: TokenStream) -> TokenStream {
    let base_type = parse_macro_input!(attr as Type);
    let item = parse_macro_input!(item as Item);
    if let Item::Struct(mut struct_item) = item {
        if let Fields::Named(fields) = &mut struct_item.fields {
            let field: Field = parse_quote! { base: #base_type };
            fields.named.insert(0, field);
            quote!(#struct_item).into()
        } else {
            Error::new_spanned(struct_item, "#[child_of] can only be used on structs with named fields").to_compile_error().into()
        }
    } else {
        Error::new_spanned(item, "#[child_of] can only be used on structs").to_compile_error().into()
    }
}
