mod complex_type;
mod schema;

#[cfg(test)]
mod onvif_files_test {
    use crate::xml_to_xsd::schema::parse_document;
    use roxmltree::Document;

    #[test]
    fn test_include() {
        const TEXT: &str = include_str!("../../../../input/xsd/include.xsd");
        let doc = Document::parse(TEXT).unwrap();
        assert!(parse_document(&doc).is_ok());
    }

    #[test]
    fn test_common() {
        const TEXT: &str = include_str!("../../../../input/xsd/common.xsd");
        let doc = Document::parse(TEXT).unwrap();
        assert!(parse_document(&doc).is_ok());
    }

    #[test]
    fn test_humanbody() {
        const TEXT: &str = include_str!("../../../../input/xsd/humanbody.xsd");
        let doc = Document::parse(TEXT).unwrap();
        assert!(parse_document(&doc).is_ok());
    }

    #[test]
    fn test_b2() {
        const TEXT: &str = include_str!("../../../../input/xsd_external/b-2.xsd");
        let doc = Document::parse(TEXT).unwrap();
        println!("{}", parse_document(&doc).err().unwrap());
        assert!(parse_document(&doc).is_ok());
    }

    #[test]
    fn test_ws_discovery() {
        const TEXT: &str = include_str!("../../../../input/xsd_external/ws-discovery.xsd");
        let doc = Document::parse(TEXT).unwrap();
        assert!(parse_document(&doc).is_ok());
    }
}