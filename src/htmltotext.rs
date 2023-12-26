use html5ever::{parse_document, tendril::TendrilSink, ParseOpts};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

struct HtmlToText {
    in_pre_block: bool,
}

const BLOCK_ELEMENTS: [&str; 4] = ["p", "div", "pre", "code"];

impl HtmlToText {
    fn new() -> HtmlToText {
        HtmlToText {
            in_pre_block: false,
        }
    }
    fn process_node(&mut self, node: &Handle) -> String {
        match node.data {
            NodeData::Text { ref contents } => contents.borrow().to_string(),
            NodeData::Element { ref name, .. } => {
                let tag_name = name.local.to_string();

                if tag_name == "pre" {
                    self.in_pre_block = true;
                }

                let content = node
                    .children
                    .borrow()
                    .iter()
                    .map(|child| self.process_node(child))
                    .collect::<Vec<String>>()
                    .join("");

                if tag_name == "pre" {
                    self.in_pre_block = false;
                }

                // 在块级元素后添加换行符
                if BLOCK_ELEMENTS.contains(&tag_name.as_str()) {
                    format!("{}\n", content)
                } else {
                    content
                }
            }
            _ => "".to_string(),
        }
    }
}

pub fn html_to_text(html: &str) -> String {
    let parser = parse_document(RcDom::default(), ParseOpts::default());
    let dom = parser.one(html);
    let mut converter = HtmlToText::new();
    let data = dom
        .document
        .children
        .borrow()
        .iter()
        .map(|child| converter.process_node(child))
        .collect::<Vec<String>>()
        .join("");

    data
}
