pub mod anim;
pub mod mention;
pub mod note;
pub mod preview;
pub mod profile;
pub mod relay;
pub mod username;

use egui::Margin;
pub use mention::Mention;
pub use note::Note;
pub use preview::{Preview, PreviewApp};
pub use profile::{ProfilePic, ProfilePreview};
pub use relay::RelayView;
pub use username::Username;

/// This is kind of like the Widget trait but is meant for larger top-level
/// views that are typically stateful. The Widget trait forces us to add mutable
/// implementations at the type level, which screws us when generating Previews
/// for a Widget. I would have just Widget instead of making this Trait
/// otherwise.
///
/// There is some precedent for this, it looks like there's a similar trait
/// in the egui demo library.
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub fn padding<R>(
    amount: impl Into<Margin>,
    ui: &mut egui::Ui,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    egui::Frame::none()
        .inner_margin(amount)
        .show(ui, add_contents)
}

#[inline]
pub fn is_mobile(_ctx: &egui::Context) -> bool {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        true
    }
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        false
    }
}
