use relm4::prelude::*;
use adw::prelude::*;
use std::time::Duration;
use crate::app::{AppModel, AppInput, PAGE_ENERGIA, PAGE_RED};
use crate::sensors;

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppInput;
    type Output = ();
    type Widgets = AppWidgets;

    view! {
        adw::Window {
            set_title: Some("Configurador Nucita 🏎️"),
            set_default_width: 700,
            set_default_height: 500,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                // MENÚ LATERAL
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_width_request: 220,
                    add_css_class: "sidebar",
                    gtk::ListBox {
                        add_css_class: "navigation-sidebar",
                        set_selection_mode: gtk::SelectionMode::None,
                        adw::ActionRow {
                            set_title: "Energía 🔋",
                            set_activatable: true,
                            connect_activated[sender] => move |_| {
                                sender.input(AppInput::CambiarPagina(PAGE_ENERGIA.to_string()));
                            },
                        },
                        adw::ActionRow {
                            set_title: "Red / Wi-Fi 🌐",
                            set_activatable: true,
                            connect_activated[sender] => move |_| {
                                sender.input(AppInput::CambiarPagina(PAGE_RED.to_string()));
                            },
                        },
                    }
                },
                gtk::Separator { set_orientation: gtk::Orientation::Vertical },
                // CONTENIDO
                adw::ViewStack {
                    set_hexpand: true,
                    #[watch]
                    set_visible_child_name: &model.pagina_activa,
                    add_named[Some(PAGE_ENERGIA)] = &adw::StatusPage {
                        #[watch]
                        set_title: &format!("Batería: {}%", model.battery_level),
                        set_description: Some("Sensor en tiempo real activo"),
                        set_icon_name: Some("battery-full-symbolic"),
                    },
                    add_named[Some(PAGE_RED)] = &adw::StatusPage {
                        set_title: "Configuración de Red",
                        set_description: Some("Gestión de conexiones en Manjaro Sway"),
                        set_icon_name: Some("network-wireless-symbolic"),
                    }
                }
            }
        }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppInput::CambiarPagina(nombre) => self.pagina_activa = nombre,
            AppInput::ActualizarBateria => self.battery_level = sensors::obtener_nivel_bateria(),
        }
    }

    fn init(_: Self::Init, _root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = AppModel {
            pagina_activa: PAGE_ENERGIA.to_string(),
            battery_level: "0".to_string(),
        };

        let sender_clone = sender.clone();
        std::thread::spawn(move || {
            loop {
                sender_clone.input(AppInput::ActualizarBateria);
                std::thread::sleep(Duration::from_secs(5));
            }
        });

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}