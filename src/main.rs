#[macroquad::main("Simulation")]
async fn main() {
    // TODO: use clap for CLI arguments
    // TODO: use log::{info, debug, warn};

    // TODO:   // Disque de Shakura-Sunyaev
    //   struct AccretionDisk {
    //       inner_radius: f64, // ~3 rs (ISCO)
    //       outer_radius: f64,
    //       temperature_profile: fn(f64) -> f64, // T ∝ r^(-3/4)
    //   }

    //   // Doppler shift : λ_observed = λ_emitted * (1 + z)
    //   // z dépend de la vitesse orbitale et du redshift gravitationnel
    //   fn doppler_shift(velocity: f64, gravitational_redshift: f64) -> f64 {
    //       // v_orbital ≈ sqrt(GM/r) pour Keplerian
    //   }

    black_hole_sim::launch().await;
}
