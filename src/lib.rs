#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

mod dom_api;
mod utils;

use dom_api::*;
use utils::*;

#[macro_export]
macro_rules! children {
    ( $( $x: expr),* ) => {
        {
            vec![$(
                $x.into(),
            )*]
        }
    };
}

pub fn h(name: &str, _props: Option<String>, children: Vec<VNode>) -> VElem {
    let name = String::from(name);
    VElem {
        name,
        _props,
        children,
    }
}
pub fn create_node<T: Into<VNode>>(node: T) -> TextOrElem {
    let node = node.into();
    match node {
        VNode::Elem(elm) => {
            let el = create_element(elm.name);
            elm.children
                .into_iter()
                .map(create_node)
                .for_each(|child| append_child(&el, &child));
            TextOrElem::Elem(el)
        }
        VNode::Text(data) => TextOrElem::Text(create_text_node(data)),
    }
}

pub fn mount(sel: &str, node: &TextOrElem) -> Element {
    let root = get_element(sel);
    // let node = node.into();
    append_child(&root, &node);
    root
}

#[wasm_bindgen]
pub fn main() {
    let a = h("ul", None, children! [
        h("li", None, children! [
            "first try"
        ]),
        h("li", None, children! [
            "first try"
        ])
    ]);
    let b = h("ul", None, children! [
        h("li", None, children! [
            "first try"
        ])
        // h("li", None, children! [
        //     "second try"
        // ])
    ]);
    let node_a = create_node(a.clone());
    // let b = create_node(b);
    let root = mount("#app", &node_a);
    // replace_child(&root, &b, &get_child(&root, 0));
    // for _i in 1..100_000_000_000 as usize {
        
    // }
    update_vnode(&root, Some(a.into()), Some(b.into()), None)

}
