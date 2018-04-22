use ::create_node;
use dom_api::{append_child, remove_child, replace_child, get_child, Element, Text, Node};
use std::mem;

pub enum TextOrElem {
    Text(Text),
    Elem(Element),
}

impl Into<TextOrElem> for Text {
    fn into(self) -> TextOrElem {
        TextOrElem::Text(self)
    }
}

impl TextOrElem {
    pub fn as_node(&self) -> Node {
        unsafe {
            mem::transmute_copy(self)
        }
    }
}

impl Into<TextOrElem> for Element {
    fn into(self) -> TextOrElem {
        TextOrElem::Elem(self)
    }
}

#[derive(Clone)]
pub struct VElem {
    pub name: String,
    pub _props: Option<String>,
    pub children: Vec<VNode>,
}

#[derive(Clone)]
pub enum VNode {
    Elem(VElem),
    Text(String),
}

impl Into<VNode> for String {
    fn into(self) -> VNode {
        VNode::Text(self)
    }
}

impl Into<VNode> for &'static str {
    fn into(self) -> VNode {
        VNode::Text(self.into())
    }
}

impl Into<VNode> for VElem {
    fn into(self) -> VNode {
        VNode::Elem(self)
    }
}

impl VNode {
    fn changed(&self, new_node: &Self) -> bool {
        match (self, new_node) {
            (&VNode::Text(ref old), &VNode::Text(ref new)) => old == new,
            (&VNode::Elem(ref old), &VNode::Elem(ref new)) => old.name == new.name,
            (_, _) => true
        }
    }
}

pub fn update_vnode(
    el: &Element,
    new_node: impl Into<Option<VNode>>,
    old_node: impl Into<Option<VNode>>,
    index: impl Into<Option<usize>>,
) {
    let old_node = old_node.into();
    let new_node = new_node.into();
    let index = match index.into() {
        Some(i) => i,
        None => 0,
    };

    match (old_node, new_node) {
        (None, Some(child)) => append_child(el, &create_node(child)),
        (Some(_), None) => (remove_child(el, index)),
        (Some(old_node), Some(new_node)) => {
            if old_node.changed(&new_node) {
                replace_child(el, &get_child(el, index), &create_node(new_node))
            } else {
                if let (VNode::Elem(ref new), VNode::Elem(ref old)) = (new_node, old_node) {
                    let ol = old.children.len();
                    let nl = new.children.len();

                    let l = if ol > nl { ol } else { nl };

                    if let TextOrElem::Elem(ref elm) = get_child(el, index) {   
                        for i in 0..l {
                            update_vnode(elm, new.children[i].clone(),old.children[i].clone(), i)
                        }
                    }
                }
            }
        }
        (None, None) => ()
    }
}
