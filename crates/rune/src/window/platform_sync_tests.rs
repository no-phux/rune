use super::ImeCursor;

const AREA: [f32; 4] = [12.0, 24.0, 8.0, 17.0];

#[test]
fn unrelated_frames_do_not_commit_an_unchanged_ime_cursor() {
    let mut cursor = ImeCursor::default();
    assert_eq!(cursor.changed(Some(AREA), 1.0), Some(AREA));
    for _ in 0..120 {
        assert_eq!(cursor.changed(Some(AREA), 1.0), None);
    }
}

#[test]
fn moved_or_resized_cursor_still_updates() {
    let mut cursor = ImeCursor::default();
    cursor.changed(Some(AREA), 1.0);
    for index in 0..4 {
        let mut area = AREA;
        area[index] += 1.0;
        assert_eq!(cursor.changed(Some(area), 1.0), Some(area));
        assert_eq!(cursor.changed(Some(area), 1.0), None);
    }
}

#[test]
fn blur_secure_input_or_composition_reset_refreshes_the_same_area() {
    let mut cursor = ImeCursor::default();
    cursor.changed(Some(AREA), 1.0);
    cursor.reset();
    assert_eq!(cursor.changed(Some(AREA), 1.0), Some(AREA));
    assert_eq!(cursor.changed(Some(AREA), 1.0), None);
}

#[test]
fn removed_anchor_and_new_window_do_not_reuse_an_old_context() {
    let mut cursor = ImeCursor::default();
    assert_eq!(cursor.changed(None, 1.0), None);
    assert_eq!(cursor.changed(Some(AREA), 1.0), Some(AREA));
    assert_eq!(cursor.changed(None, 1.0), None);
    assert_eq!(cursor.changed(Some(AREA), 1.0), Some(AREA));
    assert_eq!(ImeCursor::default().changed(Some(AREA), 1.0), Some(AREA));
}

#[test]
fn unchanged_logical_caret_is_resubmitted_after_a_dpi_change_without_refocusing() {
    let mut cursor = ImeCursor::default();
    assert_eq!(cursor.changed(Some(AREA), 1.0), Some(AREA));
    assert_eq!(cursor.changed(Some(AREA), 1.0), None);
    for scale in [1.5, 2.0, 1.0] {
        assert_eq!(cursor.changed(Some(AREA), scale), Some(AREA));
        assert_eq!(cursor.changed(Some(AREA), scale), None);
    }
}
