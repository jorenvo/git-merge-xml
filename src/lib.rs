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

fn merge_line_3_way(parent: &str, a: &str, b: &str) -> String {
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

pub fn merge_3_way(parent: &str, a: &str, b: &str) -> String {
    // can't use split_terminator because "a\n" and "a" are indistinguishable after
    let lines = parent
        .split('\n')
        .zip(a.split('\n'))
        .zip(b.split('\n'))
        .map(|((parent, a), b)| (parent, a, b));

    let mut merged = String::new();
    let mut first_loop = true;
    for (line_parent, line_a, line_b) in lines {
        // We are here because the *previous line* ended with a
        // newline. Putting this at the end of the loop is incorrect
        // because it would assume that the current line ends with a
        // newline which is not always the case.
        if !first_loop {
            merged.push('\n');
        } else {
            first_loop = false;
        }

        merged.push_str(&merge_line_3_way(line_parent, line_a, line_b));
    }

    merged
}
