#![warn(clippy::all, rust_2018_idioms)]
#[cfg(target_arch = "wasm32")]
use omnis_vanitas::TemplateApp;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "omni-vanitas",
        native_options,
        Box::new(|cc| Box::new(TemplateApp::new(cc))),
    );
}


#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();


    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "omni_vanitas_canvas", // hardcode it
            web_options,
            Box::new(|cc| Box::new(TemplateApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}


