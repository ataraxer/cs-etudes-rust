use std::collections::HashMap;
use std::fmt;
use std::cmp;


struct Suffix {
    source_node_index: uint,
    first_char_index: uint,
    last_char_index: int,
}


impl fmt::Show for Suffix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.first_char_index, self.last_char_index)
    }
}


impl Suffix {
    fn len(&self) -> uint {
        match self.last_char_index.to_uint() {
            Some(last_char_index) => last_char_index - self.first_char_index,
            None => 0,
        }
    }

    fn is_explicit(&self) -> bool {
        match self.last_char_index.to_uint() {
            Some(last_char_index) => self.first_char_index > last_char_index,
            None => true,
        }
    }
}


#[deriving(Show)]
struct Node {
    suffix_node: Option<uint>,
}


impl Node {
    fn new() -> Node {
        Node { suffix_node: None }
    }
}


struct Edge {
    first_char_index: uint,
    last_char_index: uint,
    source_node_index: Option<uint>,
    dest_node_index: uint,
}


impl cmp::PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.source_node_index == other.source_node_index &&
        self.dest_node_index == other.dest_node_index
    }

    fn ne(&self, other: &Edge) -> bool {
        !self.eq(other)
    }
}


impl cmp::Eq for Edge {}


impl cmp::PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<cmp::Ordering> {
        let result = if self.source_node_index == other.source_node_index {
            self.dest_node_index.cmp(&other.dest_node_index)
        } else {
            self.source_node_index.cmp(&other.source_node_index)
        };

        Some(result)
    }
}


impl cmp::Ord for Edge {
    fn cmp(&self, other: &Edge) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}


impl fmt::Show for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Edge({}, {}, {}, {})",
            self.first_char_index,
            self.last_char_index,
            self.source_node_index.unwrap_or(-1),
            self.dest_node_index)
    }
}


impl Edge {
    fn len(&self) -> uint {
        self.last_char_index - self.first_char_index
    }
}


struct SuffixTree<'s> {
    string: &'s str,
    active: Suffix,
    edges: HashMap<(Option<uint>, char), Edge>,
    nodes: Vec<Node>,
}


impl<'s> SuffixTree <'s> {
    fn add_prefix(&mut self, last_char_index: uint) {
        let mut last_parent_node: Option<uint> = None;
        let mut parent_node: Option<uint>;

        loop {
            parent_node = Some(self.active.source_node_index);

            if self.active.is_explicit() {
                let link = (
                    Some(self.active.source_node_index),
                    self.string.char_at(last_char_index)
                );

                if self.edges.contains_key(&link) {
                    // prefix is already in tree
                    break;
                }
            } else {
                let link = (
                    Some(self.active.source_node_index),
                    self.string.char_at(self.active.first_char_index),
                );

                if self.edges.contains_key(&link) {
                    let mut edge = self.edges[link];
                    let index = edge.first_char_index + self.active.len() + 1;

                    if self.string.char_at(index) ==
                       self.string.char_at(last_char_index)
                    {
                        // prefix is already in tree
                        break;
                    }

                    parent_node = Some(self.split_edge(&mut edge));
                }
            }

            self.nodes.push(Node::new());

            let edge = Edge {
                first_char_index: last_char_index,
                last_char_index: self.string.len() - 1,
                source_node_index: parent_node,
                dest_node_index: self.nodes.len() - 1,
            };

            self.insert_edge(&edge);

            match last_parent_node {
                Some(node_index) if node_index > 0 => {
                    self.nodes.get_mut(node_index).suffix_node = parent_node;
                }
                _ => {}
            }

            last_parent_node = parent_node;

            if self.active.source_node_index == 0 {
                self.active.first_char_index += 1;
            } else {
                let index = self.active.source_node_index;
                let node_index = self.nodes[index].suffix_node.unwrap();
                self.active.source_node_index = node_index;
            }

            self.canonize_suffix(&self.active);
        }

        match last_parent_node {
            Some(node_index) if node_index > 0 => {
                let mut node  = self.nodes[node_index];
                node.suffix_node = parent_node;
            }
            _ => {}
        }

        self.active.last_char_index += 1;
        self.canonize_suffix(&self.active);
    }


    fn canonize_suffix(&self, suffix: &Suffix) {
        if !suffix.is_explicit() {
            let link = (
                Some(suffix.source_node_index),
                self.string.char_at(suffix.first_char_index),
            );

            if self.edges.contains_key(&link) {
                let edge = self.edges[link];

                if edge.len() <= suffix.len() {
                    self.canonize_suffix(suffix);
                }
            }
        }
    }


    fn remove_edge(&mut self, edge: &Edge) {
        let link = (
            edge.source_node_index,
            self.string.char_at(edge.first_char_index),
        );

        self.edges.remove(&link);
    }


    fn insert_edge(&mut self, edge: &Edge) {
        let link = (
            edge.source_node_index,
            self.string.char_at(edge.first_char_index),
        );

        self.edges.insert(link, *edge);
    }


    fn split_edge(&mut self, edge: &mut Edge) -> uint {
        self.nodes.push(Node::new());

        let new_edge = Edge {
            first_char_index: edge.first_char_index,
            last_char_index: edge.first_char_index + self.active.len(),
            source_node_index: Some(self.active.source_node_index),
            dest_node_index: self.nodes.len() - 1,
        };

        self.remove_edge(edge);
        self.insert_edge(&new_edge);
        // need to add node for each edge
        let mut dest_node = self.nodes[new_edge.dest_node_index];
        dest_node.suffix_node = Some(self.active.source_node_index);

        edge.first_char_index += self.active.len() + 1;
        edge.source_node_index = Some(new_edge.dest_node_index);
        self.insert_edge(edge);

        new_edge.dest_node_index
    }


    fn new(input: &str) -> SuffixTree {
        let mut tree = SuffixTree {
            string: input,
            active: Suffix {
                source_node_index: 0,
                first_char_index: 0,
                last_char_index: -1,
            },
            edges: HashMap::new(),
            nodes: vec![Node::new()],

        };

        for index in range(0u, input.len()) {
            tree.add_prefix(index);
        }

        tree
    }

}


impl<'s> fmt::Show for SuffixTree<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //let curr_index = self.string.len();
        let s = "\tStart \tEnd \tSuf \tFirst \tLast \tString";
        let mut result = String::new();
        let mut values: Vec<&Edge> = Vec::new();

        for edge in self.edges.values() {
            values.push(edge);
        }

        let sorted_values = values.as_mut_slice();
        sorted_values.sort();

        for edge in sorted_values.iter() {
            if edge.source_node_index.is_none() { continue; }

            let suffix = match self.nodes[edge.dest_node_index].suffix_node {
                None => format!("-1"),
                Some(s) => format!("{}", s),
            };

            let string = match self.nodes[edge.dest_node_index].suffix_node {
                None => self.string.slice(
                        edge.first_char_index, self.string.len()),
                Some(suffix) => self.string.slice(
                        edge.first_char_index, edge.last_char_index + 1),
            };

            result = result + format!("\t{} \t{} \t{} \t{} \t{} \t{} \n",
                edge.source_node_index.unwrap_or(0),
                edge.dest_node_index,
                suffix,
                edge.first_char_index,
                edge.last_char_index,
                string);
        }

        write!(f, "{}\n{}", s, result)
    }
}


fn main() {
    let tree1 = SuffixTree::new("xxz");
    println!("{}", tree1);
    let tree2 = SuffixTree::new("abcabcd");
    println!("{}", tree2);
}
