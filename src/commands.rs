use crate::args::{DecodeArgs, EncodeArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;
use std::fs;

fn get_png(file_path: &str) -> Result<Png> {
    let file = fs::read(file_path)?;
    Ok(Png::try_from(&file[..])?)
}

pub fn encode(args: &EncodeArgs) -> Result<()> {
    let mut png = get_png(&args.file_path)?;
    let chunk_type_bytes: [u8; 4] = args.chunk_type.as_bytes().try_into().unwrap();
    let chunk = Chunk::new(
        ChunkType::try_from(chunk_type_bytes)?,
        args.message.as_bytes().to_vec(),
    );
    png.append_chunk(chunk);
    let output_path = match &args.output_path {
        Some(path) => path,
        None => &args.file_path,
    };
    fs::write(output_path, png.as_bytes())?;
    Ok(())
}

pub fn decode(args: &DecodeArgs) -> Result<()> {
    let png = get_png(&args.file_path)?;
    if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
        println!("{}", chunk.data_as_string()?);
    }
    Ok(())
}

pub fn remove(args: &RemoveArgs) -> Result<()> {
    let mut png = get_png(&args.file_path)?;
    let _chunk = png.remove_first_chunk(&args.chunk_type)?;
    fs::write(&args.file_path, png.as_bytes())?;
    Ok(())
}

pub fn print(file_path: &str) -> Result<()> {
    let png = get_png(file_path)?;
    for chunk in png.chunks() {
        println!("{}", chunk);
    }
    Ok(())
}
