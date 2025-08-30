use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    // todo!("How will you transform the tree {h:?}?")
    let mut bmap = BTreeMap::new();
    for (&n, v) in h.iter() {
        for &ch in v.iter() {
            bmap.insert(ch.to_ascii_lowercase(), n);
        }
    }

    bmap
}
