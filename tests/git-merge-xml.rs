extern crate git_merge_xml;

use git_merge_xml::*;

#[test]
fn test_empty_merge() {
    assert_eq!(merge_3_way("", "", ""), "");
}

#[test]
fn test_no_change_merge() {
    assert_eq!(merge_3_way("mouse", "mouse", "mouse"), "mouse");
}

#[test]
fn test_non_conflicting_merge_1() {
    assert_eq!(merge_3_way("", "mouse", ""), "mouse");
}

#[test]
fn test_non_conflicting_merge_2() {
    assert_eq!(merge_3_way("mouse", "cat", "mouse"), "cat");
}
