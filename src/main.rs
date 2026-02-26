// src/main.rs
use holochain_wind_tunnel_runner::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Holochain Wind Tunnel Test");
    
    // init() restituisce WindTunnelScenarioCli
    let cli = init();
    
    println!("Wind Tunnel inizializzato correttamente!");
    println!("CLI creata (non implementa Debug)");
    
    Ok(())
}
