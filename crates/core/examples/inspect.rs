use remote_compose_core::document::Document;
use remote_compose_core::operations::Operations;
use std::io::{self, Read};
use std::{env, process};

fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    let hex = hex.trim();
    if hex.len() % 2 != 0 {
        return Err("Hex string must have an even length".to_string());
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| format!("Invalid hex at index {}: {}", i, e))
        })
        .collect()
}

fn print_op(op: &Operations, depth: usize) {
    let indent = "  ".repeat(depth);

    if let Some(container) = op.as_container() {
        // For containers, we want to print the op itself but maybe skip its fields in debug if we print them recursively
        // But since we use {:?}, it's easier to just print it and then mention children/modifiers
        println!("{}Container: {:?}", indent, op);
        if !container.modifiers().is_empty() {
            println!("{}  Modifiers:", indent);
            for modifier in container.modifiers() {
                print_op(modifier, depth + 2);
            }
        }
        if !container.children().is_empty() {
            println!("{}  Children:", indent);
            for child in container.children() {
                print_op(child, depth + 2);
            }
        }
    } else if let Some(action_list) = op.as_action_list() {
        println!("{}ActionList: {:?}", indent, op);
        if !action_list.actions().is_empty() {
            println!("{}  Actions:", indent);
            for action in action_list.actions() {
                print_op(action, depth + 2);
            }
        }
    } else {
        println!("{}{:?}", indent, op);
    }
}

fn inspect_document(data: &[u8]) {
    println!("Document Structure:");
    match Document::parse(data) {
        Ok(doc) => {
            println!("Header: {:?}", doc.header);
            for op in &doc.root {
                print_op(op, 0);
            }
        }
        Err(e) => {
            eprintln!("\nDocument parsing failed (strict): {}", e);
            eprintln!("Retrying with lenient mode...");
            match Document::parse_lenient(data) {
                Ok(doc) => {
                    println!("Header: {:?}", doc.header);
                    println!("(Parsed with lenient mode - some blocks may be unclosed)");
                    for op in &doc.root {
                        print_op(op, 0);
                    }
                }
                Err(e_lenient) => {
                    eprintln!("\nDocument parsing failed (lenient): {}", e_lenient);
                    eprintln!("Falling back to flat inspection.");
                    inspect_operations(data);
                }
            }
        }
    }
}

fn inspect_operations(data: &[u8]) {
    println!("\nFlat list of operations:");
    let mut offset = 0;
    while offset < data.len() {
        let opcode_byte = data[offset];
        match Operations::read(&data[offset..]) {
            Ok((op, n)) => {
                println!("Offset {}: {:?} ({} bytes)", offset, op, n);
                offset += n;
            }
            Err(e) => {
                println!(
                    "Offset {}: READ FAILED (OpCode 0x{:02x}): {}",
                    offset, opcode_byte, e
                );
                // Visualize fail bytes
                let end = (offset + 16).min(data.len());
                print!("  Bytes at failure: ");
                for b in &data[offset..end] {
                    print!("{:02x} ", b);
                }
                println!();
                break;
            }
        }
    }
}

enum InputSource {
    File(String),
    Stdin,
    Hex(String),
}

fn print_usage(program: &str) {
    eprintln!("Usage: {} [OPTIONS] [INPUT]", program);
    eprintln!("  INPUT           File path, '-' for Stdin, or Hex string (if file not found)");
    eprintln!("  -f, --flat      Show flat list of operations (instead of hierarchical document)");
    eprintln!("  --help          Print this help menu");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut flat_mode = false;
    let mut input_arg: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg == "-f" || arg == "--flat" {
            flat_mode = true;
        } else if arg == "--help" || arg == "-h" {
            print_usage(&program);
            return;
        } else if arg.starts_with("-") && arg != "-" {
            eprintln!("Unknown option: {}", arg);
            print_usage(&program);
            process::exit(1);
        } else {
            if input_arg.is_some() {
                eprintln!("Error: Multiple input arguments provided.");
                print_usage(&program);
                process::exit(1);
            }
            input_arg = Some(arg.clone());
        }
        i += 1;
    }

    let input_source = match input_arg {
        Some(s) if s == "-" => InputSource::Stdin,
        Some(s) => {
            if std::path::Path::new(&s).exists() {
                InputSource::File(s)
            } else {
                InputSource::Hex(s)
            }
        }
        None => InputSource::Stdin,
    };

    let data = match input_source {
        InputSource::File(path) => {
            std::fs::read(&path).expect(&format!("Failed to read file: {}", path))
        }
        InputSource::Stdin => {
            let mut buffer = Vec::new();
            io::stdin()
                .read_to_end(&mut buffer)
                .expect("Failed to read stdin");
            buffer
        }
        InputSource::Hex(hex_str) => hex_to_bytes(&hex_str).expect("Failed to decode hex string"),
    };

    println!("Read {} bytes.", data.len());

    if flat_mode {
        inspect_operations(&data);
    } else {
        inspect_document(&data);
    }
}
