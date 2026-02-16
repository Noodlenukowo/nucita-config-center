use relm4::prelude::*;
use std::time::Duration;
use crate::sensors;

pub const PAGE_ENERGIA: &str = "energia";
pub const PAGE_RED: &str = "red";

pub struct AppModel {
    pub pagina_activa: String,
    pub battery_level: String,
    pub perfil_actual: String,
    pub wifi_activo: bool,
    pub redes_disponibles: Vec<String>,
}

#[derive(Debug)]
pub enum AppInput {
    CambiarPagina(String),
    ActualizarBateria,
    SetPerfil(String),
    ToggleWifi(bool),
    EscanearRedes,
}

// AQUÍ ES DONDE ESTABA EL FALLO:
impl AppModel {
    pub fn update_logic(&mut self, message: AppInput) {
        match message {
            AppInput::CambiarPagina(nombre) => self.pagina_activa = nombre,
            AppInput::ActualizarBateria => {
                self.battery_level = sensors::obtener_nivel_bateria();
                self.perfil_actual = sensors::obtener_perfil_actual();
                self.wifi_activo = sensors::obtener_estado_wifi();
            }
            AppInput::SetPerfil(perfil) => {
                sensors::cambiar_perfil(&perfil);
                self.perfil_actual = perfil;
            }
            AppInput::ToggleWifi(valor) => {
                sensors::cambiar_wifi(valor);
                self.wifi_activo = valor;
            }
            AppInput::EscanearRedes => {
                self.redes_disponibles = sensors::escanear_redes();
            }
        }
    }

    // Esta es la función que el compilador no encontraba:
    pub fn init_model(sender: ComponentSender<Self>) -> Self {
        let sender_clone = sender.clone();
        std::thread::spawn(move || {
            loop {
                sender_clone.input(AppInput::ActualizarBateria);
                std::thread::sleep(Duration::from_secs(5));
            }
        });

        Self {
            pagina_activa: PAGE_ENERGIA.to_string(),
            battery_level: "0".to_string(),
            perfil_actual: sensors::obtener_perfil_actual(),
            wifi_activo: sensors::obtener_estado_wifi(),
            redes_disponibles: Vec::new(),
        }
    }
}