use std::{collections::HashMap, fs::File};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct InstructionInfo {
    pub name: String,
    pub size: u8,
    pub opcode: HashMap<String, u8>,
    pub byte: bool
}

#[derive(Serialize, Deserialize)]
pub struct InstructionInfoFile {
    pub opcodes: HashMap<u8, String>,
    pub info: Vec<InstructionInfo>
}

impl InstructionInfo {
    pub fn new(name: impl Into<String>, size: u8, opcode: HashMap<String, u8>, byte: bool) -> Self {
        Self {
            name: name.into(),
            size,
            opcode,
            byte
        }
    }
}

pub fn get_instructions<T: Into<String>>(inst_file_path: T) -> InstructionInfoFile {
    let file = File::open(inst_file_path.into()).unwrap();

    serde_json::from_reader(file).unwrap()
}

pub fn get_instruction_info(insts: &[InstructionInfo], instruction: &str) -> Option<InstructionInfo> {
    let inst = insts.iter().find(|&x| x.name == instruction);

    inst?;

    Some(inst.unwrap().clone())
}