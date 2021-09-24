use worker::*;
use slashcloud::*;

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
                InteractionResponse::message()
                    .set_content(format!("Pong!: {}", echo))
            }
        }
    }
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    handle_request::<PingCommand>(req, env).await
}