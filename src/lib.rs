#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
// use wasm_bindgen::prelude::*;

mod dom_api;
mod utils;

use dom_api::*;
use utils::*;


#[macro_export]
macro_rules! children {
    ( $( $x: expr),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.into());
            )*
            temp_vec
        }
    };
}

pub fn h(name: &str, _props: Option<String>, children: Vec<Vnode>) -> Elem {
    let name = String::from(name);
    Elem {
        name,
        _props,
        children
    }
}
pub fn create_node<T: Into<Vnode>> (node: T) -> TextOrElem {
    let node = node.into();
    match node {
        Vnode::Elem(elm) => {
            let el = create_element(elm.name);
            elm.children.into_iter()
                .map(create_node)
                .for_each(|child| append_child(&el, child));
            TextOrElem::Elem(el)
        }
        Vnode::Text(data) => TextOrElem::Text(create_text_node(data))
    }
}

pub fn mount<T: Into<TextOrElem>> (sel: &str, node: T) {
    let root = get_element(sel);
    let node = node.into();
    append_child(&root, node)
}
