use std::collections::btree_set::Intersection;
use std::collections::{HashMap, HashSet};
use std::process::id;
use std::{fs, option};
use std::io::{self, Write};
use std::str::Split;
use std::time::Instant;

fn main() {
    // day01();
    // day02_01();
    // day02_02();
    // day03_01();
    // day03_02();
    // day04_01();
    // day04_02();
    // day05_01();
    // day09_01();
    // day09_02();
    day10_02();
}

fn day01() {
    let start = Instant::now();
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

    loop {
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

        // println!("{:8} {:8} {:8} {:8} {:8} {:8}", value, current_sum,
            // max_sums[0], max_sums[1], max_sums[2], max_sums[3]);

        i0 = i1 + 1;
    }

    println!("MaxSum: {}", max_sums[max_sums.len() - 1]);

    let mut three_sums : u32 = 0;
    for i in 1..4 {
        three_sums += max_sums[i];
    }
    println!("Three largest sums summed: {}", three_sums);

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn day02_01() {
    let start = Instant::now();
    let filepath = "inputs/day1_0.txt";
    let content : String = fs::read_to_string(filepath)
        .expect(format!("Could not read from input file '{}'", filepath).as_str());
    println!("Size of '{}' is {} bytes", filepath, content.len());

    // Results for options X (rock), Y (paper), Z (scissor).
    let options_a : [u32; 3] = [ 4, 8, 3 ]; // opponent plays rock
    let options_b : [u32; 3] = [ 1, 5, 9 ]; // opponent plays paper
    let options_c : [u32; 3] = [ 7, 2, 6 ]; // opponent plays scissors

    let result : u32 = content
        .split("\n")
        .map(|line| {
            let split : Vec<&str> = line.trim().split(" ").collect::<Vec<&str>>();
            if split.len() < 2 {
                return 0;
            }
            let choice_oponent : &str = split[0];
            let choice_self : &str = split[1];

            let index_self : usize = match choice_self {
                "X" => 0,
                "Y" => 1,
                "Z" => 2,
                &_ => panic!("Invalid choice of self")
            };

            let result : u32 = match choice_oponent {
                "A" => options_a[index_self],
                "B" => options_b[index_self],
                "C" => options_c[index_self],
                &_ => panic!("Invalid choice of opponent")
            };

            println!("{} {} -> {}", choice_oponent, choice_self, result);
            return result;
        }).sum();

    println!("Result: {}", result);

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn day02_02() {
    let start = Instant::now();
    let filepath = "inputs/day1_0.txt";
    let content : String = fs::read_to_string(filepath)
        .expect(format!("Could not read from input file '{}'", filepath).as_str());
    println!("Size of '{}' is {} bytes", filepath, content.len());

    // Values; rock (1), paper (2) scissor (3)

    // Results for options X (lose), Y (draw), Z (win).
    let options_a : [u32; 3] = [ 3, 4, 8 ]; // opponent plays rock
    let options_b : [u32; 3] = [ 1, 5, 9 ]; // opponent plays paper
    let options_c : [u32; 3] = [ 2, 6, 7 ]; // opponent plays scissors

    let result : u32 = content
        .split("\n")
        .map(|line| {
            let split : Vec<&str> = line.trim().split(" ").collect::<Vec<&str>>();
            if split.len() < 2 {
                return 0;
            }
            let choice_oponent : &str = split[0];
            let choice_self : &str = split[1];

            let index_self : usize = match choice_self {
                "X" => 0,
                "Y" => 1,
                "Z" => 2,
                &_ => panic!("Invalid choice of self")
            };

            let result : u32 = match choice_oponent {
                "A" => options_a[index_self],
                "B" => options_b[index_self],
                "C" => options_c[index_self],
                &_ => panic!("Invalid choice of opponent")
            };

            println!("{} {} -> {}", choice_oponent, choice_self, result);
            return result;
        }).sum();

    println!("Result: {}", result);

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn day03_01() {
    let start = Instant::now();
    let filepath = "inputs/day2_0.txt";
    let content : String = fs::read_to_string(filepath)
        .expect(format!("Could not read from input file '{}'", filepath).as_str());
    println!("Size of '{}' is {} bytes", filepath, content.len());

    let mut set : HashSet<u8> = HashSet::new();

    let result : u32 = content
        .split("\n")
        .map(|line| {
            let length : usize = line.len();
            if length < 2 {
                return Vec::new();
            }
            let split_idx = length / 2;
            return vec![
                line[0..split_idx].to_string(),
                line[split_idx..length].to_string()];
        })
        .map(|compartments| {
            if compartments.len() < 2 {
                return 0;
            }

            let left : &String = &compartments[0];
            let right : &String = &compartments[1];

            for c in left.chars() {
                set.insert(c as u8);
            }

            let mut found_value : u8 = 0;

            for c in right.chars() {
                if set.contains(&(c as u8)) {
                    found_value = c as u8;
                    break;
                }
            }

            set.clear();

            let mut result : u8 = 0;

            if found_value >= 'a' as u8 && found_value <= 'z' as u8 {
                result = found_value - 'a' as u8 + 1;
            }

            if found_value >= 'A' as u8 && found_value <= 'Z' as u8 {
                result = found_value - 'A' as u8 + 27;
            }

            println!("{:40} {:40} {} -> {}",
                left, right, found_value as char, result);

            return result as u32;
        })
        .sum();

    println!("Result: {}", result);

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn day03_02() {
    let start = Instant::now();
    let filepath = "inputs/day2_0.txt";
    let content : String = fs::read_to_string(filepath)
        .expect(format!("Could not read from input file '{}'", filepath).as_str());
    println!("Size of '{}' is {} bytes", filepath, content.len());

    let mut idx_in_group : u32 = 0;
    let mut found : Vec<char> = Vec::new();

    content.lines()
        .map(|line| {
            return line.chars().collect::<Vec<char>>();
        })
        .fold(Vec::new(), |accu, new| {
            let mut result : Vec<char>;
            if idx_in_group == 0 {
                result = new;
            } else {
                result = accu.clone();
                result.retain(|c| {
                    return new.contains(c);
                });
            }
            idx_in_group = (idx_in_group + 1) % 3;
            if idx_in_group == 0 {
                println!("{}", result.iter().cloned().collect::<String>());
                found.push(result[0]);
            }
            return result;
        });

    let result : u32 = found.iter()
        .map(|c| {
            let c_as_u8 : u8 = c.clone() as u8;
            let mut value : u32 = 0;

            if c_as_u8 >= 'a' as u8 && c_as_u8 <= 'z' as u8 {
                value = (c_as_u8 - 'a' as u8 + 1) as u32;
            }

            if c_as_u8 >= 'A' as u8 && c_as_u8 <= 'Z' as u8 {
                value = (c_as_u8 - 'A' as u8 + 27) as u32;
            }

            println!("{} {}", c, value);

            return value;
        }).sum();

    println!("Result: {}", result);

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn day04_01() {
    let start = Instant::now();
    let filepath = "inputs/day3_0.txt";
    let content : String = fs::read_to_string(filepath)
        .expect(format!("Could not read from input file '{}'", filepath).as_str());
    println!("Size of '{}' is {} bytes", filepath, content.len());

    let result : u32 = content.lines()
        .map(|line| {
            return line
                .split(",")
                .map(|pairing| {
                    return pairing.split("-").collect::<Vec<&str>>();
                })
                .reduce(|accu, new| {
                    let mut p1 = accu.clone();
                    let mut p2 = new.clone();
                    p1.append(&mut p2);
                    return p1;
                })
                .unwrap();
        })
        .map(|pairings| {
            return pairings.iter()
                .map(|s| {
                    return s.parse::<u32>().unwrap();
                }).collect();
        })
        .map(|pairings : Vec<u32>| {
            let result : u32;
            if (pairings[0] <= pairings[2] && pairings[1] >= pairings[3]) ||
                (pairings[2] <= pairings[0] && pairings[3] >= pairings[1]) {
                result = 1;
            } else {
                result = 0;
            }
            println!("{} {} {} {} -> {}",
                pairings[0], pairings[1], pairings[2], pairings[3], result);
            return result;
        }).sum();

    println!("Result: {}", result);

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn day04_02() {
    let start = Instant::now();
    let filepath = "inputs/day3_0.txt";
    let content : String = fs::read_to_string(filepath)
        .expect(format!("Could not read from input file '{}'", filepath).as_str());
    println!("Size of '{}' is {} bytes", filepath, content.len());

    let result : u32 = content.lines()
        .map(|line| {
            return line
                .split(",")
                .map(|pairing| {
                    return pairing.split("-").collect::<Vec<&str>>();
                })
                .reduce(|accu, new| {
                    let mut p1 = accu.clone();
                    let mut p2 = new.clone();
                    p1.append(&mut p2);
                    return p1;
                })
                .unwrap();
        })
        .map(|pairings| {
            return pairings.iter()
                .map(|s| {
                    return s.parse::<u32>().unwrap();
                }).collect();
        })
        .map(|pairings : Vec<u32>| {
            let result : u32;
            if (pairings[0] >= pairings[2] && pairings[0] <= pairings[3]) ||
                (pairings[1] >= pairings[2] && pairings[1] <= pairings[3]) ||
                (pairings[2] >= pairings[0] && pairings[2] <= pairings[1]) ||
                (pairings[3] >= pairings[0] && pairings[3] <= pairings[1]) {
                result = 1;
            } else {
                result = 0;
            }
            println!("{} {} {} {} -> {}",
                pairings[0], pairings[1], pairings[2], pairings[3], result);
            return result;
        }).sum();

    println!("Result: {}", result);

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn day05_01() {
    let start = Instant::now();
    let filepath = "inputs/day4_0.txt";
    let content : String = fs::read_to_string(filepath)
        .expect(format!("Could not read from input file '{}'", filepath).as_str());
    println!("Size of '{}' is {} bytes", filepath, content.len());

    let mut row_over_column : Vec<Vec<u8>> = Vec::new();
    let mut n_columns = 0;

    for line in content.lines() {
        if line.len() == 0 || line.starts_with(" 1") {
            break;
        }

        let mut row : Vec<u8> = Vec::new();
        for i in 0..line.len() {
            if (i % 4) == 1 {
                row.push(line.as_bytes()[i]);
                n_columns = n_columns.max(i / 4 + 1);
            }
        }
        row_over_column.push(row);
    }

    println!("{}", n_columns);

    let mut column_over_row : Vec<Vec<u8>> = Vec::new();
    for i in 0..n_columns {
        let mut column : Vec<u8> = Vec::new();
        for row in row_over_column.iter() {
            if i >= row.len() {
                // column.push(0);
            } else if row[i] != ' ' as u8 {
                column.push(row[i]);
            }
        }
        column.reverse();
        column_over_row.push(column);
    }

    for column in &column_over_row {
        println!("{}", String::from_utf8(column.to_vec()).unwrap());
    }

    for line in content.lines() {
        if !line.starts_with("move") {
            continue;
        }

        let mut line_split = line.split(" ");
        let str_count : u8 = line_split.nth(1).unwrap().parse::<u8>().unwrap();
        let str_src : u8 = line_split.nth(1).unwrap().parse::<u8>().unwrap() - 1;
        let str_dst : u8 = line_split.nth(1).unwrap().parse::<u8>().unwrap() - 1;

        println!("Move: {} {} {}", str_count, str_src, str_dst);
        println!("Move: {} {} {}", str_count, str_src, str_dst);

        /*
        for _ in 0..str_count {
            let popped = column_over_row[str_src as usize].pop().unwrap();
            println!("{}", popped);
            column_over_row[str_dst as usize].push(popped);
        }
        */

        let mut stash = Vec::new();
        for _ in 0..str_count {
            let popped = column_over_row[str_src as usize].pop().unwrap();
            stash.push(popped);
        }

        for _ in 0..str_count {
            column_over_row[str_dst as usize].push(stash.pop().unwrap());
        }
    }

    let result = column_over_row.iter()
        .map(|column| {
            return *column.last().unwrap();
        })
        .collect::<Vec<u8>>();

    println!("Result: {}", String::from_utf8(result.to_vec()).unwrap());

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

fn day09_01() {
    let start = Instant::now();
    let filepath = "inputs/day9_0.txt";
    let content : String = fs::read_to_string(filepath)
        .expect(format!("Could not read from input file '{}'", filepath).as_str());
    println!("Size of '{}' is {} bytes", filepath, content.len());

    let mut visited : HashSet<Position> = HashSet::new();

    let mut tail : Position = Position {
        x: 0,
        y: 0
    };

    let mut head : Position = Position {
        x: 0,
        y: 0
    };

    let mut update_tail = |head: &mut Position, tail: &mut Position| {
        let d_x : i32 = head.x - tail.x;
        let d_y: i32 = head.y - tail.y;

        visited.insert(tail.clone());

        if d_x.abs() >= 2 || d_y.abs() >= 2 {
            tail.x += d_x.signum();
            tail.y += d_y.signum();
        }

        visited.insert(tail.clone());
    };

    for line in content.lines() {
        if line.is_empty() {
            continue;
        }

        let mut iter_split = line.split(" ");
        let str_direction = iter_split.nth(0).unwrap();
        let str_count = iter_split.nth(0).unwrap();

        let direction : (i32, i32) = match str_direction {
            "U" => ( 0,  1),
            "D" => ( 0, -1),
            "L" => (-1,  0),
            "R" => ( 1,  0),
            _ => panic!("Invalid direction: {}", str_direction)
        };

        for i in 0..str_count.parse::<u32>().unwrap() {
            head.x += direction.0;
            head.y += direction.1;

            update_tail(&mut head, &mut tail);

            println!("{} {} {} {} {} {} {}",
                str_direction, str_count, i,
                head.x, head.y, tail.x, tail.y);
        }
    }

    println!("Result: {}", visited.iter().count());

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn day09_02() {
    let start = Instant::now();
    let filepath = "inputs/day9_0.txt";
    let content : String = fs::read_to_string(filepath)
        .expect(format!("Could not read from input file '{}'", filepath).as_str());
    println!("Size of '{}' is {} bytes", filepath, content.len());

    let mut visited : HashSet<Position> = HashSet::new();

    let mut tail : Vec<Position> = Vec::new();

    for i in 0..9 {
        tail.push(Position {
            x: 0,
            y: 0
        });
    }

    let mut head : Position = Position {
        x: 0,
        y: 0
    };

    let update_tail = |head: &mut Position, tail: &mut Position| {
        let d_x : i32 = head.x - tail.x;
        let d_y: i32 = head.y - tail.y;

        if d_x.abs() >= 2 || d_y.abs() >= 2 {
            tail.x += d_x.signum();
            tail.y += d_y.signum();
        }
    };

    for line in content.lines() {
        if line.is_empty() {
            continue;
        }

        let mut iter_split = line.split(" ");
        let str_direction = iter_split.nth(0).unwrap();
        let str_count = iter_split.nth(0).unwrap();

        let direction : (i32, i32) = match str_direction {
            "U" => ( 0,  1),
            "D" => ( 0, -1),
            "L" => (-1,  0),
            "R" => ( 1,  0),
            _ => panic!("Invalid direction: {}", str_direction)
        };

        for i in 0..str_count.parse::<u32>().unwrap() {
            head.x += direction.0;
            head.y += direction.1;

            visited.insert(tail.last().unwrap().clone());

            update_tail(&mut head, &mut tail[0]);

            for j in 0..tail.len() - 1 {
                let mut it = tail.iter_mut();
                let mut tail_0 = it.nth(j).unwrap();
                let mut tail_1 = it.nth(0).unwrap();

                update_tail(&mut tail_0, &mut tail_1);
            }

            visited.insert(tail.last().unwrap().clone());

            println!("{} {} {} {} {} {} {}",
                str_direction, str_count, i,
                head.x, head.y, tail.last().unwrap().x, tail.last().unwrap().y);

            for element in tail.clone() {
                println!("{} {}", element.x, element.y);
            }
        }
    }

    println!("Result: {}", visited.iter().count());

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn day10_01() {
    let start = Instant::now();
    let filepath = "inputs/day10_0.txt";
    let content : String = fs::read_to_string(filepath)
        .expect(format!("Could not read from input file '{}'", filepath).as_str());
    println!("Size of '{}' is {} bytes", filepath, content.len());

    let mut value_x : i32 = 1;
    let mut cycle : u32 = 0;

    let mut signal_strengths : Vec<(u32, i32)> = Vec::new();

    let mut next_cycle = |value_x : &mut i32, cycle : &mut u32, count: u32| {
        for _ in 0..count {
            *cycle += 1;
            if *cycle % 40 == 20 {
                signal_strengths.push((*cycle, *value_x));
            }
        }
    };

    for line in content.lines() {
        let mut iter_split = line.split(" ").into_iter();
        let str_command : &str = iter_split.nth(0).unwrap();

        println!("{}", str_command);

        if str_command == "noop" {
            next_cycle(&mut value_x, &mut cycle, 1);
        } else if str_command == "addx" {
            let str_count : &str = iter_split.nth(0).unwrap();
            let count : i32 = str_count.parse().unwrap();

            next_cycle(&mut value_x, &mut cycle, 2);
            value_x += count;
        }
    }

    let mut result : i32 = 0;

    for signal_strength in signal_strengths {
        println!("{} {}", signal_strength.0, signal_strength.1);
        result += signal_strength.0 as i32 * signal_strength.1;
    }

    println!("Result: {}", result);

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn day10_02() {
    let start = Instant::now();
    let filepath = "inputs/day10_0.txt";
    let content : String = fs::read_to_string(filepath)
        .expect(format!("Could not read from input file '{}'", filepath).as_str());
    println!("Size of '{}' is {} bytes", filepath, content.len());

    const WIDTH : u32 = 40;
    const HEIGHT : u32 = 6;
    const SCREEN_SIZE : usize = (WIDTH * HEIGHT) as usize;

    let mut screen : [bool; SCREEN_SIZE] = [false; SCREEN_SIZE];

    let mut value_x : i32 = 1;
    let mut cycle : u32 = 0;

    let print_screen = |screen : [bool; SCREEN_SIZE]| {
        for row in 0..HEIGHT {
            print!("{} ", row);
            for column in 0..WIDTH {
                if screen[(row * WIDTH + column) as usize] {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    };

    let mut set_pixel = |value_x : &mut i32, cycle : &mut u32| {
        if *cycle < 1 || *cycle as usize > SCREEN_SIZE {
            return;
        }

        // Shadow cycle, there is a offset somewhere
        let cycle : u32 = *cycle - 1;

        let column : i32 = (cycle % WIDTH) as i32;
        screen[cycle as usize] = *value_x >= column - 1 && *value_x <= column + 1;

        println!("{} {} {} {}", cycle, value_x, column, screen[cycle as usize]);

        print_screen(screen);
    };

    let mut next_cycle = |value_x : &mut i32, cycle : &mut u32, count: u32| {
        for _ in 0..count {
            *cycle += 1;
            set_pixel(value_x, cycle);
        }
    };

    for line in content.lines() {
        let mut iter_split = line.split(" ").into_iter();
        let str_command : &str = iter_split.nth(0).unwrap();

        if str_command == "noop" {
            next_cycle(&mut value_x, &mut cycle, 1);
        } else if str_command == "addx" {
            let str_count : &str = iter_split.nth(0).unwrap();
            let count : i32 = str_count.parse().unwrap();

            next_cycle(&mut value_x, &mut cycle, 2);
            value_x += count;
        }
    }

    println!("Cycles: {}", cycle);

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}
