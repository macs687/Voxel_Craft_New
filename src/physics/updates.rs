use std::time::Instant;


pub fn update_time(last_frame: Instant) -> (f32, Instant) {
    let now = Instant::now();
    let delta_time = (now - last_frame).as_secs_f32();
    let delta_time = delta_time.min(0.05);
    (delta_time, now)
}