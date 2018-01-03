#![recursion_limit = "1024"]
extern crate rlua;
extern crate syn;
#[macro_use] extern crate quote;

extern crate proc_macro;

use proc_macro::{TokenStream};
use syn::{VariantData,Ident};

trait FromLuaTable {
    fn from_table(table: &rlua::Table) -> Self;
}

#[proc_macro_derive(FromLuaTable)]
pub fn from_lua_table(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_macro_input(&source).unwrap();

    let idents: Vec<Ident> = match ast.body {
        syn::Body::Struct(vdata) => {
            match vdata {
                VariantData::Struct(fields) => {
                    let mut idents = Vec::new();
                    for ref field in fields.iter() {
                        match &field.ident {
                            &Some(ref ident) => {
                                idents.push(ident.clone());
                            },
                            &None => panic!("The structure is missing a field identity"),
                        }
                    }
                    idents
                },
                VariantData::Tuple(_) | VariantData::Unit => {
                    panic!("This can only be derived for structs");
                },
            }
        },
        syn::Body::Enum(_) => panic!("This is only defined for structs"),
    };

    // contains quoted strings containing the struct fields in the same order as the vector of
    // idents.
    let mut keys = Vec::new();
    for ident in idents.iter() {
        keys.push(String::from(ident.as_ref()));
    }


    let name = &ast.ident;
    #[allow(unused)]
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let tokens = quote! {
        impl #impl_generics FromLuaTable for #name #ty_generics #where_clause {
            #[allow(unused)]
            fn from_lua_table(table: &rlua::Table) -> Self {
                #[allow(unused_mut)]
                let mut ret = #name::default();

                #(
                    if let Ok(val) = table.get(#keys) {
                        ret.#idents = val;
                    }
                )*
                ret
            }
        }
    };
    tokens.parse().unwrap()
}

#[cfg(test)]
mod tests {
    /*
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    */
}


