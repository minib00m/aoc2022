use std::env;
use std::process;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main()
{
    let argc = env::args().len();
    if argc != 2 {
        let argv0 = env::args().next().unwrap();
        println!("Usage: {} filename", argv0);
        process::exit(1);
    }
    let filename = env::args().nth(1).unwrap();
    let mut cur_sum = 0;
    let mut sizes: Vec<u32> = vec![];
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            match line {
                Ok(s) => match s.as_ref() {
                    "" => {
                        sizes.push(cur_sum);
                        cur_sum = 0;
                    },
                    x => cur_sum += x.parse::<u32>().unwrap()
                },
                Err(e) => println!("Error reading file: {}", e)
            }
        }
    }
    sizes.sort_by(|a, b| b.cmp(a));
    println!("Part 1 answer: {}", sizes[0]);
    println!("Part 2 answer: {}", sizes[0] + sizes[1] + sizes[2]);
}
