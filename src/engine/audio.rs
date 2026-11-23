use web_sys::{AnalyserNode, AudioContext};

#[allow(dead_code)]
pub struct AudioEngine {
    pub ctx: AudioContext,
    pub analyser: AnalyserNode,
}

impl AudioEngine {
    #[allow(dead_code)]
    pub fn new() -> Result<Self, serde_json::Value> {
        let ctx = AudioContext::new().map_err(|e| serde_json::to_value(format!("{:?}", e)).unwrap())?;
        let analyser = ctx.create_analyser().map_err(|e| serde_json::to_value(format!("{:?}", e)).unwrap())?;
        analyser.set_fft_size(64);
        Ok(Self { ctx, analyser })
    }
}
