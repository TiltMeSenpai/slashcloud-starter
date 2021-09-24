use cloudflare_bot::PingCommand;

fn main() {
    slashcloud::gen_command_json::<PingCommand>();
}