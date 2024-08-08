use egui_demo_lib::*;
use egui_wings::*;
use voxel_engine::*;
use wings::*;

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
        });
        self.widget_gallery.show(&context, &mut true);
    }
}

impl WingsSystem for HelloClient {
    const DEPENDENCIES: Dependencies = dependencies()
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
    fn new(_: WingsContextHandle<Self>) -> Self {
        println!("Hello server!");
        Self
    }
}