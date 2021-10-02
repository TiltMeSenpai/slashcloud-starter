use worker::*;
use slashcloud::*;

#[derive(CommandOption)]
pub enum PingCommand{
    #[description = "Ping command"]
    Ping {
        #[description = "Echo value"]
        echo: String
    },
    #[description = "Fetch current server info"]
    ServerInfo
}

struct PingHandler {
    env: Env,
    command: PingCommand,
    req: InteractionRequest
}

#[async_trait(?Send)]
impl CommandHandler<PingCommand> for PingHandler {
    fn new_command(env: Env, command: PingCommand, req: InteractionRequest) -> Self {
        PingHandler{env, command, req}
    }

    async fn handle_command(&self) -> InteractionResponse {
        let env = &self.env;
        let limiter = env.durable_object("DISCORD_RATELIMITER").unwrap();
        match &self.command {
            PingCommand::Ping{echo} => {
                InteractionResponse::message()
                    .set_content(format!("Pong!: {}", echo))
            }
            PingCommand::ServerInfo => {
                if let Some(server) = self.req.guild_id {
                    if let DiscordResponse::Ok(guild, limits) = discord::Guild::get(limiter, server, true).await {
                    InteractionResponse::message()
                        .set_content(format!(r#"
                            Guild Name: {}
                            Limit: {},
                            Remaining: {},
                            Reset: {:?}
                        "#, guild.name,
                        limits.limit,
                        limits.remaining,
                        limits.reset
                    ))
                    } else {
                        InteractionResponse::message()
                            .set_content("ERROR".to_string())
                    }
                } else {
                    InteractionResponse::message()
                        .set_content("ERROR".to_string())
                }
            }
        }
    }
}


#[event(fetch)]
pub async fn handle(req: Request, env: Env) -> Result<Response> {
    handle_request::<PingCommand, PingHandler, PingHandler>(req, env).await
}

#[cfg(not(target_arch = "wasm32"))]
pub fn gen_json() {
    slashcloud::gen_command_json::<PingCommand>();
    slashcloud::gen_bulk_command_json::<PingCommand>();
}
