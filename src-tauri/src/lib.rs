// Punto di ingresso condiviso tra desktop e mobile.
// L'attributo mobile_entry_point genera l'entrypoint nativo per iOS e Android.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("errore durante l'avvio dell'applicazione Rotellario");
}
