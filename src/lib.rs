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
    let a_modified = parent != a;
    let b_modified = parent != b;

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

fn split_inclusive(content: &str) -> Vec<String> {
    // This function is similar to split(), but it retains
    // newlines. With split() newlines are stripped and because of
    // that we can't determine if a line had a newline or if we reach
    // the end of the content.
    //
    // TODO: maybe turn this into an iterator
    let mut result: Vec<String> = Vec::new();
    let mut start = 0;

    for (i, _) in content.match_indices("\n") {
        result.push(String::from(&content[start..i + 1]));
        start = i + 1;
    }

    if start < content.len() {
        result.push(String::from(&content[start..content.len()]));
    }

    result
}

pub fn merge_3_way(parent: &str, a: &str, b: &str) -> String {
    let mut merged = String::new();

    let split_parent = split_inclusive(parent);
    let split_a = split_inclusive(a);
    let split_b = split_inclusive(b);

    let mut parent_iter = split_parent.iter();
    let mut a_iter = split_a.iter();
    let mut b_iter = split_b.iter();

    loop {
        let line_parent = parent_iter.next();
        let line_a = a_iter.next();
        let line_b = b_iter.next();

        if line_a.is_none() && line_b.is_none() {
            break;
        }

        merged.push_str(&merge_line_3_way(
            line_parent.unwrap_or(&String::from("")),
            line_a.unwrap_or(&String::from("")),
            line_b.unwrap_or(&String::from("")),
        ));
    }

    merged
}
