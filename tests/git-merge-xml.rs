extern crate git_merge_xml;

use git_merge_xml::*;

#[test]
fn test_one_line_empty_merge() {
    assert_eq!(merge_3_way("", "", ""), "");
}

#[test]
fn test_one_line_no_change_merge() {
    assert_eq!(merge_3_way("mouse", "mouse", "mouse"), "mouse");
}

#[test]
fn test_one_line_no_conflict_1() {
    assert_eq!(merge_3_way("", "mouse", ""), "mouse");
}

#[test]
fn test_one_line_no_conflict_2() {
    assert_eq!(merge_3_way("mouse", "cat", "mouse"), "cat");
}

#[test]
fn test_one_line_conflict_1() {
    let parent = "mouse";
    let a = "cat";
    let b = "dog";
    assert_eq!(merge_3_way(parent, a, b), format_conflict(a, b));
}

#[test]
fn test_multi_line_empty() {
    let content = &format!("\n\n\n");
    assert_eq!(merge_3_way(content, content, content), *content);
}

#[test]
fn test_multi_line_no_conflict_1() {
    let parent = &format!("pangolin\npenguin\nelephant");
    let a = &format!("pangolin\npolar bear\nelephant");
    let b = &format!("pangolin\npenguin\nelephant");
    assert_eq!(merge_3_way(parent, a, b), *a);
}

#[test]
fn test_multi_line_no_conflict_2() {
    let parent = &format!("pangolin\npenguin\nelephant");
    let a = &format!("pangolin\npolar bear\nelephant");
    let b = &format!("pangolin\npenguin\nparrot");
    assert_eq!(
        merge_3_way(parent, a, b),
        format!("pangolin\npolar bear\nparrot")
    );
}

#[test]
fn test_multi_line_conflict_1() {
    let parent = &format!("pangolin\npenguin\nelephant");
    let a = &format!("pangolin\npolar bear\nelephant");
    let b = &format!("pangolin\nquokka\nelephant");
    assert_eq!(
        merge_3_way(parent, a, b),
        format!(
            "pangolin\n{}elephant",
            format_conflict("polar bear\n", "quokka\n")
        )
    );
}

#[test]
fn test_multi_line_delete_line_1() {
    let parent = &format!("bear\nbunny");
    let a = &format!("bear");

    assert_eq!(merge_3_way(parent, a, parent), *a);
}

#[test]
fn test_multi_line_delete_line_2() {
    let parent = &format!("bear\nbunny\nhedgehog");
    let a = &format!("bear\nhedgehog");

    assert_eq!(merge_3_way(parent, a, parent), *a);
}
