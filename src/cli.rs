use std::io::{stdin, stdout, Write};
use std::process;

#[derive(Debug, PartialEq)]
enum Command<'a> {
    Create([&'a str; 2]),
    Delete([&'a str; 2]),
    List(&'a str),
    Show([&'a str; 2]),
    Connect([&'a str; 3]),
    Disconnect([&'a str; 2]),
    Exit,
    Help,
    Undefined,
}

pub fn run() {
    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        match process_input(&buf) {
            Command::Create(args) => (),
            Command::Delete(args) => (),
            Command::List(args) => (),
            Command::Show(args) => (),
            Command::Connect(args) => (),
            Command::Disconnect(args) => (),
            Command::Exit => process::exit(0),
            Command::Help => print_help(),
            Command::Undefined => println!("Invalid command; type 'help' to see valid commands"),
        }
    }
}

fn process_input<'a>(buf: &'a str) -> Command<'a> {
    let buf = buf.trim();
    let split: Vec<&str> = buf.split(' ').collect();
    if split.len() == 0 {
        return Command::Undefined
    }
    let cmd: &str = &split[0].to_lowercase();
    match cmd {
        "create" => {
            if split.len() != 3 {
                Command::Undefined
            } else {
                Command::Create([split[1], split[2]])
            }
        },
        "delete" => {
            if split.len() != 3 {
                Command::Undefined
            } else {
                Command::Delete([split[1], split[2]])
            }
        },
        "list" => {
            if split.len() != 2 {
                Command::Undefined
            } else {
                Command::List(split[1])
            }
        },
        "show" => {
            if split.len() != 3 {
                Command::Undefined
            } else {
                Command::Show([split[1], split[2]])
            }
        },
        "connect" => {
            if split.len() != 4 {
                Command::Undefined
            } else {
                Command::Connect([split[1], split[2], split[3]])
            }
        },
        "disconnect" => {
            if split.len() != 3 {
                Command::Undefined
            } else {
                Command::Disconnect([split[1], split[2]])
            }
        }
        "exit" => Command::Exit,
        "help" => Command::Help,
        _ => Command::Undefined,

    }
}

fn print_help() {
    println!("create <{{node/component}}> <name> -- Create a node/component");
    println!("delete <{{node/component}}> <name> -- Delete a node/component");
    println!("list <{{node/component}}> -- List all nodes/components");
    println!("show <{{node/component}}> <name> -- Show node/component details");
    println!("connect <node_name> <component_name> <terminal_index> -- Connect a component terminal to a node");
    println!("disconnect <node_name> <component_name> -- Disconnect a connected component terminal and node");
    println!("exit -- Exit the program");
    println!("help -- Show this help text");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_process_input() {
        assert_eq!(process_input("create Arg1 Arg2"), Command::Create(["Arg1", "Arg2"]));
        assert_eq!(process_input("Create Arg1 Arg2"), Command::Create(["Arg1", "Arg2"]));
        assert_eq!(process_input("create"), Command::Undefined);
        assert_eq!(process_input("create Arg1"), Command::Undefined);
        assert_eq!(process_input("create Arg1 Arg2 Arg3"), Command::Undefined);

        assert_eq!(process_input("delete Arg1 Arg2"), Command::Delete(["Arg1", "Arg2"]));
        assert_eq!(process_input("Delete Arg1 Arg2"), Command::Delete(["Arg1", "Arg2"]));
        assert_eq!(process_input("delete"), Command::Undefined);
        assert_eq!(process_input("delete Arg1"), Command::Undefined);
        assert_eq!(process_input("delete Arg1 Arg2 Arg3"), Command::Undefined);

        assert_eq!(process_input("list Arg1"), Command::List("Arg1"));
        assert_eq!(process_input("List Arg1"), Command::List("Arg1"));
        assert_eq!(process_input("list"), Command::Undefined);
        assert_eq!(process_input("list Arg1 Arg2"), Command::Undefined);

        assert_eq!(process_input("show Arg1 Arg2"), Command::Show(["Arg1", "Arg2"]));
        assert_eq!(process_input("Show Arg1 Arg2"), Command::Show(["Arg1", "Arg2"]));
        assert_eq!(process_input("show"), Command::Undefined);
        assert_eq!(process_input("show Arg1"), Command::Undefined);
        assert_eq!(process_input("show Arg1 Arg2 Arg3"), Command::Undefined);

        assert_eq!(process_input("connect Arg1 Arg2 Arg3"), Command::Connect(["Arg1", "Arg2", "Arg3"]));
        assert_eq!(process_input("Connect Arg1 Arg2 Arg3"), Command::Connect(["Arg1", "Arg2", "Arg3"]));
        assert_eq!(process_input("connect"), Command::Undefined);
        assert_eq!(process_input("connect Arg1"), Command::Undefined);
        assert_eq!(process_input("connect Arg1 Arg2"), Command::Undefined);
        assert_eq!(process_input("connect Arg1 Arg2 Arg3 Arg4"), Command::Undefined);

        assert_eq!(process_input("disconnect Arg1 Arg2"), Command::Disconnect(["Arg1", "Arg2"]));
        assert_eq!(process_input("Disconnect Arg1 Arg2"), Command::Disconnect(["Arg1", "Arg2"]));
        assert_eq!(process_input("disconnect"), Command::Undefined);
        assert_eq!(process_input("disconnect Arg1"), Command::Undefined);
        assert_eq!(process_input("disconnect Arg1 Arg2 Arg3"), Command::Undefined);

        assert_eq!(process_input("exit"), Command::Exit);
        assert_eq!(process_input("Exit"), Command::Exit);
        assert_eq!(process_input("exit and some more stuff"), Command::Exit);

        assert_eq!(process_input("help"), Command::Help);
        assert_eq!(process_input("Help"), Command::Help);
        assert_eq!(process_input("help and some more stuff"), Command::Help);

        assert_eq!(process_input("thisisnotacommand"), Command::Undefined);
        assert_eq!(process_input("This Is Also Not A Command!"), Command::Undefined);
    }
}
