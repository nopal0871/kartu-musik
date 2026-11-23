pub fn get_mock_spectrum(time_sec: f64) -> Vec<f32> {
    (0..16)
        .map(|i| {
            let wave = ((time_sec * 4.0 + i as f64 * 0.5).sin() + 1.0) / 2.0;
            (wave * 0.8 + 0.2) as f32
        })
        .collect()
}
