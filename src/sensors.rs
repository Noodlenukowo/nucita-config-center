use std::fs;

pub fn obtener_nivel_bateria() -> String {
    let rutas = [
        "/sys/class/power_supply/BAT0/capacity",
        "/sys/class/power_supply/BAT1/capacity"
    ];
    for ruta in rutas {
        if let Ok(valor) = fs::read_to_string(ruta) {
            return valor.trim().to_string();
        }
    }
    "0".to_string()
}