use super::ImeCursor;

const RECTANGLE: [i32; 4] = [262, 48, 8, 17];

#[test]
fn first_publication_is_not_suppressed() {
    assert!(!ImeCursor::default().unchanged(RECTANGLE, 1.0));
}

#[test]
fn unrelated_frames_preserve_the_published_rectangle() {
    let mut cursor = ImeCursor::default();
    cursor.remember(RECTANGLE, 1.0);
    for _ in 0..120 {
        assert!(cursor.unchanged(RECTANGLE, 1.0));
    }
}

#[test]
fn each_position_and_size_component_changes_the_key() {
    let mut cursor = ImeCursor::default();
    cursor.remember(RECTANGLE, 1.0);
    for index in 0..4 {
        let mut rectangle = RECTANGLE;
        rectangle[index] += 1;
        assert!(!cursor.unchanged(rectangle, 1.0));
        cursor.remember(rectangle, 1.0);
        assert!(cursor.unchanged(rectangle, 1.0));
        cursor.remember(RECTANGLE, 1.0);
    }
}

#[test]
fn dpi_changes_refresh_an_identical_protocol_rectangle() {
    let mut cursor = ImeCursor::default();
    cursor.remember(RECTANGLE, 1.0);
    for scale in [1.5, 2.0, 1.0] {
        assert!(!cursor.unchanged(RECTANGLE, scale));
        cursor.remember(RECTANGLE, scale);
        assert!(cursor.unchanged(RECTANGLE, scale));
    }
}

#[test]
fn context_reset_republishes_the_same_rectangle_once() {
    let mut cursor = ImeCursor::default();
    cursor.remember(RECTANGLE, 1.0);
    cursor.reset();
    assert!(!cursor.unchanged(RECTANGLE, 1.0));
    cursor.remember(RECTANGLE, 1.0);
    assert!(cursor.unchanged(RECTANGLE, 1.0));
}

#[test]
fn windows_do_not_share_publication_state() {
    let mut first = ImeCursor::default();
    let mut second = ImeCursor::default();
    first.remember(RECTANGLE, 1.0);
    assert!(!second.unchanged(RECTANGLE, 1.0));
    second.remember(RECTANGLE, 1.0);
    first.reset();
    assert!(second.unchanged(RECTANGLE, 1.0));
}
