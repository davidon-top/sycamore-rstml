use proc_macro2::TokenStream;
use proc_macro2_diagnostics::{Diagnostic, Level};
use quote::ToTokens;
use rstml::node::{Node, NodeName};
use sycamore_view_parser::ir;
use sycamore_view_parser::ir::{DynNode, Root, TagIdent, TagNode, TextNode};
use syn::spanned::Spanned;

pub struct Html(Vec<Diagnostic>);

impl Html {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn impl_html(&mut self, input: TokenStream) -> TokenStream {
        let input = rstml::parse2(input).unwrap();

        let root = Root(input.iter().filter_map(|n| self.node_to_node(n)).collect());

        sycamore_view_parser::codegen::Codegen {}.root(&root)
    }

    fn node_to_node(&mut self, input: &Node) -> Option<ir::Node> {
        match input {
            Node::Comment(_) => None,
            Node::Doctype(_) => None,
            Node::Fragment(_) => None,
            Node::Element(n) => Some(ir::Node::Tag(TagNode {
                ident: match n.name() {
                    NodeName::Path(p) => TagIdent::Path(p.path.clone()),
                    NodeName::Punctuated(p) => {
                        TagIdent::Hyphenated(p.to_token_stream().to_string())
                    }
                    NodeName::Block(p) => {
                        self.0.push(Diagnostic::spanned(
                            p.span(),
                            Level::Error,
                            "Blocks as node names are not supported!",
                        ));
                        TagIdent::Hyphenated(p.to_token_stream().to_string())
                    }
                },
                props: n
                    .attributes()
                    .iter()
                    .map(|a| syn::parse2(a.to_token_stream()).unwrap())
                    .collect(),
                children: Root(
                    n.children
                        .iter()
                        .filter_map(|n| self.node_to_node(n))
                        .collect(),
                ),
            })),
            Node::Block(n) => Some(ir::Node::Dyn(DynNode {
                value: match n.try_block() {
                    None => {
                        self.0.push(Diagnostic::spanned(
                            n.span(),
                            Level::Error,
                            "Invalid code block",
                        ));
                        syn::Expr::Block(syn::ExprBlock {
                            attrs: vec![],
                            label: None,
                            block: syn::Block {
                                brace_token: Default::default(),
                                stmts: vec![],
                            },
                        })
                    }
                    Some(b) => syn::Expr::Block(syn::ExprBlock {
                        attrs: vec![],
                        label: None,
                        block: b.clone(),
                    }),
                },
            })),
            Node::Text(n) => Some(ir::Node::Text(TextNode {
                value: n.value.clone(),
            })),
            Node::RawText(n) => Some(ir::Node::Text(TextNode {
                value: syn::LitStr::new(n.to_token_stream_string().as_str(), n.span()),
            })),
            Node::Custom(_) => None,
        }
    }
}
