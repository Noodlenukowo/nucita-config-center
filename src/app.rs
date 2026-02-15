pub const PAGE_ENERGIA: &str = "energia";
pub const PAGE_RED: &str = "red";

pub struct AppModel {
    pub pagina_activa: String,
    pub battery_level: String,
}

#[derive(Debug)]
pub enum AppInput {
    CambiarPagina(String),
    ActualizarBateria,
}