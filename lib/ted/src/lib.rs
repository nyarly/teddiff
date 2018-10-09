type Label = u64;

type Cost = u64;

struct Node {
    label: Label,
    value: Value,
}

impl Node {
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

    fn subforest_enum(&self, which: Strategy) -> SubforestEnum {
        SubforestEnum { which }
    }

    fn subforest(i: u64, j: u64) -> Forest {}
}


fn top_light_subtrees(node: Node) -> impl Iterator<Item = Node> {

}

fn heavy_path() -> impl Iterator<Item = Node> {

}

struct SubforestEnum {
    which: Strategy,
}

enum Strategy {
    Left,
    Right,
}

impl Iterator for SubforestEnum {
    Item = Enumerated;

    fn next(&mut self) -> Option<Self::Item> {
    }
}

struct Enumerated {
    forest: Forest,
}

struct Delta {
    f: Forest,
    g: Forest,
}

impl Delta {
    fn compute(f: Forest, g: Forest) -> Cost {
        Delta{ f, g }.compute();
    }

    fn compute(&mut self) -> Cost {
        if self.f.size() < self.g.size() {
            return Delta(f: g, g: f).compute()
        }

        for v in self.f.top_light_subtrees() {
            Delta(f: v, g: self.g).compute() // to fill tables...
        }

        for v in self.f.heavy_path() {
            self.fill_delta(v)
        }
    }
}

struct Period {
    delta: Delta
}

struct STable {
    period: Period,
    delta: Delta,
}

impl STable {
    fn new(i: u64) -> STable {}
}


fn ted(left: Forest, right: Forest) -> DeltaTable {
}

impl Period {
    fn compute(&mut self, vp: Node, which: Strategy, left: Forest, right: Forest) {
        for i in g.size()..=0 {
            which.s_table(i, left, right)
        }
    }
}

impl Strategy {
    fn s_table(&self, i: u64, left: Forest, right: Forest) -> STable {
        for f in self.intermediate_subforest_enum(left) {
            for g in self.subforest_enum(right, i) {
                self.del_f(f).cost() + stored_ted_kmo(),
                self.def_g(g).cost() + stored_ted_jpo(),
                self.rename(f,g).cost() + stored_ted_circles() + self.stored_ted_minus_trees()
            }
        }
    }
}

fn stored_ted_kmo() -> Cost {
    if k == 1 {
        a
    } else {
        s_table.get(k - 1, j)
    }
}

fn stored_ted_jpo() -> Cost {
    if j == j(i) {
        if i + j(i) == right.size() {
            ted(left, NullTree)
        } else {
            q.get(j)
        }
    } else {
        s_table.get(k, j + 1)
    }
}

fn stored_ted_circles() -> Cost {
    let v = get_delta_left_index(f);
    let w = get_delta_right_index(g);
    delta_table.get(v,w)
}

fn stored_ted_minus_trees() -> Cost {
    if j == j(i) {
        del_forest(f)
    } else {
        s_table.get(k - f.left_tree().size(), j + g.left_tree().size())
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
