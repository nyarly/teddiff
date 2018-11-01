mod tree;

type Cost = u64;

pub fn ted(left: tree::Forest, right: tree::Forest) -> Cost {
    Delta::compute(left, rigth)
}

// From F[i,j(i)] to F[i,0]
struct PartialSubforestEnum {
    which: Strategy,
}

impl PartialSubforestEnum {

}

impl Iterator for PartialSubforestEnum {
    Item = tree::Forest;

    fn next(&mut self) -> Option<Self::Item> {
    }
}

//
struct ItermediateSubforestEnum {
    which: Strategy,
}

impl Iterator for IntermediateSubforestEnum {
    Item = tree::Forest;

    fn next(&mut self) -> Option<Self::Item> {
    }
}

struct Delta {
    f: tree::Forest,
    g: tree::Forest,
}

impl Delta {
    fn compute(f: tree::Forest, g: tree::Forest) -> Cost {
        Delta{ f, g }.compute();
    }

    fn compute(&mut self) -> Cost {
        if self.f.size() < self.g.size() {
            return Delta{f: g, g: f}.compute()
        }

        for v in self.f.top_light_subtrees() {
            Delta{f: v, g: self.g}.compute() // to fill tables...
        }

        for v in self.f.heavy_path() {
            self.fill_delta(v)
        }
    }
}

struct Table {
    data: Vec<Vec<Cost>>
}

// Will want to make the get/set a trait and add a Transpose wrapper
impl Table {
    fn new(i usize, j usize) -> Table {
        data = Vec::with_capacity(i);
        for idx in 0..i {
            data.push(Vec::with_capacity(j))
        }

        Table{ data }
    }


    fn get(&self, i usize, j usize) -> Cost {
        // algorithmically, we know the sizes of these tables
        // a priori, so accesses outside of the range are unexpected.
        data.get(i).unwrap().get(j).unwrap()
    }

    fn set(&mut self, i usize, j usize, c Cost) {
        // algorithmically, we know the sizes of these tables
        // a priori, so accesses outside of the range are unexpected.
        *(data.get_mut(i).unwrap().gem_mut(j).unwrap()) = c;
    }
}


struct STable {
    period: Period,
    delta: Delta,
}

impl STable {
    fn new(i: u64) -> STable {}
}

struct Costs {
}

impl Costs {
    fn delete(n: tree::Node) -> Cost {
        1
    }

    fn rename(f: tree::Node, g: tree::Node) -> Cost {
        if f.label == g.label {
            0
        } else {
            1
        }
    }
}

struct Period {
    left: &tree::Forest,
    right: &tree::Forest,
    delta: &Delta,
    s: STable,
    t: TTable,
    q: QTable,
}

impl Period {
    fn new() -> Period {
        Period{}
    }

    fn compute(&mut self, vp: tree::Node) {
        for i in g.size()..=0 {
            let s = self.s_table(i, left, right);
            self.update_t(i, s);
            self.update_q(s);
            self.update_delta(s);
        }
    }

    fn s_table(&self, i: u64) -> STable {
        let j_i = right.j(i);
        for (k, f) in self.intermediate_subforest_enum(left).enumerate() {
            for (h, g) in self.partial_subforest_enum(right, i).enumerate() {
                let j = j_i - h;
                [
                    self.del_f(f).cost() + self.kmo(),
                    self.def_g(g).cost() + self.jpo(),
                    self.rename(f,g).cost() + self.circles() + self.minus_trees()
                ].iter().min().unwrap()
            }
        }
    }

    fn kmo(&self, j u64, k u64) -> Cost {
        if k == 1 {
            a
        } else {
            s_table.get(k - 1, j)
        }
    }

    fn jpo(&self, i: u64, j: u64, j_i: u64) -> Cost {
        if j == j_i {
            if i + j_i == right.size() {
                ted(self.left, NullTree)
            } else {
                self.q.get(j)
            }
        } else {
            self.s.get(k, j + 1)
        }
    }

    fn circles(&self) -> Cost {
        let v = get_delta_left_index(f);
        let w = get_delta_right_index(g);
        self.delta.get(v,w)
    }

    fn minus_trees(&self, j: u64, j_i: u64, k: u64) -> Cost {
        if j == j_i {
            del_forest(f)
        } else {
            self.s.get(k - f.left_tree().size(), j + g.left_tree().size())
        }

    }

}

enum Strategy {
    Left,
    Right,
}

impl Strategy {
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
