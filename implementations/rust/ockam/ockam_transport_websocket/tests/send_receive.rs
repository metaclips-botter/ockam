use ockam_core::compat::rand::{self, Rng};
use ockam_core::{route, Result, Routed, Worker};
use ockam_node::Context;
use ockam_transport_websocket::{WebSocketTransport, WS};

#[ignore]
#[ockam_macros::test]
async fn send_receive(ctx: &mut Context) -> Result<()> {
    let transport = WebSocketTransport::create(ctx).await?;
    let listener_address = transport.listen("127.0.0.1:0").await?;
    ctx.start_worker("echoer", Echoer).await?;

    // Sender
    {
        let msg: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(256)
            .map(char::from)
            .collect();
        let r = route![(WS, listener_address.to_string()), "echoer"];
        let reply = ctx.send_and_receive::<String>(r, msg.clone()).await?;

        assert_eq!(reply, msg, "Should receive the same message");
    };
    Ok(())
}

pub struct Echoer;

#[ockam_core::worker]
impl Worker for Echoer {
    type Message = String;
    type Context = Context;

    async fn handle_message(&mut self, ctx: &mut Context, msg: Routed<String>) -> Result<()> {
        ctx.send(msg.return_route(), msg.body()).await
    }
}
