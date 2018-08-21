enum Strategy {
    Left,
    Right,
}

struct Delta;

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


type Cost = u64;

fn ted(left: Forest, right: Forest) -> DeltaTable {
    if left.size() < right.size() {
        return ted(right, left);
    }

    for v in left.top_light() {
        ted(left.subtree_at(v), right)
    }

    for v in left.heavy_path() {
        fill_delta(left, v, right)
    }
}

impl Delta {
    fn compute_on(&mut self, left: Forest, right: Forest) {
        if left.size() < right.size() {
            return self.compute_on(right, left);
        }

        for v in left.top_light_subtrees() {
            self.compute_on(v, right)
        }

        for v in left.heavy_path() {
            self.fill_delta(left, v, right)
        }
    }

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

struct Node {
    left: Option<Arc<Node>>,
    right: Option<Arc<Node>>,
    num_children: u64,
}

struct Forest {
    roots: Vec<Node>
}

impl Forest {
    fn size() -> u64 {
        roots.fold(0, |acc, node| acc + node.children)
    }

    fn subforest_enum(&self, which: Strategy) -> SubforestEnum {
        SubforestEnum { which }
    }

    fn subforest(i: u64, j: u64) -> Forest {}

}

struct SubforestEnum {
    which: Strategy,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
