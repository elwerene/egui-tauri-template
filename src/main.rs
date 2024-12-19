mod app;
mod tauri;

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::web_sys::{window, HtmlCanvasElement};
    use wasm_bindgen::JsCast;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let canvas = window()
            .expect("global window does not exists")
            .document()
            .expect("expecting a document on window")
            .get_element_by_id("the_canvas_id") // hardcode it
            .expect("expecting an element with id 'the_canvas_id'")
            .dyn_into::<HtmlCanvasElement>()
            .expect("expecting an element with id 'the_canvas_id' to be a canvas element");

        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
            )
            .await
            .expect("failed to start eframe");
    });
}
