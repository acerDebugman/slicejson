use std::{
    fs::File,
    io::{BufRead, Write},
    path::Path,
};

use clap::ArgAction;
pub fn main() -> anyhow::Result<()> {
    test_from_file()?;
    Ok(())
}

pub fn test_from_file() -> anyhow::Result<()> {
    let matches = clap::Command::new("test_json")
        .args(&[
            clap::arg!(-i --input <FILE> "input file"),
            clap::arg!(-n --epoch <FILE> "epoch num"),
        ])
        .arg(
            clap::Arg::new("simd")
                .short('s')
                .long("simd")
                .value_name("true/false")
                .help("open simd process")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let infile = matches
        .get_one::<String>("input")
        .expect("no input file")
        .as_str();
    let epoch_n = matches
        .get_one::<String>("epoch")
        .unwrap_or(&"1".to_string())
        .parse::<i32>()
        .unwrap_or(1);

    let is_simd = matches.get_flag("simd");
    println!("input file: {infile}");
    println!("epoch num: {epoch_n}");
    println!("is simd: {is_simd}");

    // let mut outbuf = vec![];
    let file_name = std::path::Path::new(infile);
    let lines = read_lines(file_name).unwrap();
    let mut lines = lines.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>();
    let total_line = lines.len() as i32 * epoch_n as i32;

    println!("start to count time...");
    let now = std::time::Instant::now();

    for _ in 0..epoch_n {
        for line in lines.iter_mut() {
            // println!("line: {}", line);
            // let _ = slicejson::parser::parse(&line);
            // let mut s = String::from(line);
            // let b = unsafe { s.as_bytes_mut() };
            let b = unsafe { line.as_bytes_mut() };
            let _ = simd_json::serde::from_slice::<serde_json::Value>(b);
        }
    }

    let cost = now.elapsed().as_micros() as f32 / 1000.0;
    println!(
        "[time cost] total line: {}, total cost: {} ms, avg: {} ms",
        total_line,
        cost,
        cost / total_line as f32
    );

    Ok(())
}

fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}
