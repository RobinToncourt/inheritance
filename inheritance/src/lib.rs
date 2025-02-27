use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Token, parse_macro_input};
use syn::{Item, ItemStruct, ItemImpl, Type};
use std::collections::HashMap;
use std::cell::RefCell;
use syn_serde::json;

thread_local! {
    static PROTOTYPE_STRUCT: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());

    static PROTOTYPE_IMPL: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

#[proc_macro_attribute]
pub fn prototype(_: TokenStream, struct_token: TokenStream) -> TokenStream {
    // println!("{item_struct:#?}");
    let struct_token_clone = struct_token.clone();
    let item: Item = parse_macro_input!(struct_token_clone as Item);

    match item {
        Item::Struct(item_struct) => {
            PROTOTYPE_STRUCT.with_borrow_mut(|structs|
                structs.insert(
                    item_struct.clone().ident.to_string(),
                    json::to_string(&item_struct)
                )
            );
        },
        Item::Impl(item_impl) => {
            let item_impl = item_impl.clone();
            PROTOTYPE_IMPL.with_borrow_mut(|impls|
                impls.insert(
                    get_item_impl_struct_ident(&item_impl).to_string(),
                    json::to_string(&item_impl)
                )
            );
        }
        _ => unimplemented!(),
    }

    struct_token
}

fn get_item_impl_struct_ident(item_impl: &ItemImpl) -> &syn::Ident {
    if let Type::Path(type_path) = item_impl.self_ty.as_ref() {
        &type_path.path.segments.first().unwrap().ident
    } else {
        unimplemented!()
    }
}

#[derive(Debug)]
struct Extends {
    first_parent: syn::Ident,
    other_parents: Vec<syn::Ident>,
}

impl Parse for Extends {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let first_parent = input.parse()?;
        let mut other_parents: Vec<syn::Ident> = Vec::new();

        while input.parse::<Token![,]>().is_ok() {
            other_parents.push(input.parse()?);
        }

        Ok(Self {
            first_parent,
            other_parents,
        })
    }
}

#[proc_macro_attribute]
pub fn extends(parent: TokenStream, child: TokenStream) -> TokenStream {
    let e: Extends = parse_macro_input!(parent as Extends);
    let mut c: ItemStruct = parse_macro_input!(child as ItemStruct);

    {
        PROTOTYPE_STRUCT.with_borrow(|map| {
            let first_parent: ItemStruct = get_parent_struct(map, &e.first_parent);
            match &mut c.fields {
                syn::Fields::Named(ref mut fields) => {
                    add_parent_fields(fields, &first_parent);
                },
                _ => unimplemented!(),
            }
        });
    }

    let mut impls: ItemImpl = PROTOTYPE_IMPL.with_borrow(|map| {
        get_parent_impls(map, &e.first_parent)
    });

    change_impl_from_parent_to_child(&mut impls, &c.ident);

    quote! {
        #c

        #impls
    }.into()
}

fn get_parent_struct(map: &HashMap<String, String>, parent: &syn::Ident) -> ItemStruct {
    let parent: String = parent.to_string();
    json::from_str::<ItemStruct>(&map[&parent])
        .expect(&format!("Parent '{parent}' not found!"))
}

fn add_parent_fields(fields_named: &mut syn::FieldsNamed, parent: &ItemStruct) {
    match &parent.fields {
        syn::Fields::Named(fields) => {
            for field in fields.named.iter().rev() {
                fields_named.named.insert(0, field.clone());
            }
        },
        _ => unimplemented!(),
    }
}

fn get_parent_impls(map: &HashMap<String, String>, parent: &syn::Ident) -> ItemImpl {
    let parent: String = parent.to_string();
    json::from_str::<ItemImpl>(&map[&parent])
        .expect(&format!("Parent '{parent}' not found!"))
}

fn change_impl_from_parent_to_child(
    impls: &mut ItemImpl, child: &syn::Ident,
) {
    if let Type::Path(ref mut type_path) = impls.self_ty.as_mut() {
        type_path.path.segments.first_mut().unwrap().ident = child.clone();
    } else {
        unimplemented!();
    }
}
