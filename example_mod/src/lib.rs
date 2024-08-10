use egui_demo_lib::*;
use serde::*;
use voxel_engine::*;
use voxel_engine::asset::*;
use voxel_engine::egui::*;
use wings::*;

include_assets!("example_mod/assets");

instantiate_systems!(Client, [HelloClient]);
instantiate_systems!(Server, [HelloServer]);

/// Prints a hello when instantiated on the client.
#[export_system]
pub struct HelloClient {
    /// The context handle.
    ctx: WingsContextHandle<Self>,
    /// Sample text
    text_box: String,
    /// The widget gallery window.
    widget_gallery: WidgetGallery
}

impl HelloClient {
    fn draw_ui(&mut self, _: &voxel_engine::timing::on::Frame) {
        let egui = self.ctx.get::<dyn Egui>();
        let context = egui.context();
        Window::new("Hello from WASM!")
        .show(&context, |ui| {
            ui.label("Welcome to here.");
            ui.text_edit_singleline(&mut self.text_box);
            Image::new(self.ctx.get::<dyn AssetManager>().get_ui_texture(assets::MEME))
                .max_size(egui::vec2(200.0, 200.0))
                .ui(ui);
        });
        self.widget_gallery.show(&context, &mut true);
    }
}

impl WingsSystem for HelloClient {
    const DEPENDENCIES: Dependencies = dependencies()
        .with::<dyn AssetManager>()
        .with::<dyn Egui>();

    const EVENT_HANDLERS: EventHandlers<Self> = event_handlers()
        .with(Self::draw_ui);

    fn new(ctx: WingsContextHandle<Self>) -> Self {
        println!("Hello client!");
        Self {
            ctx,
            text_box: String::default(),
            widget_gallery: WidgetGallery::default()
        }
    }
}

/// Prints a hello when instantiated on the server.
#[export_system]
pub struct HelloServer;

impl WingsSystem for HelloServer {
    const DEPENDENCIES: Dependencies = dependencies()
        .with::<dyn AssetManager>();

    fn new(ctx: WingsContextHandle<Self>) -> Self {
        println!("Hello server! {:?}", ctx.get::<dyn AssetManager>().get_from_toml::<MyConfig>(assets::MY_CONFIG));
        Self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MyConfig {
    pub hello: String,
    pub is_true: bool
}