pub mod annotation;
pub mod any;
pub mod app_info;
pub mod attribute;
pub mod documentation;
pub mod explicit_group;
pub mod facets;
pub mod group;
pub mod import;
pub mod include;
pub mod list;
pub mod nested_particle;
pub mod restriction;
pub mod schema;
pub mod schema_top;
pub mod simple_type;
pub mod union;
pub mod utils;
pub mod complex_type;
pub mod type_def_particle;

use crate::xsd_model::elements::{xsd_element_type, ElementType};
use crate::xsd_model::simple_types::any_uri::AnyUri;
use crate::xsd_model::simple_types::id::Id;
use crate::xsd_model::simple_types::language::Language;
use crate::xsd_model::simple_types::ncname::NCName;
use roxmltree::Attribute;

pub const XSD_NS_URI: &str = "http://www.w3.org/2001/XMLSchema";

pub trait XsdNode {
    fn xsd_type(&self) -> Result<ElementType, String>;
}

impl XsdNode for roxmltree::Node<'_, '_> {
    fn xsd_type(&self) -> Result<ElementType, String> {
        if let Some(uri) = self.tag_name().namespace() {
            if uri != XSD_NS_URI {
                return Err(format!(
                    "Invalid prefix for xsd element: {:?}",
                    self.tag_name()
                ));
            }
        }

        xsd_element_type(self.tag_name().name())
    }
}

macro_rules! impl_from_attr {
    ($type_name:ident) => {
        impl<'a> From<&'a Attribute<'a>> for $type_name<'a> {
            fn from(a: &'a Attribute<'a>) -> Self {
                $type_name(a.value())
            }
        }
    };
}

impl_from_attr!(AnyUri);
impl_from_attr!(Language);
impl_from_attr!(Id);
impl_from_attr!(NCName);
