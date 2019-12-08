use crate::xsd2::utils::{MaxOccurs, MinOccurs, get_documentation};


pub struct Sequence<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input: 'a> Sequence<'a, 'input> {

    pub fn max_occurs(&self) -> Option<MaxOccurs> {
        match self.node.attribute("MaxOccurs") {
            Some(v) => match v {
                "unbounded" => Some(MaxOccurs::Unbounded),
                v => v.parse::<usize>().ok().map(|val| MaxOccurs::Bounded(val))
            },
            None => None
        }
    }

    pub fn min_occurs(&self) -> Option<MinOccurs> {
        self.node.attribute("MinOccurs").map(|v| v.parse::<usize>().ok()).flatten()
    }

    pub fn elements(&self) -> Vec<Element> {
        self.node.
            children().
            filter(|node| node.is_element() && node.tag_name().name() == "element").
            map(|node| Element{node}).
            collect::<Vec<Element>>()
    }
}

pub struct Element<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input> Element<'a, 'input> {
    pub fn name(&self) -> Option<&'a str> {
        self.node.attribute("name")
    }
    pub fn documentation(&self) -> Option<&'a str> {
        get_documentation(&self.node)
    }
    pub fn typename(&self) -> Option<&'a str> {
        self.node.attribute("type")
    }
}