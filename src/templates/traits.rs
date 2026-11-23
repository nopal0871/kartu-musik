use crate::types::{CardData, ExportResolution, Orientation};
use web_sys::CanvasRenderingContext2d;

pub trait CardTemplate: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn supported_orientations(&self) -> Vec<Orientation>;
    fn render_view(&self, data: &CardData) -> leptos::View;
    fn render_canvas(
        &self,
        ctx: &CanvasRenderingContext2d,
        data: &CardData,
        time_sec: f64,
        spectrum_data: &[f32],
        res: ExportResolution,
    );
}
