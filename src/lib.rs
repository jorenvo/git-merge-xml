#![allow(unused_variables)]

static CONFLICT_MARKER_OPEN: &str = "<<<<<<<";
static CONFLICT_MARKER_MID: &str = "=======";
static CONFLICT_MARKER_CLOSE: &str = ">>>>>>>";

pub fn format_conflict(a: &str, b: &str) -> String {
    String::from(format!(
        "{}
{}
{}
{}
{}",
        CONFLICT_MARKER_OPEN, a, CONFLICT_MARKER_MID, b, CONFLICT_MARKER_CLOSE
    ))
}

pub fn merge_3_way(parent: &str, a: &str, b: &str) -> String {
    let a_modified = !(parent == a);
    let b_modified = !(parent == b);

    if a_modified && b_modified {
        format_conflict(a, b)
    } else if a_modified {
        String::from(a)
    } else if b_modified {
        String::from(b)
    } else {
        String::from(parent)
    }
}
