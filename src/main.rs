use std::env;
use std::fs::File;
use std::io::{ BufRead, BufReader, Write, BufWriter, Result, Error, ErrorKind };
use std::collections::HashMap;

fn main() {
    if let Err(error) = run_command() {
        println!("{}", error);
    }
}

fn run_command() -> Result<()> {
    let props = parse_args()?;

    let input_data: Vec<String> = BufReader::new(props.in_file)
        .lines()
        .filter_map(Result::ok)
        .collect();

    let (block_trimmed, block_defs) = get_blocks(input_data);
        
    let output_data = expand_blocks(block_trimmed, block_defs);

    let _ = BufWriter::new(props.out_file)
        .write(
            output_data
                .join("\n")
                .as_bytes()
        );

    Ok(())
}

struct Props {
    in_file: File,
    out_file: File
}

fn parse_args() -> Result<Props> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        Ok(Props {
            in_file: File::open(args[1].clone())?,
            out_file: File::create(args[2].clone())?
        })
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "Incorrect Number of args"))
    }
}

fn expand_blocks(data: Vec<String>, blocks: HashMap<String, Vec<String>>) -> Vec<String> {
    let mut result_lines = Vec::new();

    for line in data.into_iter() {
        if let Some(block) = blocks.get(line.trim()) {
            result_lines.append(&mut block.clone());
        } else {
            result_lines.push(line);
        }
    }
        
    result_lines
}

fn get_blocks(data: Vec<String>) -> (Vec<String>, HashMap<String, Vec<String>>) {
    let mut data_iter = data.into_iter();
    
    let mut result_lines = Vec::new();
    let mut block_defs = HashMap::new();

    while let Some(line) = data_iter.next() {
        if line.starts_with("#define") {
            let args: Vec<&str> = line.split(" ").collect();

            if args.len() != 2 {
                result_lines.push(line);
                continue;
            }

            let block_name = args[1].to_string();
            let mut block_contents = Vec::new();

            while let Some(line) = data_iter.next() {
                if line.starts_with("#end") {
                    break;
                }
                block_contents.push(line);
            }
            
            block_defs.insert(block_name, block_contents);
        } else {
            result_lines.push(line);
        }
    }

    (result_lines, block_defs)
}