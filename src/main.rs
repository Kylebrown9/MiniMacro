use std::env;
use std::fs::File;
use std::io::{ BufRead, BufReader, Write, BufWriter, Result, Error, ErrorKind };
use std::collections::HashMap;

fn main() {
    let props = parse_args().unwrap();

    let input_data: Vec<String> = BufReader::new(props.in_file)
        .lines()
        .filter_map(Result::ok)
        .collect();

    let (block_trimmed, block_defs) = get_blocks(input_data);
        
    let output_data = expand_blocks(block_trimmed, block_defs);

    let mut writer = BufWriter::new(props.out_file);

    for line in output_data {
        let _ = writer.write(line.as_bytes());
    }
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

    for line in data {
        if let Some(block) = blocks.get((&line).trim()) {
            for block_line in block {
                result_lines.push(block_line.clone());
                result_lines.push(String::from("\n"));
            }
        } else {
            result_lines.push(line.clone());
            result_lines.push(String::from("\n"));
        }
    }
        
    result_lines
}

fn get_blocks(data: Vec<String>) -> (Vec<String>, HashMap<String, Vec<String>>) {
    let mut block_defs = HashMap::new();

    let mut result_lines: Vec<String> = Vec::new();

    let mut in_block = false;
    let mut block_name = String::new();
    let mut block_contents = Vec::new();

    for line in data.iter() {
        if in_block {
            if line.starts_with("#end") {
                block_defs.insert(block_name, block_contents);

                block_name = String::new();
                block_contents = Vec::new();
                
                in_block = false;
            } else {
                block_contents.push(line.clone());
            }
        } else {
            if line.starts_with("#define") {
                let args: Vec<&str> = line.split(" ").collect();

                if args.len() == 2 {
                    in_block = true;
                    block_name = args[1].to_string();
                    block_contents = Vec::new();
                } else {
                    result_lines.push(line.clone());
                }
            } else {
                result_lines.push(line.clone());
            }
        }
    }

    (result_lines, block_defs)
}