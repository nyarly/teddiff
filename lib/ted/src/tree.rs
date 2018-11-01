type Label = u64;

struct Node {
    label: Label,
    value: Value,
}

impl Node {
    fn size(&self) -> u64 {
        match self.value {
            Simple(_) => 0,
            Node(n) => n.num_children,
        }
    }

    fn num_children(&self) -> u64 {
        match self.value {
            Simple(_) => 0,
            Node(n) => n.num_children,
        }
    }

    fn heavy_child(&self) -> Option<Node> {
        match self.value {
            Simple(_) => None,
            Node(n) => n.heavy_child(),
        }
    }

    fn light_children(&self) -> Vec<Node> {
        match self.value {
            Simple(_) => vec![],
            Node(n) => n.light_children(),
        }
    }

    fn children(&self) -> Iterator<Value> {
        match self.value {
            Simple(_) => iter::empty(),
            Node(n) => n.iter_children(),
        }
    }

    fn ltr_preorder(&self) -> Iterator<&Node> {
        self.children().fold(iter::once(self), |i, n| i.chain(n.ltr_preorder()))
    }

    fn rtl_preorder(&self) -> Iterator<&Node> {
        self.children().rev().fold(iter::once(self), |i, n| i.chain(n.rtl_preorder()))
    }
}

enum Value {
    Simple(SimpleValue),
    Node(NodeValue),
}

type SimpleValue u64;

struct NodeValue {
    left: Option<Arc<Node>>,
    right: Option<Arc<Node>>,
    num_children: u64,
}

impl NodeValue {
    fn iter_children(&self) -> Iterator<Value> {
        left.into_iter().chain(right.into_iter());
    }

    fn heavy_child(&self) -> Option<Node> {
    }

    fn light_children(&self) -> Vec<Node> {
    }
}


struct Forest {
    roots: Vec<Node>
}

impl Forest {
    fn size() -> u64 {
        roots.fold(0, |acc, node| acc + node.children())
    }

    // The "row size" of a subtree enumeration
    // Defined at 2:15 of Demaine et al
    // XXX Strategy dependent - this is the left impl
    fn j(&self, i u64) -> u64 {
        self.size() - i - self.subtree_at(i).size()
    }

    fn subtree_at(i: u64) -> Node {
        // consider caching this...
        rtl_preorder(self).nth(i)
    }

    fn right_subforest(&self) -> Forest {
        let mut roots = self.roots.clone();
        let rightmost = roots.pop();
        for c in rightmost.children() {
            root.append(c);
        }
    }

    // Probably want to conside a Deque instead of Vec for this
    // because the left inserts are spendy
    fn left_subforest(&self) -> Forest {
        let mut roots = self.roots.clone();
        let leftmost = roots.remove(0);
        for (i, c) in leftmost.children().enumerate() {
            root.insert(i, c);
        }
    }
}


fn top_light_subtrees(node: Node) -> impl Iterator<Item = Node> {

}

fn heavy_path() -> impl Iterator<Item = Node> {

}

fn rtl_preorder(f: &Forest) -> impl Iterator<Item = Node> {
    f.roots.into_iter().rev().fold(iter::empty(), |i, r| i.chain(r))
}
