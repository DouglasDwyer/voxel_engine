use egui_demo_lib::*;
use serde::*;
use voxel_engine::*;
use voxel_engine::asset::*;
use voxel_engine::egui::*;
use wings::*;

include_assets!("example_mod/assets");

instantiate_systems!(Client, [HelloClient]);
instantiate_systems!(Server, [HelloServer]);

/// Prints a hello when instantiated on the client, then displays an example window.
#[export_system]
pub struct HelloClient {
    /// The context handle.
    ctx: WingsContextHandle<Self>,
    /// The widget gallery window.
    widget_gallery: WidgetGallery
}

impl HelloClient {
    /// Draws the `egui` UI.
    fn draw_ui(&mut self, _: &voxel_engine::timing::on::Frame) {
        let egui = self.ctx.get::<dyn Egui>();
        self.widget_gallery.show(&egui.context(), &mut true);
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
            widget_gallery: WidgetGallery::default()
        }
    }
}

/// Prints a hello when instantiated on the server.
/// In addition, reads some embedded asset data and prints it out.
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

/// An example config that can be read from embedded asset files.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct MyConfig {
    /// A string option
    pub hello: String,
    /// A boolean option
    pub is_true: bool
}