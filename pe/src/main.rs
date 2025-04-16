use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{fs::File, io::BufReader};

use anyhow::Result;
use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;
use clap::Parser;
use fastnbt::{Value, from_le_bytes};
use mcnbt::Tag;
use serde::{Deserialize, Serialize};

pub mod models;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    world_folder: PathBuf,
}

fn main() -> Result<()> {
    println!("Hello, world!");
    let args = Args::parse();

    let mut level_dat_file_path = args.world_folder.clone();
    level_dat_file_path.push("level.dat");
    println!("Reading file {level_dat_file_path:?}");

    let mut level_dat_file = BufReader::new(File::open(level_dat_file_path)?);

    let mut storage_version = [0u8; 4];
    let mut level_dat_length = [0u8; 4];

    level_dat_file.read_exact(&mut storage_version)?;
    level_dat_file.read_exact(&mut level_dat_length)?;

    let storage_version = i32::from_le_bytes(storage_version);
    let level_dat_length = i32::from_le_bytes(level_dat_length);

    println!("Storage version: {storage_version}, level.dat length: {level_dat_length}");

    //let mut level_dat_bytes = Vec::new();
    //level_dat_file.read_to_end(&mut level_dat_bytes)?;

    //let mut level_dat_bytes = Bytes::from(level_dat_bytes);
    //let level_dat = Tag::from_bytes(&mut level_dat_bytes, mcnbt::ByteOrder::LittleEndian)?;
    ////dbg!(&level_dat);

    //println!("{}", PrettyNbt(level_dat));

    let mut level_dat_bytes = Vec::new();
    level_dat_file.read_to_end(&mut level_dat_bytes)?;

    let mut level_dat_bytes = Bytes::from(level_dat_bytes);
    let level_dat: Value = fastnbt::from_le_bytes(&mut level_dat_bytes).unwrap();
    //println!("{level_dat:?}");
    println!("{}", PrettyNbt(&level_dat));

    use models::pe0_8_1::*;
    let level_dat: Result<LevelDat, _> = from_le_bytes(&level_dat_bytes);
    println!("{level_dat:?}");

    let mut level_dat = level_dat.unwrap();
    level_dat.LevelName = "LAN World".into();

    let new_bytes = fastnbt::to_le_bytes(&level_dat).unwrap();
    let mut outfile = std::fs::File::create("level.dat").unwrap();
    outfile.write(&i32::to_le_bytes(3)).unwrap();
    outfile
        .write(&i32::to_le_bytes(new_bytes.len() as i32))
        .unwrap();
    outfile.write_all(&new_bytes).unwrap();

    Ok(())
}

//#[derive(Debug)]
//struct PrettyNbt_(Tag);
//
//fn str_or_empty(name: &Option<String>) -> &str {
//    match name {
//        Some(s) => s.as_str(),
//        None => "",
//    }
//}
//
//fn print(tag: &Tag, indent: &mut String, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//    write!(f, "{indent}")?;
//    match tag {
//        Tag::Byte(name, value) => writeln!(f, "[Byte] '{}'={value}", str_or_empty(name)),
//        Tag::Short(name, value) => writeln!(f, "[Short] '{}'={value}", str_or_empty(name)),
//        Tag::Int(name, value) => writeln!(f, "[Int] '{}'={value}", str_or_empty(name)),
//        Tag::Long(name, value) => writeln!(f, "[Long] '{}'={value}", str_or_empty(name)),
//        Tag::Float(name, value) => writeln!(f, "[Float] '{}'={value}", str_or_empty(name)),
//        Tag::Double(name, value) => writeln!(f, "[Double] '{}'={value}", str_or_empty(name)),
//        Tag::ByteArray(name, items) => {
//            writeln!(f, "[ByteArray] '{}'={items:#?}", str_or_empty(name))
//        }
//        Tag::String(name, value) => writeln!(f, "[String] '{}'=\"{value}\"", str_or_empty(name)),
//        Tag::List(name, tags) => {
//            writeln!(f, "[List] '{}=[", str_or_empty(name))?;
//            indent.push_str("  ");
//            for tag in tags {
//                print(tag, indent, f)?;
//            }
//            indent.truncate(indent.len() - 2);
//            writeln!(f, "{indent}]")
//        }
//        Tag::Compound(name, tags) => {
//            writeln!(f, "[Compound] '{}'={{", str_or_empty(name))?;
//            indent.push_str("  ");
//            for tag in tags {
//                print(tag, indent, f)?;
//            }
//            indent.truncate(indent.len() - 2);
//            writeln!(f, "{indent}}}")
//        }
//        Tag::IntArray(name, items) => {
//            writeln!(f, "[IntArray] '{}'={items:#?}", str_or_empty(name))
//        }
//        Tag::LongArray(name, items) => {
//            writeln!(f, "[LongArray] '{}'={items:#?}", str_or_empty(name))
//        }
//    }
//}
//fn print2(tag: &Tag, indent: &mut String, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//    write!(f, "{indent}")?;
//    match tag {
//        Tag::Byte(name, value) => writeln!(f, "'{}'={value} i8", str_or_empty(name)),
//        Tag::Short(name, value) => writeln!(f, "'{}'={value} i16", str_or_empty(name)),
//        Tag::Int(name, value) => writeln!(f, "'{}'={value} i32", str_or_empty(name)),
//        Tag::Long(name, value) => writeln!(f, "'{}'={value} i64", str_or_empty(name)),
//        Tag::Float(name, value) => writeln!(f, "'{}'={value} f32", str_or_empty(name)),
//        Tag::Double(name, value) => writeln!(f, "'{}'={value} f64", str_or_empty(name)),
//        Tag::ByteArray(name, items) => {
//            writeln!(f, "'{}'={items:?} [i8]", str_or_empty(name))
//        }
//        Tag::String(name, value) => writeln!(f, "'{}'=\"{value}\"", str_or_empty(name)),
//        Tag::List(name, tags) => {
//            writeln!(f, "'{}=[", str_or_empty(name))?;
//            indent.push_str("  ");
//            for tag in tags {
//                print2(tag, indent, f)?;
//            }
//            indent.truncate(indent.len() - 2);
//            writeln!(f, "{indent}]")
//        }
//        Tag::Compound(name, tags) => {
//            writeln!(f, "'{}'={{", str_or_empty(name))?;
//            indent.push_str("  ");
//            for tag in tags {
//                print2(tag, indent, f)?;
//            }
//            indent.truncate(indent.len() - 2);
//            writeln!(f, "{indent}}}")
//        }
//        Tag::IntArray(name, items) => {
//            writeln!(f, "'{}'={items:#?} [i32]", str_or_empty(name))
//        }
//        Tag::LongArray(name, items) => {
//            writeln!(f, "'{}'={items:#?} [i64]", str_or_empty(name))
//        }
//    }
//}
//
//impl std::fmt::Display for PrettyNbt_ {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        print2(&self.0, &mut "".into(), f)
//    }
//}

#[derive(Debug)]
#[allow(dead_code)]
struct PrettyNbt<'a>(&'a Value);

//#[allow(dead_code)]
//fn print(tag: &Tag, indent: &mut String, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//    write!(f, "{indent}")?;
//    match tag {
//        Tag::Byte(name, value) => writeln!(f, "[Byte] '{}'={value}", str_or_empty(name)),
//        Tag::Short(name, value) => writeln!(f, "[Short] '{}'={value}", str_or_empty(name)),
//        Tag::Int(name, value) => writeln!(f, "[Int] '{}'={value}", str_or_empty(name)),
//        Tag::Long(name, value) => writeln!(f, "[Long] '{}'={value}", str_or_empty(name)),
//        Tag::Float(name, value) => writeln!(f, "[Float] '{}'={value}", str_or_empty(name)),
//        Tag::Double(name, value) => writeln!(f, "[Double] '{}'={value}", str_or_empty(name)),
//        Tag::ByteArray(name, items) => {
//            writeln!(f, "[ByteArray] '{}'={items:#?}", str_or_empty(name))
//        }
//        Tag::String(name, value) => writeln!(f, "[String] '{}'=\"{value}\"", str_or_empty(name)),
//        Tag::List(name, tags) => {
//            writeln!(f, "[List] '{}=[", str_or_empty(name))?;
//            indent.push_str("  ");
//            for tag in tags {
//                print(tag, indent, f)?;
//            }
//            indent.truncate(indent.len() - 2);
//            writeln!(f, "{indent}]")
//        }
//        Tag::Compound(name, tags) => {
//            writeln!(f, "[Compound] '{}'={{", str_or_empty(name))?;
//            indent.push_str("  ");
//            for tag in tags {
//                print(tag, indent, f)?;
//            }
//            indent.truncate(indent.len() - 2);
//            writeln!(f, "{indent}}}")
//        }
//        Tag::IntArray(name, items) => {
//            writeln!(f, "[IntArray] '{}'={items:#?}", str_or_empty(name))
//        }
//        Tag::LongArray(name, items) => {
//            writeln!(f, "[LongArray] '{}'={items:#?}", str_or_empty(name))
//        }
//    }
//}

fn tag_to_str(value: &Value) -> &str {
    match value {
        Value::Byte(_) => "i8",
        Value::Short(_) => "i16",
        Value::Int(_) => "i32",
        Value::Long(_) => "i64",
        Value::Float(_) => "f32",
        Value::Double(_) => "f64",
        Value::String(_) => "string",
        Value::ByteArray(_) => "[i8]",
        Value::IntArray(_) => "[i32]",
        Value::LongArray(_) => "[i64]",
        Value::List(_) => "List",
        Value::Compound(_) => "{}",
    }
}

fn print2(value: &Value, indent: &mut String, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match value {
        Value::Byte(value) => writeln!(f, "{value}"),
        Value::Short(value) => writeln!(f, "{value}"),
        Value::Int(value) => writeln!(f, "{value}"),
        Value::Long(value) => writeln!(f, "{value}"),
        Value::Float(value) => writeln!(f, "{value}"),
        Value::Double(value) => writeln!(f, "{value}"),
        Value::String(value) => writeln!(f, "'{value}'"),
        Value::ByteArray(byte_array) => {
            writeln!(f, "{byte_array:?}")
        }
        Value::IntArray(int_array) => {
            writeln!(f, "{int_array:#?}")
        }
        Value::LongArray(long_array) => {
            writeln!(f, "{long_array:#?}")
        }
        Value::List(values) => {
            writeln!(f, "[")?;
            indent.push_str("  ");
            for tag in values {
                write!(f, "{indent}")?;
                print2(tag, indent, f)?;
            }
            indent.truncate(indent.len() - 2);
            writeln!(f, "{indent}]")
        }
        Value::Compound(hash_map) => {
            writeln!(f, "{{")?;
            indent.push_str("  ");
            for (name, value) in hash_map {
                write!(f, "{indent}{name}: {} = ", tag_to_str(value))?;
                print2(value, indent, f)?;
            }
            indent.truncate(indent.len() - 2);
            writeln!(f, "{indent}}}")
        }
    }
}

impl std::fmt::Display for PrettyNbt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print2(self.0, &mut "".into(), f)
    }
}
