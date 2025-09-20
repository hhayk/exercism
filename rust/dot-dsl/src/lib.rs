pub mod graph {
    use std::collections::HashMap;

    use crate::graph::graph_items::{edge::Edge, node::Node};

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            nodes.iter().for_each(|node| self.nodes.push(node.clone()));
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            edges.iter().for_each(|edge| self.edges.push(edge.clone()));
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (key, value) in attrs.iter() {
                self.attrs
                    .entry(key.to_string())
                    .and_modify(|v| *v = value.to_string())
                    .or_insert(value.to_string());
            }
            self
        }

        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == name)
        }
    }

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            use crate::with_attrs;

            #[derive(Debug, PartialEq, Clone, Default)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        ..Default::default()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    with_attrs!(self, attrs);

                    self
                }

                pub fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.iter().find_map(|(key, value)| {
                        if key == name {
                            Some(value.as_str())
                        } else {
                            None
                        }
                    })
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            use crate::with_attrs;

            #[derive(Debug, PartialEq, Clone, Default)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self {
                        from: from.to_string(),
                        to: to.to_string(),
                        ..Default::default()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    with_attrs!(self, attrs);

                    self
                }

                pub fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.iter().find_map(|(key, value)| {
                        if key == name {
                            Some(value.as_str())
                        } else {
                            None
                        }
                    })
                }
            }
        }
    }
}

#[macro_export]
macro_rules! with_attrs {
    ($self:ident, $attrs:ident) => {
        for (key, value) in $attrs.iter() {
            $self
                .attrs
                .entry(key.to_string())
                .and_modify(|v| *v = value.to_string())
                .or_insert(value.to_string());
        }
    };
}
