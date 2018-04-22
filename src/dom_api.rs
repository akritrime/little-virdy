use wasm_bindgen::prelude::*;
use TextOrElem;

#[wasm_bindgen]
extern {
    pub type Document;
    pub type Element;
    pub static document: Document;
    // pub type HTMLElement;
    pub type Text;
    pub type NodeList;
    pub type Node;

    #[wasm_bindgen(method)]
    fn createElement(this: &Document, name: String) -> Element;
    #[wasm_bindgen(method)]
    fn createTextNode(this: &Document, data: String) -> Text;
    #[wasm_bindgen(method)]
    fn querySelector(this: &Document, sel: &str) -> Element;

    #[wasm_bindgen(method, js_name= appendChild)]
    fn appendElementNode(this: &Element, child: &Element);
    #[wasm_bindgen(method, js_name= appendChild)]
    fn appendTextNode(this: &Element, child: &Text);
    #[wasm_bindgen(method, js_name= removeChild)]
    fn removeElementNode(this: &Element, child: Element);
    #[wasm_bindgen(method, js_name= removeChild)]
    fn removeTextNode(this: &Element, child: Text);
    #[wasm_bindgen(method, js_name= replaceChild)]
    fn replaceTextWithElement(this: &Element, new_child: &Element, old_child: &Text);
    #[wasm_bindgen(method, js_name= replaceChild)]
    fn replaceElementWithElement(this: &Element, new_child: &Element, old_child: &Element);
    #[wasm_bindgen(method, js_name= replaceChild)]
    fn replaceElementWithText(this: &Element, new_child: &Text, old_child: &Element);
    #[wasm_bindgen(method, js_name= replaceChild)]
    fn replaceTextWithText(this: &Element, new_child: &Text, old_child: &Text);
    #[wasm_bindgen(method, getter)]
    fn childNodes(this: &Element) -> NodeList;

    #[wasm_bindgen(method)]
    fn item(this: &NodeList, index: usize) -> Node;

    #[wasm_bindgen(method, getter)]
    fn nodeType(this: &Node) -> usize;
}

// pub trait Node {

// }

// impl Node for Element {}
// impl Node for Text {}

impl Into<TextOrElem> for Node {
    fn into(self) -> TextOrElem {
        let node_type = self.nodeType();
        let jsv: JsValue = self.into();
        match node_type {
            1 => TextOrElem::Elem(jsv.into()),
            _ => TextOrElem::Text(jsv.into()),
        }
    }
}

pub fn create_element(name: String) -> Element {
    document.createElement(name)
}

pub fn create_text_node(data: String) -> Text {
    document.createTextNode(data)
}

pub fn append_child(node: &Element, child: &TextOrElem) {
    match child {
        &TextOrElem::Text(ref txt) => node.appendTextNode(txt),
        &TextOrElem::Elem(ref elm) => node.appendElementNode(elm),
    }
}

pub fn get_element(sel: &str) -> Element {
    document.querySelector(sel)
}

pub fn remove_child(el: &Element, i: usize) {
    match get_child(el, i) {
        TextOrElem::Text(txt) => el.removeTextNode(txt),
        TextOrElem::Elem(elm) => el.removeElementNode(elm),
    }
}

pub fn get_child(el: &Element, i: usize) -> TextOrElem {
    el.childNodes().item(i).into()
}

pub fn replace_child(
    el: &Element,
    new_child: &TextOrElem,
    old_child: &TextOrElem
) {
    use TextOrElem::*;
    match (new_child, old_child) {
        (Elem(ref new), Elem(ref old)) => el.replaceElementWithElement(new, old),
        (Elem(ref new), Text(ref old)) => el.replaceTextWithElement(new, old),
        (Text(ref new), Text(ref old)) => el.replaceTextWithText(new, old),
        (Text(ref new), Elem(ref old)) => el.replaceElementWithText(new, old)
    }
} 
