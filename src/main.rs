use dazeus::{Connection, DaZeus, EventType};
use docopt::Docopt;

mod factoid_commands;

static USAGE: &str = "\
A DaZeus plugin for factoids.

Usage:
    dazeus-factoids [options]

Options:
    -h, --help                  Show this help message
    -s SOCKET, --socket=SOCKET  Specify the socket DaZeus is listening on. Use
                                `unix:/path/to/socket` or `tcp:host:port`
                                [default: unix:/tmp/dazeus.sock]
";

fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|d| d.parse())
        .unwrap_or_else(|e| e.exit());
    let socket = args.get_str("--socket");
    let mut dazeus = DaZeus::new(Connection::from_str(socket).unwrap());

    dazeus.subscribe(EventType::PrivMsg, factoid_commands::handle_privmsg);
    dazeus.subscribe_command("blame", factoid_commands::blame);
    dazeus.subscribe_command("learn", factoid_commands::learn);
    dazeus.subscribe_command("reply", factoid_commands::reply);
    dazeus.subscribe_command("forget", factoid_commands::forget);
    dazeus.subscribe_command("forward", factoid_commands::forward);

    // We unwrap the result, which we will retrieve when listening has failed
    dazeus.listen().unwrap();
}
