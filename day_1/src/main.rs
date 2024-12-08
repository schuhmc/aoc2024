use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::usize;

fn main() -> io::Result<()>{
    // read input
    let file = File::open("./input")?;
    let reader = BufReader::new(file);

    let mut columns: Vec<Vec<i32>> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?; // for error handling
        let values: Vec<&str> = line.split_whitespace().collect();

        // make the columns vector the correct size
        if i == 0 {
            columns = vec![Vec::new(); values.len()];
        }

        for (j, value) in values.iter().enumerate() {
            match value.parse::<i32>() {
                Ok(num) => columns[j].push(num),
                Err(e) => eprintln!("error parsing value '{}': {}", value, e),
            }
        }
    }

    // sort the values in the columns. This is done in-place
    for values in columns.iter_mut(){
        values.sort_unstable();
    }

    // evaluate the distance
    let mut sum: i32 = 0;
    for i in 0..columns[0].len(){
        let dis = (columns[0][i] - columns[1][i]).abs();
        sum += dis;
    }

    println!("The sum of the distances is: {}", sum);

    // Part 2
    // 1. find out how often each number in the left column (index 0)
    // occurs in the right column (index 1)
    // Multiply the value by the occurence
    // Sum it all up
    
    let mut sum: i32 = 0;
    let occurences: i32 = 0;
    let _prev_l: i32 = 0;
    let _prev_ri: usize = 0;
    for l in columns[0].iter(){

        if _prev_l == *l {
            sum += occurences * l;
            }
        else {
            // We count the occurence instead of adding up directly
            // in order to not have to double-count for the same ls
            let mut occurence: i32 = 0;
            for (ri, r) in columns[1][_prev_ri..columns[1].len()].iter().enumerate(){
                if r == l {
                    occurence += 1;
                    let _prev_ri: usize = ri; // keep track of where the last hit was
                }
                // the lists are already sorted.
                // If we are bigger, we can stop searching.
                else if r > l {
                    break;
                }
            }
            sum += l * occurence;

            let _prev_l: i32 = *l;
            }
    }

    println!("Similarity score: {}", sum);

    Ok(())
}
