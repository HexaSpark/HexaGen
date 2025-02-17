use std::{collections::HashMap, fs::File, hash::Hash};
use serde::{Deserialize, Serialize};
use HexaGen::*;

fn gen_opcode(opcode_str: impl Into<String>) -> u8 {
    let mut res = 0_u8;

    for chr in opcode_str.into().chars() {
        (res, _) = res.overflowing_add(chr as u8);
    }
    
    res
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
enum InstructionSize {
    Implied,
    Single,
    RegisterAndData
}

#[derive(Deserialize, Debug, Clone)]
struct RawInstructionInfo {
    name: String,
    size: InstructionSize,
    mode_support: Option<String>,
    has_byte_variant: Option<bool>,
    is_byte_variant: Option<bool>
}

fn create_instruction_info(info: RawInstructionInfo, all_opcodes: &mut Vec<u8>, inst_info: &mut Vec<InstructionInfo>, opcode_info: &mut HashMap<u8, String>) {
    let opcode: HashMap<String, u8> = if let InstructionSize::Implied = info.size {
        let mut opcode_str = info.name.to_ascii_uppercase();
        let mut opcode = gen_opcode(&opcode_str);

        if all_opcodes.contains(&opcode) {
            let mut iterations: u8 = 0;
            loop {
                opcode_str += " ";
                opcode = gen_opcode(&opcode_str);

                if !all_opcodes.contains(&opcode) {
                    break;
                } else if iterations > 100 {
                    panic!("Could not find a opcode that isn't taken. (Rare)");
                }

                iterations += 1;
            }
        }

        opcode_info.insert(opcode, format!("{}|M", info.name));
        all_opcodes.push(opcode);

        HashMap::from([
            ("M".into(), opcode)
        ])
    } else {
        let mut res: HashMap<String, u8> = HashMap::new();

        let mode_support = info.mode_support.clone().unwrap_or_default();
        for mode in mode_support.chars() {
            let mut opcode_str = format!("{}{}", info.name.to_ascii_uppercase(), mode);
            let mut opcode = gen_opcode(&opcode_str);

            if all_opcodes.contains(&opcode) {
                let mut iterations: u8 = 0;
                loop {
                    opcode_str += " ";
                    opcode = gen_opcode(&opcode_str);

                    if !all_opcodes.contains(&opcode) {
                        break;
                    } else if iterations > 100 {
                        panic!("Could not find a opcode that isn't taken. (Rare)");
                    }

                    iterations += 1;
                }
            }

            let key = match info.size {
                InstructionSize::Single => mode.to_string(),
                InstructionSize::RegisterAndData => format!("R{}", mode),
                _ => panic!("Should not reach this panic")
            };

            opcode_info.insert(opcode, format!("{}|{}", info.name, mode));
            res.insert(key, opcode);
            all_opcodes.push(opcode);
        }

        res
    }; 

    if let Some(has_byte_variant) = info.has_byte_variant {
        if has_byte_variant {
            let mut cloned_info = info.clone();
            cloned_info.has_byte_variant = Some(false);
            cloned_info.is_byte_variant = Some(true);
            cloned_info.name += "b";

            create_instruction_info(cloned_info, all_opcodes, inst_info, opcode_info);
        }
    }

    inst_info.push(InstructionInfo::new(info.name, info.size as u8, opcode, info.is_byte_variant.unwrap_or(false)));
}

fn main() {
    let info_file = File::open("info.json").unwrap();

    let info: Vec<RawInstructionInfo> = serde_json::from_reader(info_file).unwrap();
    let mut all_opcodes: Vec<u8> = vec![];
    let mut opcode_info: HashMap<u8, String> = HashMap::new();
    let mut inst_info: Vec<InstructionInfo> = vec![];

    for raw_info in info {
        if let Some(val) = raw_info.has_byte_variant {
            if val {
                println!("{}b", raw_info.name);
            }
        }
        println!("{}", raw_info.name);

        create_instruction_info(raw_info, &mut all_opcodes, &mut inst_info, &mut opcode_info);
    }

    let instruction_file = File::create("instructions.json").unwrap();

    serde_json::to_writer(&instruction_file, &InstructionInfoFile {
        opcodes: opcode_info,
        info: inst_info
    }).unwrap();
}
