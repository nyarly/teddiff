type Label = u64;

struct Node {
    label: Label,
    left: Option<Arc<Node>>,
    right: Option<Arc<Node>>,
    num_children: u64,
}

impl Node {
    fn num_children(&self) -> u64 {
        self.num_children
    }

    fn heavy_child(&self) -> Option<Node> {
    }

    fn light_children(&self) -> Iterator<&Node> {
        if self.left.num_children > self.right.num_children {
            self.right.into_iter()
        } else {
            self.left.into_iter()
        }
    }

    fn ltr_preorder(&self) -> Iterator<&Node> {
        self.iter_children().fold(iter::once(self), |i, n| i.chain(n.ltr_preorder()))
    }

    fn rtl_preorder(&self) -> Iterator<&Node> {
        self.iter_children().rev().fold(iter::once(self), |i, n| i.chain(n.rtl_preorder()))
    }

    fn iter_children(&self) -> Iterator<&Node> {
        left.into_iter().chain(right.into_iter());
    }
}

struct Forest {
    roots: Vec<Node>
}

impl Forest {
    fn size() -> u64 {
        roots.fold(0, |acc, node| acc + node.num_children())
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

    fn rtl_preorder(&self) -> impl Iterator<Item = Node> {
        self.roots.into_iter().rev().fold(iter::empty(), |i, r| i.chain(r))
    }
}

struct Tree {
    root: Node
}

impl Tree {
    fn top_light_subtrees(node: Node) -> impl Iterator<Item = Node> {

    }

    // leaf to root
    fn heavy_path() -> impl Iterator<Item = Node> {

    }
}
