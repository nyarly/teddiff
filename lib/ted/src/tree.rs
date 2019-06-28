type Label = u64;

trait Node {
    fn iter_children(&self) -> impl DoubleEndedIterator<Item = Node>;

    fn left_preorder(&self) -> impl Iterator<Item = Node> {
        self.iter_children().fold(iter::once(self), |i, n| i.chain(n.left_preorder()) )
    }

    fn right_preorder(&self) -> impl Iterator<Item = Node> {
        self.iter_children().rev().fold(iter::once(self), |i, n| i.chain(n.right_preorder()) )
    }

    fn left_postorder(&self) -> impl Iterator<Item = Node> {
        self.iter_children().fold(iter::once(self), |i, n| n.left_postorder().chain(i) )
    }

    fn right_postorder(&self) -> impl Iterator<Item = Node> {
        self.iter_children().rev().fold(iter::once(self), |i, n| n.right_postorder().chain(i) )
    }
}

struct ANode {
    label: Label,
    children: Vector<&Node>,
}

impl Node for ANode {
    fn iter_children(&self) -> Iterator<&Node> {
        self.children.iter()
    }
}

struct NodeFrame {
    index: usize,
    node: Arc<Node>,
}

fn bedeck(tree: impl Node) -> Vector<NodeFrame> {
    tree.left_postorder.enumerate().map(|(i,n)|{
        NodeFrame{
            index: i
            node: Arc::new(n)
        }
    }).collect()
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
