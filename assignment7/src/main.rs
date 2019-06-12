extern crate wasmchat;
extern crate web_logger;
extern crate yew;
use wasmchat::{services::PubnubService, Model};
use yew::prelude::*;
pub struct Context {
pubnub: PubnubService,
}
impl AsMut<PubnubService> for Context {
fn as_mut(&mut self) -> &mut PubnubService {
&mut self.pubnub
}
}
fn main() {
web_logger::init();
yew::initialize();
let context = Context {
pubnub: PubnubService::new("pub-c-bc26e47f-de65-417d-89f8-9f3078b8b7d6","sub-c-22c8b29a-8d45-11e9-90d9-8a9dabba299e"),
};
let app: App<_, Model> = App::new(context);
app.mount_to_body();
yew::run_loop();
}