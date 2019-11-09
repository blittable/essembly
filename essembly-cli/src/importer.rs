use roxmltree;

pub trait Parser {
    fn new() -> Self;
    fn parse(&self, content: String);
}

pub struct XLBRParser {}

impl Parser for XLBRParser {
    fn new() -> XLBRParser {
        XLBRParser {}
    }

    fn parse(&self, content: String) {
        let doc = match roxmltree::Document::parse(&content) {
            Ok(doc) => doc,
            Err(e) => {
                println!("Error: {}.", e);
                return;
            }
        };

        let attrs_count: usize = doc.root().descendants().map(|n| n.attributes().len()).sum();
        println!("Attributes count: {}", attrs_count);

        for node in doc.descendants() {
            if node.is_element() {
                match node.tag_name().name().as_ref() {
                    "accountingEntries" => println!("Entries"),
                    "entryHeader" => println!("Header"),
                    "entryDetail" => println!("Detail"),
                    _ => (),
                }

                // println!("Type: {:#?}", node.node_type());
                // println!("Attributes: {:#?}", node.attributes());
                // println!(
                //     "{:?} at {}",
                //     node.tag_name(),
                //     doc.text_pos_at(node.range().start)
                // );
            }
        }
    }
}
