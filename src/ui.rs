use relm4::prelude::*;
use adw::prelude::*;
use adw::glib;
use crate::app::{AppModel, AppInput, PAGE_ENERGIA, PAGE_RED};

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppInput;
    type Output = ();
    type Widgets = AppWidgets;

    view! {
        adw::Window {
            set_title: Some("Configurador Nucita 🏎️"),
            set_default_width: 750,
            set_default_height: 550,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                
                // --- SIDEBAR (Menú Lateral) ---
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_width_request: 220,
                    // Quitamos la clase CSS sidebar para eliminar los warnings críticos
                    gtk::ListBox {
                        add_css_class: "navigation-sidebar",
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

                // --- CONTENT (Páginas) ---
                adw::ViewStack {
                    set_hexpand: true,
                    // Dejamos el nombre vacío inicialmente para que no busque 'energia' antes de tiempo
                    #[watch]
                    set_visible_child_name: &model.pagina_activa,

                    // PÁGINA ENERGÍA
                    add_named[Some(PAGE_ENERGIA)] = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        adw::StatusPage {
                            #[watch]
                            set_title: &format!("Batería: {}%", model.battery_level),
                            #[watch]
                            set_description: Some(&format!("Perfil: {}", model.perfil_actual)),
                            set_icon_name: Some("battery-full-symbolic"),
                        },
                        adw::PreferencesGroup {
                            set_title: "Rendimiento",
                            set_margin_start: 30,
                            set_margin_end: 30,
                            adw::ActionRow {
                                set_title: "Modo Rendimiento",
                                set_activatable: true,
                                connect_activated[sender] => move |_| {
                                    sender.input(AppInput::SetPerfil("performance".to_string()));
                                },
                            },
                            adw::ActionRow {
                                set_title: "Modo Equilibrado",
                                set_activatable: true,
                                connect_activated[sender] => move |_| {
                                    sender.input(AppInput::SetPerfil("balanced".to_string()));
                                },
                            },
                            adw::ActionRow {
                                set_title: "Modo Ahorro",
                                set_activatable: true,
                                connect_activated[sender] => move |_| {
                                    sender.input(AppInput::SetPerfil("power-saver".to_string()));
                                },
                            },
                        }
                    },

                    // PÁGINA RED
                    add_named[Some(PAGE_RED)] = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        adw::StatusPage {
                            set_title: "Wi-Fi y Red",
                            set_description: Some("Gestiona tu conectividad inalámbrica"),
                            set_icon_name: Some("network-wireless-symbolic"),
                        },
                        adw::PreferencesGroup {
                            set_margin_start: 30,
                            set_margin_end: 30,

                            // INTERRUPTOR INTUITIVO
                            adw::ActionRow {
                                set_title: "Estado del Wi-Fi",
                                set_subtitle: "Prender o apagar la antena",
                                // Añadimos el switch como sufijo (a la derecha)
                                add_suffix = &gtk::Switch {
                                    #[watch]
                                    set_active: model.wifi_activo,
                                    // Cuando el usuario lo mueve, enviamos la señal
                                    connect_state_set[sender] => move |_, state| {
                                        sender.input(AppInput::ToggleWifi(state));
                                        glib::Propagation::Proceed
                                    }
                                }
                            },

                            adw::ActionRow {
                                set_title: "Escanear Redes Cercanas",
                                set_activatable: true,
                                connect_activated[sender] => move |_| {
                                    sender.input(AppInput::EscanearRedes);
                                },
                            }
                        },

                        // LISTA DE REDES
                        gtk::ListBox {
                            set_margin_all: 30,
                            add_css_class: "boxed-list",
                            #[watch]
                            set_visible: !model.redes_disponibles.is_empty(),
                            
                            gtk::ListBoxRow {
                                gtk::Label {
                                    #[watch]
                                    set_label: &format!("Redes encontradas: {}", model.redes_disponibles.len()),
                                    set_margin_all: 10,
                                }
                            }
                        }
                    },
                }
            }
        }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        self.update_logic(message);
    }

    fn init(_: Self::Init, _root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = crate::app::AppModel::init_model(sender.clone());
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}