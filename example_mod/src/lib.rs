use voxel_engine::*;
use wings::*;

instantiate_systems!(Client, [HelloClient]);
instantiate_systems!(Server, [HelloServer]);

/// Prints a hello when instantiated on the client.
#[export_system]
pub struct HelloClient;

impl HelloClient {
    fn draw_ui(&mut self, _: &voxel_engine::timing::on::Frame) {
        panic!("There's a frame a happenin!");
    }
}

impl WingsSystem for HelloClient {
    const EVENT_HANDLERS: EventHandlers<Self> = event_handlers()
        .with(Self::draw_ui);

    fn new(_: WingsContextHandle<Self>) -> Self {
        println!("Hello client!");
        Self
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