use ::dom_api::*;

pub enum TextOrElem {
    Text(Text),
    Elem(Element)
}

impl Into<TextOrElem> for Text  {
    fn into(self) -> TextOrElem {
        TextOrElem::Text(self)
    }
}

impl Into<TextOrElem> for Element  {
    fn into(self) -> TextOrElem {
        TextOrElem::Elem(self)
    }
}

pub struct VElem {
    pub name: String,
    pub _props: Option<String>,
    pub children: Vec<VNode>
}

pub enum VNode {
    Elem(VElem),
    Text(String)
}

impl Into<VNode> for String  {
    fn into(self) -> VNode {
        VNode::Text(self)
    }
}

impl Into<VNode> for &'static str  {
    fn into(self) -> VNode {
        VNode::Text(self.into())
    }
}

impl Into<VNode> for VElem  {
    fn into(self) -> VNode {
        VNode::Elem(self)
    }
}