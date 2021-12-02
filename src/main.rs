// crates, which have to be specified in Cargo.toml unless they are common
use std::fs::File;
use std::io::{Error, BufReader, BufRead, ErrorKind};
use std::collections::HashMap;

// okay, let's create a function, we are passing a path as an immutable reference 
// the function will return a result (for how rust handles errors) which contains 
// an array, more specifically a vector which is a dynamically allocated array
fn read_txt_ints(path: &str) -> Result<Vec<i32>, Error> {
    // righto, let's create a file object, seems easy, the '?' operator functions
    // as shorthand for how errors are handled in rust, exact functionality isn't
    // clear to me when it comes to handling errors (or the lack thereof) in rust
    // but for now the '?' operator is used wherever i expect potential errors
    let file = File::open(path)?;

    // next we create a buffered reader from the file handle though seems baroque
    // here it does have a nice lines struct to read data from our input file
    let br = BufReader::new(file);

    // let's create the vector in which we are going to store the data and ensure
    // it is mutable because by default rust says no, make once assign once!
    let mut readings = Vec::new();

    // oh boy, just like python!
    for line in br.lines() {
        // define our line from the result, because io is always a risk for errors
        // we do our '?' operator again to handle it, or not handle idk
        let line = line?;

        // seems like a rust thing to do these "multiline" operations, i'm liking
        // it for readability, but i am going to be peppering python with ';' for
        // weeks after this. 
        let num = line
            // get rid of any whitespaces
            .trim()
            // parse below seems to automagically take the datatype from function
            // definition and backcalcs it at compile time to whatever types from
            // string, in this case i32, i hope
            .parse()
            // of course, parse can return an error when dealing with lines which
            // do not have valid ints, first mystery to me is the '|e|' syntax in
            // the function which is something called a closure, aking to lambdas
            // in python but more powerful in that it has acccess to variables in
            // the scope which it is definited in, this may result in Fun™ later.
            // in this case we convert the parsing error to an io one
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        // now that we have a beautiful integer, we add it to the vector
        readings.push(num);
    }
    // unline normal return a rust function returns both a result and potentially
    // an error, how we return a success value is as follows
    Ok(readings)
}

// above function takes heavily from some online examples, setting my comments to
// -vvvvv has helped hugely with feeling a semblance of understanding of how rust
// deals with things, next function is all homegrown babey! i will not be worried
// about using results just yet for these functions because i think it's unlikely
// that these will error out and i don't understand the error types well enough
fn get_sum_positive_diffs(readings: &[i32], window_length: i32) -> i32 {
    // this will be our return int
    let mut final_sum = 0;

    // we can jump straight into a loop after making the values iterable, also we
    // could reference a slice offset of the vector and zip those up, but this is
    // nice and simple feeling
    let iter_readings = readings.iter();    
    for (i, val) in iter_readings.enumerate() {
        // this as type syntax seems to be quite common in rust, with usize being
        // an intlike specifically common from iterators for indexing
        if i < window_length as usize {
            continue;
        }

        // the problem simplifies to only the first and last value from each diff
        // group for the final sum
        let minus_index = i - window_length as usize;
        let diff = val - readings[minus_index];
        if diff > 0 {
            final_sum += 1;
        }
    }

    // when not definig a result return you can just use traditional return
    return final_sum;
}

// okay so for day 2 we need somewhat different file parsing, this time one using
// a different kind of vector return which will be tuple pairs
fn read_txt_pairs(path: &str) -> Result<Vec<(String, i32)>, Error> {
    // we've been down this path before, though it's clearer now that '?' handles
    // passing the error back up to the scope of the function for handling later
    let file = File::open(path)?;
    let br = BufReader::new(file);

    // this is the vector we will be returning
    let mut pair_vector = Vec::new();

    for line in br.lines() {
        let line = line?;

        let line_pair = line
            // we want to split on the white space, this gives us an iterator for
            // use later, and my later i mean right now 
            .split(' ');

        // i'm sure there is a nice way to do this but let's take the iterator to
        // a vector so i can use indexing on it
        let mut line_pair_array = Vec::new();
        for pair in line_pair {
            line_pair_array.push(pair);
        }

        // get the direction string, i don't think we need to handle results here
        let direction: String = line_pair_array[0]
            // whitespace begone
            .trim()
            // this create a new string instance, which prevents an error in rust
            // later related to passing the vector this string ends up in outside
            // of this function. i still feel very much guided by the errors from
            // rust compilation rather than intuiting these allocations
            .to_string();

        // get the magnitude string and take it to int, which we 100% must handle
        // the result of as we've done in our day 1 file read function
        let magnitude: i32 = line_pair_array[1]
            // whitespace, in my inputs!?
            .trim()
            // more magic casting
            .parse()
            // it's our friend the error map, see day 1
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        // we have our vals, so let's make the pair and add it to the pair vector
        pair_vector.push((direction, magnitude));
    }
    // so if i want to return an explicit error i can call Err(), something which
    // i haven't noted is the lack of a ';', this essentially ensures that i have
    // gauranteed return function from here. add a comma, see rust cry
    Ok(pair_vector)
}

// day 2 part 1, set comment verbosity to -v
fn get_depth_distance_multiple(readings: &[(String, i32)]) -> i32 {
    // intermediate values
    let mut horizontal_pos = 0;
    let mut depth = 0;

    // loops, my precious loops
    for val in readings {
        // PATTERN MATCHING YESSSS
        match val.0.as_str() {
            "forward" => horizontal_pos += val.1,
            "up" => depth -= val.1,
            "down" => depth += val.1,
            _ => println!("Unknown value in data")
        }
    }
    return horizontal_pos * depth;
}

// day 2 part 2, my initial answer was uncomfortably close to the limits of int32
// so let's use int64 just for fun
fn get_depth_distance_aim_multiple(readings: &[(String, i32)]) -> i64 {
    // intermdiate values, you can explicitly declare types like this, though not
    // sure how it decides by default between i32 and i64, seems like from common
    // usage in rest of code
    let mut horizontal_pos: i64 = 0;
    let mut depth: i64 = 0;
    let mut aim: i64 = 0;

    // lots of copy paste from part 1
    for val in readings {
        // have i mentioned i like pattern matching?
        match val.0.as_str() {
            "forward" => {
                horizontal_pos += val.1 as i64;
                depth += aim * val.1 as i64;
            },
            "up" => aim -= val.1 as i64,
            "down" => aim += val.1 as i64,
            _ => println!("Unknown value in data")
        }
    }
    return horizontal_pos * depth;
}

// like a lot of other languages rust starts execution from main()
fn main() {
    // hello
    println!("Advent of Code 2021 Day 1");

    // day 1 start
    let path = "./data/day1.txt";
    let readings = read_txt_ints(path).expect("Something went wrong with my file parsing");    

    let first_sum = get_sum_positive_diffs(&readings, 1);
    println!("First sum is {}", first_sum);

    let second_sum = get_sum_positive_diffs(&readings, 3);
    println!("Second sum is {}", second_sum);
    // day 1 end

    // day 2 start
    println!("Advent of Code 2021 Day 2");
    let path = "./data/day2.txt";
    let readings = read_txt_pairs(path).expect("Something went wrong with my file parsing");

    let first_calc = get_depth_distance_multiple(&readings);
    println!("Multiple of final depth and position is {}", first_calc);

    let second_calc = get_depth_distance_aim_multiple(&readings);
    println!("Multiple of aimed depth and position is {}", second_calc)
    // day 2 stop

    // day 3 start
}