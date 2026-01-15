#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;
use tauri::Manager;

fn main() {
    // Verifica se foi passado um perfil via argumento
    // Uso: pdv-desktop --profile=2 ou pdv-desktop -p 2
    let args: Vec<String> = env::args().collect();
    let mut profile: Option<String> = None;

    for (i, arg) in args.iter().enumerate() {
        if arg.starts_with("--profile=") {
            profile = Some(arg.replace("--profile=", ""));
        } else if (arg == "--profile" || arg == "-p") && i + 1 < args.len() {
            profile = Some(args[i + 1].clone());
        }
    }

    // Gera o contexto uma única vez
    let mut context = tauri::generate_context!();

    // Se um perfil foi especificado, modifica o identificador para criar sessão separada
    if let Some(ref p) = profile {
        let identifier = format!("com.i9smart.pdv-desktop.profile-{}", p);
        context.config_mut().tauri.bundle.identifier = identifier;
    }

    let profile_clone = profile.clone();

    tauri::Builder::default()
        .setup(move |app| {
            // Se um perfil foi especificado, renomeia a janela
            if let Some(ref p) = profile_clone {
                if let Some(window) = app.get_window("main") {
                    let title = format!("PDV Desktop - Perfil {}", p);
                    let _ = window.set_title(&title);
                }
            }
            Ok(())
        })
        .run(context)
        .expect("Erro ao iniciar a aplicação PDV Desktop");
}
