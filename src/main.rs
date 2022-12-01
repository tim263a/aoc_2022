use std::io::{self, Write};

use std::fs;

fn main() {
    let filepath = "inputs/day0_0.txt";
    let content : Vec<u8> = fs::read(filepath)
        .expect("Could not read from input file");
    println!("Size of '{}' is {} bytes", filepath, content.len());
    // io::stdout()
        // .write(&content)
        // .expect("Could not write to stdout");

    let mut max_sum : u32 = 0;
    let mut current_sum : u32 = 0;

    let mut max_sums : [u32; 4] = [0; 4];

    let mut i0 : usize = 0;
    let mut i1 : usize = 0;

    while true {
        if i0 >= content.len() {
            break;
        }

        if content[i0] == '\n' as u8 {
            max_sums[0] = current_sum;
            max_sums.sort();
            max_sums[0] = 0;
            current_sum = 0;
            i0 += 1;
            continue;
        }

        for i in i0..content.len() {
            if content[i] == '\n' as u8 {
                i1 = i;
                break;
            }
        }

        let line_bytes : Vec<u8> = content[i0..i1].to_vec();
        let line : String = String::from_utf8(line_bytes)
            .expect("Input not encoded in UTF-8");
        let value : u32 = line.parse::<u32>()
            .expect(format!("Could not parse line '{}' [{},{})",
                line, i0, i1).as_str());

        current_sum += value;

        println!("{:8} {:8} {:8} {:8} {:8} {:8}", value, current_sum,
            max_sums[0], max_sums[1], max_sums[2], max_sums[3]);

        i0 = i1 + 1;
    }

    println!("MaxSum: {}", max_sums[max_sums.len() - 1]);

    let mut three_sums : u32 = 0;
    for i in 1..4 {
        three_sums += max_sums[i];
    }
    println!("Three largest sums summed: {}", three_sums);
}
