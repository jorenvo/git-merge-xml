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
fn test_one_line_non_conflicting_merge_1() {
    assert_eq!(merge_3_way("", "mouse", ""), "mouse");
}

#[test]
fn test_one_line_non_conflicting_merge_2() {
    assert_eq!(merge_3_way("mouse", "cat", "mouse"), "cat");
}

#[test]
fn test_one_line_conflict_1() {
    let parent = "mouse";
    let a = "cat";
    let b = "dog";
    assert_eq!(merge_3_way(parent, a, b), format_conflict(a, b));
}
