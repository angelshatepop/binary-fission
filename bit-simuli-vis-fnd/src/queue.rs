use std::collections::BTreeMap;

pub fn add_alphachildren(child1: (Vec<u8>, u32), child2: (Vec<u8>, u32), queue: &mut BTreeMap<Vec<u8>,u32>) {
    queue.insert(
        child1.0,
        child1.1);
    queue.insert(
        child2.0,
        child2.1);
}

pub fn add_parents(mut parent_a: (Vec<u8>, u32), mut parent_b: (Vec<u8>, u32), queue: &mut BTreeMap<Vec<u8>,u32>) {
    parent_a.1 -= parent_a.1 / 100 * 15;
    parent_b.1 -= parent_b.1 / 100 * 15;

    queue.insert(
        parent_a.0,
        parent_a.1);
    queue.insert(
        parent_b.0,
        parent_b.1);
}

pub fn _add_child(child: (Vec<u8>, u32), queue: &mut BTreeMap<Vec<u8>,u32>) {
    queue.insert(
        child.0,
        child.1);
}