use dazeus::{DaZeusClient, Event};

/// Factoids are prefixed by this
static FACTOID_PREFIX: &str = "]";

/// Split a command based on a separator
fn parse_command_with_separator(command: &[String], separator: &str) -> (String, String) {
    let mut query = command.iter().map(|s| s.as_ref());

    let factoid = query.by_ref()
        .take_while(|&x| x != separator)
        .collect::<Vec<_>>()
        .join(" ");
    let value = query.collect::<Vec<_>>().join(" ");

    (factoid, value)
}

/// Process a blame event
pub fn blame(event: Event, dazeus: &dyn DaZeusClient) {
    let factoid = &event.params[4..].join(" ");
    unimplemented!("Do something with {}", factoid);
}

/// Learn a factoid
pub fn learn(event: Event, dazeus: &dyn DaZeusClient) {
    let query = &event.params[4..]; // the arguments to the command.
    let (factoid, value) = parse_command_with_separator(query, "is");
    if factoid.is_empty() || value.is_empty() {
        dazeus.reply(&event, "Usage: }learn <factoid> is <value>", true);
        return;
    }
    unimplemented!();
}

/// Reply to a factoid
pub fn reply(event: Event, dazeus: &dyn DaZeusClient) {
    let (factoid, value) = parse_command_with_separator(&event.params[4..], "with");
    if factoid.is_empty() || value.is_empty() {
        dazeus.reply(&event, "Usage: }reply <factoid> with <value>", true);
        return;
    }
    unimplemented!()
}

/// Forward a factoid
pub fn forward(event: Event, dazeus: &dyn DaZeusClient) {
    let factoid = &event.params[4..].join(" ");
    unimplemented!()
}

/// Forget a factoid
pub fn forget(event: Event, dazeus: &dyn DaZeusClient) {
    let factoid = &event.params[4..].join(" ");
    unimplemented!()
}

/// Parse and handle privmsgs to check for DaZeus commands
pub fn handle_privmsg(event: Event, dazeus: &dyn DaZeusClient) {
    let message = &event.params[3];
    if !message.starts_with(FACTOID_PREFIX) {
        return;
    }
    // take off prefix
    let factoid = &message[1..];

    // replace <who> and <channel>

    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_command_separator() {
        assert_eq!(
            parse_command_with_separator(
                &vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
                "bar",
            ),
            ("foo".to_string(), "baz".to_string())
        );
        assert_eq!(
            parse_command_with_separator(
                &vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
                "nietes",
            ),
            ("foo bar baz".to_string(), "".to_string())
        );
        assert_eq!(
            parse_command_with_separator(
                &vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
                "foo",
            ),
            ("".to_string(), "bar baz".to_string())
        );
    }
}