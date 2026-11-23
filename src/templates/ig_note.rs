use super::traits::CardTemplate;
use crate::types::{CardData, ExportResolution, Orientation};
use leptos::*;
use web_sys::CanvasRenderingContext2d;

pub struct IgNoteTemplate;

impl IgNoteTemplate {
    pub fn new() -> Self {
        Self
    }
}

impl CardTemplate for IgNoteTemplate {
    fn id(&self) -> &'static str {
        "ig_note"
    }
    fn name(&self) -> &'static str {
        "Instagram Note Music"
    }
    fn description(&self) -> &'static str {
        "Gaya stiker gelembung musik mini khas fitur Catatan Instagram."
    }
    fn supported_orientations(&self) -> Vec<Orientation> {
        vec![Orientation::Portrait]
    }

    fn render_view(&self, data: &CardData) -> View {
        let title = data.title.clone();
        let artist = data.artist.clone();

        view! {
            <div class="w-80 h-40 bg-gray-900 rounded-2xl p-4 border border-gray-800 flex items-center gap-4">
                <div class="w-16 h-16 rounded-full bg-gray-700 flex-shrink-0"></div>
                <div class="overflow-hidden">
                    <span class="text-xs bg-blue-600 px-2 py-0.5 rounded-full font-semibold">"IG Note"</span>
                    <h3 class="font-bold text-sm text-white truncate mt-1">{title}</h3>
                    <p class="text-xs text-gray-400 truncate">{artist}</p>
                </div>
            </div>
        }.into_view()
    }

    fn render_canvas(
        &self,
        ctx: &CanvasRenderingContext2d,
        data: &CardData,
        _time_sec: f64,
        _spectrum_data: &[f32],
        _res: ExportResolution,
    ) {
        ctx.clear_rect(0.0, 0.0, 1080.0, 1920.0);
        ctx.set_fill_style_str("#111827");
        ctx.fill_rect(0.0, 0.0, 1080.0, 1920.0);
        ctx.set_fill_style_str("#ffffff");
        let _ = ctx.fill_text(&data.title, 100.0, 800.0);
    }
}
