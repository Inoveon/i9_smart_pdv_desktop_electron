#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent};

fn create_menu() -> Menu {
    // Menu Arquivo
    let quit = CustomMenuItem::new("quit".to_string(), "Sair").accelerator("CmdOrCtrl+Q");
    let arquivo_menu = Submenu::new("Arquivo", Menu::new().add_item(quit));

    // Menu Editar (padrão do sistema)
    let editar_menu = Submenu::new("Editar", Menu::new()
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::SelectAll)
    );

    // Menu Configurações
    let settings = CustomMenuItem::new("settings".to_string(), "Configurações Rápidas").accelerator("CmdOrCtrl+,");
    let wizard = CustomMenuItem::new("wizard".to_string(), "Assistente de Configuração");
    let reset_config = CustomMenuItem::new("reset_config".to_string(), "Resetar Configurações");
    let config_menu = Submenu::new("Configurações", Menu::new()
        .add_item(settings)
        .add_item(wizard)
        .add_native_item(MenuItem::Separator)
        .add_item(reset_config)
    );

    // Menu Visualizar
    let reload = CustomMenuItem::new("reload".to_string(), "Recarregar").accelerator("CmdOrCtrl+R");
    let fullscreen = CustomMenuItem::new("fullscreen".to_string(), "Tela Cheia").accelerator("F11");
    let dev_tools = CustomMenuItem::new("dev_tools".to_string(), "Ferramentas de Desenvolvimento").accelerator("CmdOrCtrl+Shift+I");
    let visualizar_menu = Submenu::new("Visualizar", Menu::new()
        .add_item(reload)
        .add_item(fullscreen)
        .add_native_item(MenuItem::Separator)
        .add_item(dev_tools)
    );

    // Menu Ajuda
    let about = CustomMenuItem::new("about".to_string(), "Sobre o PDV Desktop");
    let ajuda_menu = Submenu::new("Ajuda", Menu::new().add_item(about));

    Menu::new()
        .add_submenu(arquivo_menu)
        .add_submenu(editar_menu)
        .add_submenu(config_menu)
        .add_submenu(visualizar_menu)
        .add_submenu(ajuda_menu)
}

fn handle_menu_event(event: WindowMenuEvent) {
    let window = event.window();

    match event.menu_item_id() {
        "quit" => {
            std::process::exit(0);
        }
        "settings" => {
            // Navegar para a tela de configurações rápidas
            let _ = window.eval("window.location.href = window.location.origin + '/?settings=true'");
        }
        "wizard" => {
            // Navegar para o wizard de configuração
            let _ = window.eval("window.location.href = window.location.origin + '/?wizard=true'");
        }
        "reset_config" => {
            // Limpar localStorage e recarregar
            let _ = window.eval("localStorage.removeItem('pdv_desktop_config'); window.location.href = window.location.origin + '/'");
        }
        "reload" => {
            let _ = window.eval("window.location.reload()");
        }
        "fullscreen" => {
            if let Ok(is_fullscreen) = window.is_fullscreen() {
                let _ = window.set_fullscreen(!is_fullscreen);
            }
        }
        "dev_tools" => {
            #[cfg(debug_assertions)]
            {
                if window.is_devtools_open() {
                    window.close_devtools();
                } else {
                    window.open_devtools();
                }
            }
        }
        "about" => {
            let _ = window.eval(r#"
                alert('PDV Desktop - I9 Smart\n\nVersão: 1.0.0\n\nSistema de Ponto de Venda para Postos de Combustíveis\n\n© 2024 I9 Smart - Todos os direitos reservados');
            "#);
        }
        _ => {}
    }
}

fn main() {
    tauri::Builder::default()
        .menu(create_menu())
        .on_menu_event(handle_menu_event)
        .run(tauri::generate_context!())
        .expect("Erro ao iniciar a aplicação PDV Desktop");
}
