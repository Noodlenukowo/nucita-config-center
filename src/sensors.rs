use std::process::Command;
use std::fs;

pub fn obtener_nivel_bateria() -> String {
    let rutas = ["/sys/class/power_supply/BAT0/capacity", "/sys/class/power_supply/BAT1/capacity"];
    for ruta in rutas {
        if let Ok(valor) = fs::read_to_string(ruta) {
            return valor.trim().to_string();
        }
    }
    "0".to_string()
}

pub fn obtener_perfil_actual() -> String {
    let output = Command::new("powerprofilesctl")
        .arg("get")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "desconocido".to_string());
    output
}

pub fn cambiar_perfil(perfil: &str) {
    let _ = Command::new("powerprofilesctl")
        .arg("set")
        .arg(perfil)
        .spawn();
}

pub fn obtener_estado_wifi() -> bool {
    let output = Command::new("nmcli")
        .args(["radio", "wifi"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "disabled".to_string());
    output == "enabled"
}

pub fn cambiar_wifi(activar: bool) {
    let estado = if activar { "on" } else { "off" };
    let _ = Command::new("nmcli")
        .args(["radio", "wifi", estado])
        .spawn();
}

pub fn escanear_redes() -> Vec<String> {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "SSID", "dev", "wifi", "list"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();
    
    let mut redes: Vec<String> = output
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();
    redes.dedup();
    redes
}