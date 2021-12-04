// crates, which have to be specified in Cargo.toml unless they are common
use std::fs::File;
use std::io::{Error, BufReader, BufRead, ErrorKind};

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
        // define our line from the result because io is always a risk for errors
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

// i can probably use this as a base function and then call it with a wrapper for
// any other input manipulation i need to do
fn read_txt_strings(path: &str) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);

    let mut readings = Vec::new();

    for line in br.lines() {
        let line = line?;

        let num = line
            // get rid of any whitespaces
            .trim()
            .to_string();

        // now that we have a beautiful integer, we add it to the vector
        readings.push(num);
    }
    // unline normal return a rust function returns both a result and potentially
    // an error, how we return a success value is as follows
    Ok(readings)
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

// day 3 part 1
fn get_gamma_and_epsilon(readings: &[String]) -> (i32, i32) {
    // this is going to need binary operations, honestly i would rather user some
    // assembly. also need to convert the binary encased in ints to actual values
    let num_readings = readings.len() as i32;
    let bit_lenght = readings[0].len();
    let mut bit_sums = Vec::new(); // i could probably make an array here

    let readings = readings.iter();

    for (i, reading) in readings.enumerate() {
        let int_value = isize::from_str_radix(reading, 2).unwrap();

        // wow this is hideous, i am sure there must be a better way with bitwise
        // operators, i could just as well iterate over the string here
        for j in 0..bit_lenght {
            if i == 0 {
                bit_sums.push((int_value >> j & 1) as i32);
            } else {
                bit_sums[j] += (int_value >> j & 1) as i32;
            }
        }
        
    }

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    let bit_sums = bit_sums.iter();
    for (i, bit_sum) in bit_sums.enumerate() {
        // if 1 was the most common bit
        if bit_sum > &(num_readings / 2) {
            gamma_rate += (2 as i32).pow(i as u32);
        } else {
            epsilon_rate += (2 as i32).pow(i as u32);
        }

    }
    return (gamma_rate, epsilon_rate)
}

// day 3 part 2
fn get_o2_co2(readings: &[String]) -> (i32, i32) {
    // same stuff from previous function, but i feel recursion coming in my bones
    // we are now going to use the counts to find oxygen numbers, does rust allow
    // nested functions? yes

    // so this is going to return a new list... 
    fn get_new_list(passed_readings: &[String], passed_bit_sums: &[i32], bit_index: i32) -> (Vec<String>, Vec<String>) {
        let mut o2_number_list = Vec::new();
        let mut co2_number_list = Vec::new();
        let num_passed_readings = passed_readings.len();
        let mut most_common_bit_high = false;
        let mut most_common_bit_equal = false;

        if passed_bit_sums[bit_index as usize] as f32 > (num_passed_readings as f32 / 2.0) {
            most_common_bit_high = true;
        // i've probably maodified this if statement a billion times, dang types
        } else if (passed_bit_sums[bit_index as usize] as f32) == (num_passed_readings as f32 / 2.0) {
            most_common_bit_equal = true;
        }
        for passed_reading in passed_readings {
            let int_value = isize::from_str_radix(passed_reading, 2).unwrap();
            let bit_of_interest_high = (int_value >> bit_index & 1) == 1;

            if most_common_bit_equal {
                if bit_of_interest_high {
                    o2_number_list.push(passed_reading.to_string());
                } else {
                    co2_number_list.push(passed_reading.to_string());
                }
            } else {
                // i'm mildly please by this if statement
                if bit_of_interest_high == most_common_bit_high {
                    o2_number_list.push(passed_reading.to_string());
                } else {
                    co2_number_list.push(passed_reading.to_string());
                }
            }
            
            
        }
        return (o2_number_list, co2_number_list)
    }
        

    // we also need a function for the most common at each bit
    fn get_new_bit_sums(passed_readings: &[String]) -> Vec<i32> {
        let bit_lenght = passed_readings[0].len();
        let mut bit_sums = Vec::new(); // i could probably make an array here

        let passed_readings = passed_readings.iter();

        for (i, passed_reading) in passed_readings.enumerate() {
            let int_value = isize::from_str_radix(passed_reading, 2).unwrap();

            // wow this is hideous, i am sure there must be a better way with bitwise
            // operators, i could just as well iterate over the string here
            for j in 0..bit_lenght {
                if i == 0 {
                    bit_sums.push((int_value >> j & 1) as i32);
                } else {
                    bit_sums[j] += (int_value >> j & 1) as i32;
                }
            }            
        }
        // bit_sums are from least to most significant digit
        return bit_sums
    }

    let starting_bit_sums = get_new_bit_sums(readings);
    let bit_length = readings[0].len();

    let mut o2_number_list = Vec::new();
    let mut co2_number_list = Vec::new();
    let mut o2_bit_sums = Vec::new();
    let mut co2_bit_sums = Vec::new();
    for i in 0..starting_bit_sums.len() {
        let bit_index = bit_length - i - 1;
        if i == 0 {
            let (temp_o2, temp_co2) = get_new_list(readings, &starting_bit_sums, bit_index as i32);
            o2_number_list = temp_o2;
            co2_number_list = temp_co2;
            o2_bit_sums = get_new_bit_sums(&o2_number_list);
            co2_bit_sums = get_new_bit_sums(&co2_number_list);
        } else {
            if o2_number_list.len() > 1 {
                let (temp_o2, _temp_co2) = get_new_list(&o2_number_list, &o2_bit_sums, bit_index as i32);
                o2_number_list = temp_o2;
                o2_bit_sums = get_new_bit_sums(&o2_number_list);
            }
            if co2_number_list.len() > 1 {
                let (_temp_o2, temp_co2) = get_new_list(&co2_number_list, &co2_bit_sums, bit_index as i32);
                co2_number_list = temp_co2;
                co2_bit_sums = get_new_bit_sums(&co2_number_list);
            }
        }
    }
    let o2 = isize::from_str_radix(&o2_number_list[0], 2).unwrap() as i32;
    let co2 = isize::from_str_radix(&co2_number_list[0], 2).unwrap() as i32;

    // i guess recursion didn't happen
    return (o2, co2)
}

// day 4 let's make a bingo scoring function
fn score_board(board: &[[i32; 5]; 5]) -> (bool, i32) {
    let mut col_completed = false;

    // making a function like this is a real "i know kung fu moment"
    let row_completed = board
        .iter()
        // i wonder if there is a nice way if having this nested iter
        .map(|row| row.iter().all(|&num| num == -1))
        .any(|row| row);

    // doing for the columns is not yet within my reach though, i am sure i could
    // transpose it somewhat, but i'm happy enought with this as is
    for i in 0..5 {
        // we need to break out of this loop in case we accidentally set false on
        // a prior true
        if col_completed {
            break;
        }

        col_completed = board
            .iter()
            .map(|row| row[i])
            .all(|num| num == -1);
    }

    // now if we have completed rows or cols let's get the score back
    if row_completed | col_completed {
        let score_sum: i32 = board
                .iter()
                .flatten()
                .filter(|number| number.is_positive())
                .sum();
        return (true, score_sum)
    } else {
        return (false, 0)
    }
}

// day 4 let's make a function to parse the bingo boards and numbers
fn get_bingo_boards(readings: &[String]) -> (Vec<i32>, Vec<[[i32; 5]; 5]>) {
    // okay so readings are unparsed in the strings, first line are bingo numbers
    // which will be read out in groups of 5 to be evaluated on the bingo boards
    let bingo_nums: Vec<i32> = readings[0]
        .split(',')
        .map(|s| s.parse().expect("Something went wrong parsing the bingo_nums"))
        .collect();

        let mut bingo_boards = Vec::new();
        let mut current_board = [[0; 5]; 5];
        let mut board_row = 0;
    
    // next we parse the bingo boards into a vector, right now this is assuming a
    // fixed bingo board of 5x5 and then storing it in a 5x5 2d array, storing it
    // as a straight vector 25 in length (or arbitrary square number) would probs
    // reduce looping and make some of the expressions easier, but it's very nice
    // to look at the array in the debugger and actually see your bingo board
    for (i, reading) in readings.iter().enumerate() {        
        // skip the bingo readings and first whitespace
        if i < 2 {
            continue;
        }

        // pattern matching ❤
        match reading.as_str() {
            "" => {
                // blank line, store and clear the bingo board
                bingo_boards.push(current_board);
                current_board = [[0; 5]; 5];
                board_row = 0;
            }
            _ => {
                // any other case, time to parse lines!
                for (j, number) in reading.split_whitespace().enumerate() {
                    current_board[board_row][j] = number
                        .trim()
                        .parse()
                        .expect("Something went wrong parsing the number in row");
                }
                board_row += 1;
            }
        }
    }    

    // last board is still resident in current_board, so push to the array before
    // returning the results
    bingo_boards.push(current_board);
    return(bingo_nums, bingo_boards)
}

// day 4 part 1
fn get_bingo_score(readings: &[String]) -> i32 {
    // let's parse the bingo board and number data
    let (bingo_nums, mut bingo_boards) = get_bingo_boards(readings);    

    // we have our boards, now we need to do scoring and i don't how to not use a
    // loop here again, i've been vaguely told that 
    for bingo_num in bingo_nums {
        for board in bingo_boards.iter_mut() {
            // nice
            board.iter_mut()
                // now this is nice for dealing with multidimensional arrays
                .flatten()
                // just keep looking at values that pass this condition
                .filter(|num| **num == bingo_num)
                // assinging -1 where bingo_num is found, something that does not
                // occur naturally in the bingo boards and still is an int
                .for_each(|num| *num = -1);

            // check if we have any winning hands
            let (victory, score) = score_board(board);
            if victory {
                return score * bingo_num
            }
        }
    }
    // better would be to use Ok() above and Err() here since i should always get
    // some score, unless the puzzle was not solved
    return 0
}

// day 4 part 2
fn get_bingo_score_last(readings: &[String]) -> i32 {
    // let's parse the bingo board and number data
    let (bingo_nums, mut bingo_boards) = get_bingo_boards(readings);

    // we have our boards, now we need to do scoring and i don't see not to use a
    // bunch of loops again
    let number_of_boards = bingo_boards.len();
    let mut win_vector = vec![0 as usize; number_of_boards];
    for bingo_num in bingo_nums {
        for (i, board) in bingo_boards.iter_mut().enumerate() {
            // nice again
            board.iter_mut()
                .flatten()
                .filter(|num| **num == bingo_num)
                .for_each(|num| *num = -1);

            // again we check for success, but in this case we're looking for the
            // last winning board
            let (victory, score) = score_board(board);
            if victory {
                // we need to keep track of which of the boards have been won, so 
                // we no longer count them as further wins in the loop
                win_vector[i] = 1;
                let won_board_count: usize = win_vector.iter().sum();
                // technically, this will fail if end up with boards that are not
                // winning after the input number are exhausted, but i think that
                // would not be case here and would make things a lot harder, but
                // recursion could be the answer then ;)
                if number_of_boards - won_board_count == 0 {
                    return score * bingo_num
                }
            }
        }
    }
    return 0
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
    println!("Multiple of aimed depth and position is {}", second_calc);
    // day 2 stop

    // day 3 start
    println!("Advent of Code 2021 Day 3");
    let path = "./data/day3.txt";    
    let readings = read_txt_strings(path).expect("Something went wrong with my file parsing");

    let (gamma, epsilon) = get_gamma_and_epsilon(&readings);
    println!("Multiple of gamma and epsilon are {}", gamma * epsilon);

    let (o2, co2) = get_o2_co2(&readings);
    println!("Multiple of o2 and co2 are {}", o2 * co2);
    // day 3 stop

    // day 4 start
    println!("Advent of Code 2021 Day 4");
    let path = "./data/day4.txt";
    let readings = read_txt_strings(path).expect("Something went wrong with my file parsing");

    let bingo_score = get_bingo_score(&readings);
    println!("Bingo score is {}", bingo_score);

    let last_bingo_score = get_bingo_score_last(&readings);
    println!("Last winning bingo score is {}", last_bingo_score);

}