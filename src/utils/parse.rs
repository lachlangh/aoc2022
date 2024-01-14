use std::fs::File;
use std::io::{self, BufReader, BufRead};

pub fn read_split_paragraphs(path: &str) -> Result<Vec<Vec<String>>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut paras: Vec<Vec<String>> = vec![];
    let mut para: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            paras.push(para.clone());
            para.clear();
        } else {
            para.push(line);
        }
    }
    
    Ok(paras)
}
