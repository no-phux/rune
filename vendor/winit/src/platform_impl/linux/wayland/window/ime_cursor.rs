//! The last cursor rectangle actually published to this window's text inputs.

#[derive(Default)]
pub(crate) struct ImeCursor {
    last: Option<([i32; 4], u64)>,
}

impl ImeCursor {
    pub(crate) fn unchanged(&self, rectangle: [i32; 4], scale_factor: f64) -> bool {
        // Other Unix Wayland targets retain their existing publication behavior.
        cfg!(any(target_os = "linux", test))
            && self.last == Some((rectangle, scale_factor.to_bits()))
    }

    pub(crate) fn remember(&mut self, rectangle: [i32; 4], scale_factor: f64) {
        self.last = Some((rectangle, scale_factor.to_bits()));
    }

    pub(crate) fn reset(&mut self) {
        self.last = None;
    }
}

#[cfg(test)]
#[path = "ime_cursor_tests.rs"]
mod tests;
