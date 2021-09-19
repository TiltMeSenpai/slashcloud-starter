use worker::*;
use cloudflare_slash::*;

#[derive(CommandOption)]
pub enum PingCommand{
    #[description = "Ping command"]
    Ping {
        #[description = "Echo value"]
        echo: String
    }
}

impl CommandHandler for PingCommand {
    fn handle(&self, _req: InteractionRequest) -> InteractionResponse{
        match self {
            PingCommand::Ping{echo} => {
                InteractionResponse::ChannelMessage {
                    deferred: false,
                    body: InteractionResponseBody {
                        tts: None,
                        content: Some(format!("Pong!: {}", echo)),
                        embeds: None,
                        flags: None,
                        components: None,
                    }
                }
            }
        }
    }
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    handle_request::<PingCommand>(req, env).await
}
