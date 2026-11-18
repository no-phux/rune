use winit::dpi::{LogicalPosition, LogicalSize};

use crate::runtime::Runtime;
use super::Win;

/// What a built frame hands the window besides its pixels: secure input, the IME, the cursor,
/// help tags and the clipboard.
pub(super) fn sync_platform<S: 'static>(l: &mut Win, rt: &mut Runtime<S>, secure_input: &mut crate::input::secure_input::SecureInput,
    ime: &mut bool, clipboard: &crate::clipboard::Client) {
    let focused = l.window.has_focus();
    let ime_allowed = sync_input_focus(l, rt, secure_input, ime, focused);
    // A terminal's caret asks for Option as Alt (`El::ime_option_as_alt`): AppKit then hands the key
    // under Option over as itself, not the character the layout composes with it.
    #[cfg(target_os = "macos")]
    {
        use winit::platform::macos::{OptionAsAlt, WindowExtMacOS};
        let want = if ime_allowed && rt.ime_option_as_alt { OptionAsAlt::Both } else { OptionAsAlt::None };
        if l.window.option_as_alt() != want {
            l.window.set_option_as_alt(want);
        }
    }
    l.apply_cursor(rt, focused);
    #[cfg(target_os = "macos")]
    if rt.zoom() == 1.0 {
        l.native.help.sync(&l.window, rt.help_regions());
    } else {
        // Window points: the view's times its zoom.
        let regions: Vec<([f32; 4], String)> = rt.help_regions().iter().map(|(r, t)| (rt.to_window(*r), t.clone())).collect();
        l.native.help.sync(&l.window, &regions);
    }
    // The process has one menu theme: the active window's, so its window menu matches it.
    #[cfg(target_os = "windows")]
    if focused { crate::platform::win32::dark_mode::menus(rt.backdrop_dark().unwrap_or(rt.env.system_dark)); }
    clipboard.flush(rt);
    // The focused field's text, for the input method to convert again or read around the caret.
    crate::platform::ime::sync(&l.window, if ime_allowed { rt.ime_text() } else { None });
    l.sync_ime_cursor(rt.ime_area.map(|a| rt.to_window(a)));
}

fn sync_input_focus<S: 'static>(l: &mut Win, rt: &mut Runtime<S>, secure_input: &mut crate::input::secure_input::SecureInput,
    ime: &mut bool, focused: bool) -> bool {
    secure_input.set(rt.has_secure_focus() && focused);
    let ime_allowed = rt.ime_area.is_some() && !rt.has_secure_focus() && focused;
    let reset = std::mem::take(&mut rt.ime_reset);
    // Rune ended a composition itself: the input method drops its copy too.
    if reset { crate::platform::ime::cancel(&l.window); }
    if reset || ime_allowed != *ime {
        #[cfg(target_os = "linux")]
        l.native.ime_cursor.reset();
        l.window.set_ime_allowed(false);
        l.window.set_ime_allowed(ime_allowed);
        *ime = ime_allowed;
    }
    ime_allowed
}

/// Wayland commits a new text-input serial for every cursor rectangle, even an
/// unchanged one. Sending those on unrelated paint frames invalidates fcitx's
/// in-flight preedit/commit batches. A new input context still needs its rectangle.
#[cfg(any(target_os = "linux", test))]
#[derive(Default)]
pub(super) struct ImeCursor {
    last: Option<([f32; 4], f64)>,
}

#[cfg(any(target_os = "linux", test))]
impl ImeCursor {
    pub(super) fn reset(&mut self) { self.last = None; }

    fn changed(&mut self, area: Option<[f32; 4]>, scale: f64) -> Option<[f32; 4]> {
        // X11 converts logical coordinates using the current DPI. The same
        // logical caret must be resubmitted when its physical position changes.
        let key = area.map(|area| (area, scale));
        if self.last == key { return None; }
        self.last = key;
        area
    }
}

impl Win {
    pub(super) fn sync_ime_cursor(&mut self, area: Option<[f32; 4]>) {
        #[cfg(target_os = "linux")]
        let area = self.native.ime_cursor.changed(area, self.window.scale_factor());
        if let Some(a) = area {
            self.window.set_ime_cursor_area(
                LogicalPosition::new(a[0] as f64, a[1] as f64),
                LogicalSize::new(a[2] as f64, a[3] as f64),
            );
        }
    }

    /// The runtime's cursor while the pointer is in the window, the arrow once it leaves.
    /// `Hidden` needs focus, so it can't outlive the window being active.
    pub(super) fn apply_cursor<S>(&mut self, rt: &Runtime<S>, focused: bool) {
        let want = match rt.cursor() { crate::el::Cursor::Hidden if !focused => crate::el::Cursor::Default, c => c };
        self.cursor.apply(&self.window, want, rt.pointer().is_some());
    }
}

#[cfg(test)]
#[path = "platform_sync_tests.rs"]
mod tests;
