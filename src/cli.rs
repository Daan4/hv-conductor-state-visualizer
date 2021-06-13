use std::io::{stdin, stdout, Write};
use std::process;

use super::network::Network;
use super::component::*;

/// A CLI command
#[derive(Debug, PartialEq)]
enum Command<'a> {
    /// Create component / node command
    Create([&'a str; 2]),
    /// Delete component / node command
    Delete(&'a str),
    /// List all components + nodes command
    List(),
    /// Show component / node command
    Show(&'a str),
    /// Connect component and node command
    Connect([&'a str; 3]),
    /// Disconnected component and node command
    Disconnect([&'a str; 2]),
    /// Open switchgear command
    Open(&'a str),
    /// Close switchgear command
    Close(&'a str),
    /// Exit program command
    Exit,
    /// Display help command
    Help,
    /// Unknown command
    Undefined,
}

/// Run the CLI loop; takes input from the user and executes the corresponding command
pub fn run() {
    let net = &Network::new("default_network");
    loop {
        let mut buf = String::new();
        print!(">");
        stdout().flush().unwrap();
        stdin().read_line(&mut buf).unwrap();
        let cmd = process_input(&buf);
        match execute_command(net, cmd) {
            Err(e) => {
                println!("<{}", e);
            },
            Ok(_) => {},
        }
    }
}

fn execute_command(net: &Network, cmd: Command) -> Result<(), String> {
    match cmd {
        Command::Create(args) => create(&net, args[0], args[1]),
        Command::Delete(arg) => delete(&net, arg),
        Command::List() => list(&net),
        Command::Show(arg) => show(&net, arg),
        Command::Connect(args) => connect(&net, args[0], args[1], args[2]),
        Command::Disconnect(args) => disconnect(&net, args[0], args[1]),
        Command::Open(arg) => open(&net, arg),
        Command::Close(arg) => close(&net, arg),
        Command::Exit => process::exit(0),
        Command::Help => print_help(),
        Command::Undefined => Err("Invalid command; type 'help' to see valid commands".to_string()),
    }
}

fn create(net: &Network, _type: &str, name: &str) -> Result<(), String> {
    match _type {
        "cb" => {
            println!("<Created Circuit Breaker {}", name);
            net.create_component::<CircuitBreaker>(&name)
        },
        "ds" => {
            println!("<Created Disconnector {}", name);
            net.create_component::<Disconnector>(&name)
        },
        "es" => {
            println!("<Created Disconnector {}", name);
            net.create_component::<EarthingSwitch>(&name)
        },
        "vt" => {
            println!("<Created Voltage Transformer {}", name);
            net.create_component::<VoltageTransformer>(&name)
        },
        "tf" => {
            println!("<Created Transformer {}", name);
            net.create_component::<Transformer>(&name)
        },
        "node" => {
            println!("<Created Node {}", name);
            net.create_node(&name)
        },
        _ => Err(format!("{} type does not exist (cb, ds, es, vt, tf, node)", _type)),
    }
}

fn delete(net: &Network, name: &str) -> Result<(), String> {
    match (net.remove_node(name), net.remove_component(name)) {
        (Ok(_), _) => {
            println!("<Deleted Node {}", name);
            Ok(())
        },
        (_, Ok(_)) => {
            println!("<Deleted Component {}", name);
            Ok(())
        },
        (_, _) => Err(format!("No node or component with name {} exists", name)),
    }
}

fn list(net: &Network) -> Result<(), String> {
    println!("<--Nodes--");
    net.list_nodes();
    println!("<--Components--");
    net.list_components();
    Ok(())
}

fn show(net: &Network, name: &str) -> Result<(), String> {
    match (net.get_node(name), net.get_component(name)) {
        (Ok(n), _) => {
            println!("<{}", n);
        },
        (_, Ok(c)) => {
            println!("<{}", c);
        },
        (_, _) => return Err(format!("No node or component with name {} exists", name)),
    }
    Ok(())
}

fn connect(net: &Network, node_name: &str, component_name: &str, terminal: &str) -> Result<(), String> {
    let terminal_usize: usize;
    match terminal.parse::<usize>() {
        Ok(t) => {
            terminal_usize = t;
        }, 
        Err(_) => return Err("Terminal (3rd argument) is not an unsigned integer".to_string()),
    }
    net.connect(node_name, component_name, terminal_usize)
}

fn disconnect(net: &Network, node_name: &str, component_name: &str) -> Result<(), String>  {
    net.disconnect(node_name, component_name)
}

fn open(net: &Network, switchgear_name: &str) -> Result<(), String> {
    let c = net.get_component(switchgear_name)?;
    c.open()
}

fn close(net: &Network, switchgear_name: &str) -> Result<(), String> {
    let c = net.get_component(switchgear_name)?;
    c.close()
}

fn process_input(buf: &str) -> Command {
    let buf = buf.trim();
    let split: Vec<&str> = buf.split(' ').collect();
    if split.len() == 0 {
        return Command::Undefined
    }
    let cmd: &str = &split[0].to_lowercase();
    match cmd {
        "create" => {
            if split.len() < 3 {
                Command::Undefined
            } else {
                Command::Create([split[1], split[2]])
            }
        },
        "delete" => {
            if split.len() < 2 {
                Command::Undefined
            } else {
                Command::Delete(split[1])
            }
        },
        "list" => {
            if split.len() < 1 {
                Command::Undefined
            } else {
                Command::List()
            }
        },
        "show" => {
            if split.len() < 2 {
                Command::Undefined
            } else {
                Command::Show(split[1])
            }
        },
        "connect" => {
            if split.len() < 4 {
                Command::Undefined
            } else {
                Command::Connect([split[1], split[2], split[3]])
            }
        },
        "disconnect" => {
            if split.len() < 3 {
                Command::Undefined
            } else {
                Command::Disconnect([split[1], split[2]])
            }
        },
        "open" => {
            if split.len() < 2 {
                Command::Undefined
            } else  {
                Command::Open(split[1])
            }
        },
        "close" => {
            if split.len() < 2 {
                Command::Undefined
            } else  {
                Command::Close(split[1])
            }
        }
        "exit" => Command::Exit,
        "help" => Command::Help,
        _ => Command::Undefined,
    }
}

fn print_help() -> Result<(), String> {
    println!("<create <{{node/component type}}> <name> -- Create a node/component");
    println!("<delete <name> -- Delete a node/component");
    println!("<list -- List all nodes/components");
    println!("<show <name> -- Show node/component details");
    println!("<connect <node_name> <component_name> <terminal_index> -- Connect a component terminal to a node");
    println!("<disconnect <node_name> <component_name> -- Disconnect a connected component terminal and node");
    println!("<exit -- Exit the program");
    println!("<help -- Show this help text");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_process_input() {
        assert_eq!(process_input("create Arg1 Arg2"), Command::Create(["Arg1", "Arg2"]));
        assert_eq!(process_input("Create Arg1 Arg2"), Command::Create(["Arg1", "Arg2"]));
        assert_eq!(process_input("create Arg1 Arg2 junk data here"), Command::Create(["Arg1", "Arg2"]));
        assert_eq!(process_input("create"), Command::Undefined);
        assert_eq!(process_input("create Arg1"), Command::Undefined);

        assert_eq!(process_input("delete Arg1"), Command::Delete("Arg1"));
        assert_eq!(process_input("Delete Arg1"), Command::Delete("Arg1"));
        assert_eq!(process_input("delete Arg1 junk data here"), Command::Delete("Arg1"));
        assert_eq!(process_input("delete"), Command::Undefined);

        assert_eq!(process_input("list"), Command::List());
        assert_eq!(process_input("List"), Command::List());
        assert_eq!(process_input("list junk data here"), Command::List());

        assert_eq!(process_input("show Arg1"), Command::Show("Arg1"));
        assert_eq!(process_input("Show Arg1"), Command::Show("Arg1"));
        assert_eq!(process_input("show Arg1 junk data here"), Command::Show("Arg1"));
        assert_eq!(process_input("show"), Command::Undefined);

        assert_eq!(process_input("connect Arg1 Arg2 Arg3"), Command::Connect(["Arg1", "Arg2", "Arg3"]));
        assert_eq!(process_input("Connect Arg1 Arg2 Arg3"), Command::Connect(["Arg1", "Arg2", "Arg3"]));
        assert_eq!(process_input("connect Arg1 Arg2 Arg3 junk data here"), Command::Connect(["Arg1", "Arg2", "Arg3"]));
        assert_eq!(process_input("connect"), Command::Undefined);
        assert_eq!(process_input("connect Arg1"), Command::Undefined);
        assert_eq!(process_input("connect Arg1 Arg2"), Command::Undefined);

        assert_eq!(process_input("disconnect Arg1 Arg2"), Command::Disconnect(["Arg1", "Arg2"]));
        assert_eq!(process_input("Disconnect Arg1 Arg2"), Command::Disconnect(["Arg1", "Arg2"]));
        assert_eq!(process_input("disconnect Arg1 Arg2 junk data here"), Command::Disconnect(["Arg1", "Arg2"]));
        assert_eq!(process_input("disconnect"), Command::Undefined);
        assert_eq!(process_input("disconnect Arg1"), Command::Undefined);

        assert_eq!(process_input("open Arg1"), Command::Open("Arg1"));
        assert_eq!(process_input("Open Arg1"), Command::Open("Arg1"));
        assert_eq!(process_input("open Arg1 junk data here"), Command::Open("Arg1"));
        assert_eq!(process_input("open"), Command::Undefined);

        assert_eq!(process_input("close Arg1"), Command::Close("Arg1"));
        assert_eq!(process_input("Close Arg1"), Command::Close("Arg1"));
        assert_eq!(process_input("close Arg1 junk data here"), Command::Close("Arg1"));
        assert_eq!(process_input("close"), Command::Undefined);

        assert_eq!(process_input("exit"), Command::Exit);
        assert_eq!(process_input("Exit"), Command::Exit);
        assert_eq!(process_input("exit and some more stuff"), Command::Exit);

        assert_eq!(process_input("help"), Command::Help);
        assert_eq!(process_input("Help"), Command::Help);
        assert_eq!(process_input("help and some more stuff"), Command::Help);

        assert_eq!(process_input(""), Command::Undefined);
        assert_eq!(process_input("thisisnotacommand"), Command::Undefined);
        assert_eq!(process_input("This Is Also Not A Command!"), Command::Undefined);
    }

    #[test]
    fn cli_commands() {
        let net = &Network::new("net");
        
        // Create
        assert!(execute_command(net, Command::Create(["cb", "1"])).is_ok());
        assert_eq!(net.get_component("1").unwrap().r#type(), ComponentType::CircuitBreaker);

        assert!(execute_command(net, Command::Create(["ds", "2"])).is_ok());
        assert_eq!(net.get_component("2").unwrap().r#type(), ComponentType::Disconnector);

        assert!(execute_command(net, Command::Create(["es", "3"])).is_ok());
        assert_eq!(net.get_component("3").unwrap().r#type(), ComponentType::EarthingSwitch);

        assert!(execute_command(net, Command::Create(["vt", "4"])).is_ok());
        assert_eq!(net.get_component("4").unwrap().r#type(), ComponentType::VoltageTransformer);

        assert!(execute_command(net, Command::Create(["tf", "5"])).is_ok());
        assert_eq!(net.get_component("5").unwrap().r#type(), ComponentType::Transformer);

        assert!(execute_command(net, Command::Create(["node", "6"])).is_ok());
        assert_eq!(net.get_node("6").unwrap().name(), "6");

        assert!(execute_command(net, Command::Create(["node", "7"])).is_ok());

        assert!(execute_command(net, Command::Create(["garb", "name"])).is_err());

        // Delete
        assert!(execute_command(net, Command::Delete("i dont exist")).is_err());
        assert!(execute_command(net, Command::Delete("7")).is_ok());
        assert!(net.get_node("7").is_err());
        assert!(execute_command(net, Command::Delete("5")).is_ok());
        assert!(net.get_component("5").is_err());

        // List
        assert!(execute_command(net, Command::List()).is_ok());

        // Show
        assert!(execute_command(net, Command::Show("i dont exist")).is_err());
        assert!(execute_command(net, Command::Show("6")).is_ok());
        assert!(execute_command(net, Command::Show("4")).is_ok());

        // Connect
        assert!(execute_command(net, Command::Connect(["6", "1", "z"])).is_err());
        assert!(execute_command(net, Command::Connect(["1", "6", "0"])).is_err());
        assert!(execute_command(net, Command::Connect(["6", "1", "0"])).is_ok());
        assert!(net.get_component("1").unwrap().terminal(0).unwrap().borrow().get_node().is_ok());

        // Disconnect
        assert!(execute_command(net, Command::Disconnect(["1", "6"])).is_err());
        assert!(execute_command(net, Command::Disconnect(["6", "1"])).is_ok());
        assert!(net.get_component("1").unwrap().terminal(0).unwrap().borrow().get_node().is_err());

        // Open / Close
        assert!(execute_command(net, Command::Open("4")).is_err());
        assert!(execute_command(net, Command::Close("4")).is_err());

        assert!(net.get_component("1").unwrap().position().unwrap().borrow().is_open());
        assert!(execute_command(net, Command::Open("1")).is_err());
        assert!(execute_command(net, Command::Close("1")).is_ok());
        assert!(net.get_component("1").unwrap().position().unwrap().borrow().is_closed());
        assert!(execute_command(net, Command::Close("1")).is_err());
        assert!(execute_command(net, Command::Open("1")).is_ok());
        assert!(net.get_component("1").unwrap().position().unwrap().borrow().is_open());
    }
}
