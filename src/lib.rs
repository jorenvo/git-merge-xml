#![allow(unused_variables)]

pub fn merge_3_way(parent: &str, a: &str, b: &str) -> String {
    let a_modified = !(parent == a);
    let b_modified = !(parent == b);

    if a_modified && b_modified {
        // todo return conflict
        String::from("conflict")
    } else if a_modified {
        String::from(a)
    } else if b_modified {
        String::from(b)
    } else {
        String::from(parent)
    }
}
