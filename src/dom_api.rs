use wasm_bindgen::prelude::*;
use ::TextOrElem;

#[wasm_bindgen]
extern {
    pub type Document;
    pub type Element;
    pub static document: Document;
    // pub type HTMLElement;
    pub type Text;
    #[wasm_bindgen(method)]
    fn createElement(this: &Document, name: String) -> Element;
    #[wasm_bindgen(method)]
    fn createTextNode(this: &Document, data: String) -> Text;
    #[wasm_bindgen(method)]
    fn querySelector(this: &Document, sel: &str) -> Element;

    #[wasm_bindgen(method, js_name= appendChild)]
    fn appendElementNode(this: &Element, child: Element);

    #[wasm_bindgen(method, js_name= appendChild)]
    fn appendTextNode(this: &Element, child: Text);
    
}

// pub trait Node {
    
// }

// impl Node for Element {}
// impl Node for Text {}


pub fn create_element(name: String) -> Element {
    document.createElement(name)
}

pub fn create_text_node(data: String) -> Text {
    document.createTextNode(data)
}

pub fn append_child(node: &Element, child: TextOrElem) {
    match child {
        TextOrElem::Text(txt) => node.appendTextNode(txt),
        TextOrElem::Elem(elm) => node.appendElementNode(elm)
    }
}

pub fn get_element(sel: &str) -> Element {
    document.querySelector(sel)
}

