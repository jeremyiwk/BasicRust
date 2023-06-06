use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() -> io::Result<()> {
    // Read from a file and store lines in a Vec
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>()?;

    // Write lines to a file
    let mut output_file = BufWriter::new(File::create("output.txt")?);

    for line in &lines {
        writeln!(output_file, "{}", line)?;
    }

    output_file.flush()?; // Ensure all buffered data is written to the file

    Ok(())
}

