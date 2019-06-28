mod tree;
mod demaine;
mod shasha;
mod rted;

type Cost = u64;

type Table = Vec<Vec<Option<Cost>>>

pub fn exact(left: impl tree::Node, right: impl tree::Node) -> Cost {
    rted::compute(left, right)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
