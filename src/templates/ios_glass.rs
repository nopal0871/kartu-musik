use super::traits::CardTemplate;
use crate::types::{CardData, ExportResolution, Orientation};
use leptos::*;
use web_sys::CanvasRenderingContext2d;

pub struct IosGlassTemplate;

impl IosGlassTemplate {
    pub fn new() -> Self {
        Self
    }
}

impl CardTemplate for IosGlassTemplate {
    fn id(&self) -> &'static str {
        "ios_glass"
    }
    fn name(&self) -> &'static str {
        "iOS Glassmorphism"
    }
    fn description(&self) -> &'static str {
        "Pemutar lagu ala iOS dengan efek kaca transparan dan latar blur adaptif."
    }
    fn supported_orientations(&self) -> Vec<Orientation> {
        vec![Orientation::Portrait, Orientation::Landscape]
    }

    fn render_view(&self, data: &CardData) -> View {
        let title = data.title.clone();
        let artist = data.artist.clone();
        let cover = data.cover_url.clone();

        view! {
            <div class="relative w-80 h-[480px] rounded-3xl overflow-hidden shadow-2xl p-6 flex flex-col justify-between border border-white/20 bg-white/10 backdrop-blur-xl">
                <div class="w-full aspect-square rounded-2xl overflow-hidden bg-gray-800 shadow-md">
                    <img src=cover class="w-full h-full object-cover" alt="Cover"/>
                </div>
                <div class="my-4 text-center">
                    <h2 class="text-xl font-bold truncate text-white">{title}</h2>
                    <p class="text-sm text-gray-300 truncate">{artist}</p>
                </div>
                <div class="w-full h-1.5 bg-white/20 rounded-full overflow-hidden">
                    <div class="w-1/3 h-full bg-white rounded-full"></div>
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
        ctx.set_fill_style_str("rgba(20, 20, 30, 1.0)");
        ctx.fill_rect(0.0, 0.0, 1080.0, 1920.0);

        ctx.set_fill_style_str("white");
        ctx.set_font("bold 48px sans-serif");
        let _ = ctx.fill_text(&data.title, 100.0, 1200.0);
        ctx.set_font("32px sans-serif");
        let _ = ctx.fill_text(&data.artist, 100.0, 1270.0);
    }
}
