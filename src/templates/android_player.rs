use super::traits::CardTemplate;
use crate::types::{CardData, ExportResolution, Orientation};
use leptos::*;
use web_sys::CanvasRenderingContext2d;

pub struct AndroidTemplate;

impl AndroidTemplate {
    pub fn new() -> Self {
        Self
    }
}

impl CardTemplate for AndroidTemplate {
    fn id(&self) -> &'static str {
        "android_player"
    }
    fn name(&self) -> &'static str {
        "Android Material You"
    }
    fn description(&self) -> &'static str {
        "Desain pemutar Android modern dengan elemen bergelombang khas OS Android."
    }
    fn supported_orientations(&self) -> Vec<Orientation> {
        vec![Orientation::Portrait]
    }

    fn render_view(&self, data: &CardData) -> View {
        let title = data.title.clone();
        let artist = data.artist.clone();

        view! {
            <div class="w-80 h-[480px] bg-slate-900 rounded-3xl p-6 flex flex-col justify-between border border-slate-800 shadow-xl">
                <div class="w-full aspect-square rounded-xl bg-slate-800 flex items-center justify-center">
                    <span class="text-slate-500 text-sm">"Cover Art"</span>
                </div>
                <div>
                    <h2 class="text-lg font-bold text-slate-100">{title}</h2>
                    <p class="text-xs text-slate-400">{artist}</p>
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
        ctx.set_fill_style_str("#0f172a");
        ctx.fill_rect(0.0, 0.0, 1080.0, 1920.0);
        ctx.set_fill_style_str("#f8fafc");
        let _ = ctx.fill_text(&data.title, 80.0, 1000.0);
    }
}
