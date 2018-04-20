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

pub struct Elem {
    pub name: String,
    pub _props: Option<String>,
    pub children: Vec<Vnode>
}

pub enum Vnode {
    Elem(Elem),
    Text(String)
}

impl Into<Vnode> for String  {
    fn into(self) -> Vnode {
        Vnode::Text(self)
    }
}

impl Into<Vnode> for &'static str  {
    fn into(self) -> Vnode {
        Vnode::Text(self.into())
    }
}

impl Into<Vnode> for Elem  {
    fn into(self) -> Vnode {
        Vnode::Elem(self)
    }
}