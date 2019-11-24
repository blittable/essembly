use roxmltree;
use roxmltree::Node;

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
                    "entryHeader" => {
                        println!("Header");
                        hunt_type(&node);
                    }
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

            //let p = doc.descendants().find(|n| n.has_tag_name("p")).unwrap();
            // if node.has_attribute() {
            //     println!("Type: {:#?}", node.node_type());
            // }
        }
    }
}

fn hunt_type(root_node: &Node) {
    //let mut uris = HashMap::new();

    for node in root_node.children() {
        println!("type {:?}", node.node_type());
        println!("tag {:?}", node.tag_name().name());

        match node.tag_name().name().as_ref() {
            "entryDetail" => {
                println!("Recursing");
                hunt_type(&node);
            }
            _ => (),
        }
    }
}

// for node in doc.root().descendants() {

// for ns in node.children() {
//     println!("val {:?}", ns);
//     uris.insert(ns.text(), "text");
// }
// }

// println!("name {:?}", node.tag_name().name());
// println!("Child: {:?}", node.node_type());

// fn parse_xml_children(node: &Node) {
//     println!("Children **********************************");

//     let details = node
//         .descendants()
//         .find(|n| n.has_tag_name("entryDetail"))
//         .unwrap();

//     for child in details.children() {
//         println!("Child: {:#?}", child.node_type());
//         println!("Attributes: {:#?}", child.attributes());
//     }
// }

// Type: Element
// Attributes: [
//     Attribute { name: contextRef, value: "now" },
//     Attribute { name: decimals, value: "2" },
//     Attribute { name: unitRef, value: "usd" },
