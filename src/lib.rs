// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of ink!.
//
// ink! is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// ink! is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with ink!.  If not, see <http://www.gnu.org/licenses/>.

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{
    TokenStream as TokenStream2,
    Span,
};
use syn::{
    Result,
    parse::Error,
    spanned::Spanned,
};

// Attribute applicable to inline modules (e.g. `mod foo { ... }`) or
// applicable to structures, e.g. `struct Foo { ... }`.
//
// Always returns an error pointing to the identifier of either the module or the struct.
// This is going to return a properly spanned error for structs but will miss span information
// if applied on modules.
#[proc_macro_attribute]
pub fn my_attribute(_attr: TokenStream, item: TokenStream) -> TokenStream {
    match my_attribute_or_err(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn my_attribute_or_err(item: TokenStream2) -> Result<TokenStream2> {
    match syn::parse2::<syn::ItemMod>(item.clone()) {
        Ok(item_mod) => Err(Error::new(item_mod.ident.span(), "attribute on module")),
        Err(no_mod) => {
            match syn::parse2::<syn::ItemStruct>(item) {
                Ok(item_struct) => Err(Error::new(item_struct.ident.span(), "attribute on struct")),
                Err(_) => Err(Error::new(Span::call_site(), "attribute on something else")),
            }
        }
    }
}
