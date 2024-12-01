use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Open the input file (replace "input.txt" with your file name)
    let file = File::open("dataday1.txt")?;
    let reader = io::BufReader::new(file);

    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();  // Split line by whitespace
        
        if parts.len() == 2 {
            let value1: i32 = parts[0].parse().unwrap();
            let value2: i32 = parts[1].parse().unwrap();

            column1.push(value1);
            column2.push(value2);
        }

       

    }
    let mut sum: i32 = 0;
    let mut number: i32 = 0;
    column1.sort();
    column2.sort();
    for i in 0..column1.len() {
        number= (column1[i] -column2[i]).abs() ;
        sum += number  as i32;
    }
    // Print the results
    println!("result: {:?}", sum);
    // Part two
    sum=0;
    number = 0;
    let mut num: i32 = 0;
    for i in 0..column1.len() {
        num=0;
        for j in 0..column1.len() {
            if column1[i]==column2[j]{
                num= num + 1;
            } ;
        }
        number=num*column1[i];
        sum += number  as i32;
    }
    println!("result: {:?}", sum);
    Ok(())
}