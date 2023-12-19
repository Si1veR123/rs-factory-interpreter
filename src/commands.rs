pub enum Command {
    Boot,
    Left,
    Right,
    Claw,
    Ram,
    SendShipment,
    RequestShipment
}

pub fn commands_from_str(string: &str) -> Vec<Command> {
    let mut commands = Vec::new();
    let mut chars = string.chars();
    
    loop {
        let current_opt = chars.next();

        let command = match current_opt {
            Some(current) => match current {
                'B' => {
                    chars.nth(2);
                    Command::Boot
                },
                '<' => Command::Left,
                '>' => Command::Right,
                'v' => Command::Claw,
                '^' => Command::Ram,
                'O' => Command::SendShipment,
                'I' => Command::RequestShipment,
                '/' => {
                    let mut current = chars.next();
                    while current != Some('\n') && current != None  {
                        current = chars.next()
                    }
                    continue
                }
                _ => continue
            },
            None => break
        };

        commands.push(command);
    }

    commands
}
