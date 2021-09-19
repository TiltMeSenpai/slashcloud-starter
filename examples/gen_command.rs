use cloudflare_bot::PingCommand;

fn main() {
    cloudflare_slash::gen_command_json::<PingCommand>();
}