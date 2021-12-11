// crates, which have to be specified in Cargo.toml unless they are common
use std::fs::File;
use std::io::{Error, BufReader, BufRead, ErrorKind};
use std::cmp::{min, max};
use std::collections::HashMap;
use itertools::Itertools;

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

// day 5 parsing coordinate pairs
fn parse_coordinate_pairs(readings: &[String]) -> Vec<Vec<i32>>{
    let coordinate_pairs = readings
        .iter()
        .map({
            |reading| reading
                .replace("->", ",")
                .split(',')
                .map({
                    |s| s
                        .trim()
                        .parse()
                        .unwrap()
                })
                .collect()

        })
        .collect();

    return coordinate_pairs
}

// day 5 part 1 and 2
fn get_pair_crossings(vectors: &Vec<Vec<i32>>) -> i32 {
    let max_dimension = vectors
    .iter()
    .flatten()
    .max()
    .unwrap() + 1;
    
    let mut vent_map = vec![vec![0; max_dimension as usize]; max_dimension as usize];

    for vector in vectors {
        // check if vector is vertical, horizontal, or diagonal
        if vector[0] == vector[2] {
            // vector is vertical
            let x = vector[0];
            for y in min(vector[1], vector[3])..(max(vector[1], vector[3]) + 1) {
                vent_map[y as usize][x as usize] += 1;
            }
        } else if vector[1] == vector[3]{
            // vector is horizontal
            let y = vector[1];
            for x in min(vector[0], vector[2])..(max(vector[0], vector[2]) + 1) {
                vent_map[y as usize][x as usize] += 1;
            }     
        } else {
            // vector is diagonal
            // need to check if going down left or down right
            let dy = vector[3] - vector[1];
            let dx = vector[2] - vector[0];
            let magnitude = dy.abs(); // since we are dealing with 45 degree vectors

            if dy.is_positive() == dx.is_positive() {
                let x = min(vector[0], vector[2]);
                let y = min(vector[1], vector[3]);
                for m in 0..(magnitude + 1) {
                    vent_map[(y + m) as usize][(x + m) as usize] += 1;
                }
            } else {
                let x = min(vector[0], vector[2]);
                let y = max(vector[1], vector[3]);

                for m in 0..(magnitude + 1) {
                    vent_map[(y - m) as usize][(x + m) as usize] += 1;
                }
            }
        }
    }

    let crossings = vent_map
        .iter()
        .flatten()
        .filter(|vent| **vent >= 2)
        .count();
    return crossings as i32
}

// day 6 part 1 and 2
fn get_fish_population_faster(fish_ages: &[String], days: i32) -> i64 {
    // get our starting fish
    let fish: Vec<i64> = fish_ages[0]
        .split(',')
        // i feel like i'm getting really cocky about error handling, oh well
        .map(|timer| timer.trim().parse().unwrap())
        .collect();

    // let's make an array that keeps track of how many fish are at what age, the
    // range of valid ages for fish are 0 to 8
    let mut fish_age_counts = [0; 9];

    // set the initial fish population age groups, not sure if i could do this in
    // an iter? can i modify another array like that then?
    for f in fish {
        fish_age_counts[f as usize] += 1;
    }


    for _ in 0..days {
        fish_age_counts.rotate_left(1);
        fish_age_counts[6] += fish_age_counts[8];
    }

    return fish_age_counts.iter().sum()
}

// day 7 i need a median function
fn median(numbers: &mut Vec<i32>) -> f64 {
    let length = numbers.len();
    numbers.sort_unstable();

    if length % 2 == 0 {
        let left = numbers[(length / 2) - 1];
        let right = numbers[length / 2];

        return (left as f64 + right as f64) / 2.0
    } else {
        return numbers[length / 2] as f64
    }
}

// day 7 part 1
fn get_crab_fuel_cost(crab_pos: &[String]) -> f64 {
    // get our positions
    let mut positions: Vec<i32> = crab_pos[0]
        .split(',')
        .map(|timer| timer.trim().parse().unwrap())
        .collect();

    let median_pos = median(&mut positions);

    let fuel_use: f64 = positions
        .iter_mut()
        .map(|s| ((*s as f64) - median_pos).abs())
        .sum();

    return fuel_use
}

// day 7 part 2
fn get_crab_fuel_cost_exp(crab_pos: &[String]) -> i32 {
    // get our positions
    let positions: Vec<i32> = crab_pos[0]
        .split(',')
        .map(|timer| timer.trim().parse().unwrap())
        .collect();

    // i don't think our median trick is going to work here but the mean position
    // should be a good starting point to do some eggregious gradient descent, it
    // is possible i'm just rounding wrong since i seem to always be off by one
    let mean_pos = ((positions.iter().sum::<i32>() as f64) / (positions.len() as f64)).round() as i32;

    let mut curr_minimum: i32 = positions
        .iter()
        // first time using copied, as far as i can see, this returns a shiny new
        // vector to do work on instead of modifying the original, useful
        .copied()
        .map(|s| (s - mean_pos).abs())
        .map(|s| (s*(s+1))/2)
        .sum();

    // okay, now we try to descend the curve
    let mut at_minima = false;
    let mut direction = 1;
    let mut direction_changes = 0;
    let mut offset = 1;

    // this is probably overkill, i'm sure i am rounding the mean a bit wrong
    while at_minima == false {
        let fuel_use: i32 = positions
            .iter()
            .copied()
            .map(|s| (s - (mean_pos + offset)).abs())
            .map(|s| (s*(s+1))/2)
            .sum();

        if fuel_use > curr_minimum {
            // change direction
            direction *= -1;
            direction_changes += 1;
        } else if fuel_use < curr_minimum {
            // going down the gradient, all is well
            curr_minimum = fuel_use;
        } // no need for else, if use is equal
        offset += direction;

        if direction_changes > 1 {
            // if we change direction more than twice we must be at a minima
            at_minima = true;
        }            
    }
    return curr_minimum
}

// day 8 part 1
fn get_sub_display_number_count(sub_num: &[String]) -> i32 {
    // get digits
    let output_values: Vec<String> = sub_num
        .iter()
        .map(|entry| entry.split('|').map(|sub_entry| sub_entry.to_string()).collect::<Vec<String>>())
        .map(|entry| entry[1].to_string())
        .collect();

    let mut count_1_4_8_7 = 0;

    for digit_group in output_values {
        for digit in digit_group.split_whitespace().map(|s| s.trim().to_string()).collect::<Vec<String>>() {
            match digit.len() {
                2 => count_1_4_8_7 += 1,
                3 => count_1_4_8_7 += 1,
                4 => count_1_4_8_7 += 1,
                7 => count_1_4_8_7 += 1,
                _ => ()

            }
        }
    }

    return count_1_4_8_7
}

// day 8 part 2
fn get_sub_display_number_values_and_sums(sub_num: &[String]) -> i32 {
    let mut final_output: i32 = 0;
    // get digits
    let all_digits: Vec<Vec<String>> = sub_num
        .iter()
        .map(|entry| entry
                        .replace("|", "")
                        .split_whitespace()
                        .map(|sub_entry| sub_entry
                                            .trim()
                                            .to_string())
                        .collect::<Vec<String>>())
        .collect();

    let output_digits: Vec<String> = sub_num
        .iter()
        .map(|entry| entry.split('|').map(|sub_entry| sub_entry.to_string()).collect::<Vec<String>>())
        .map(|entry| entry[1].to_string())
        .collect();

    let mut digit_maps = Vec::new();

    // have to get this starting list of values as they are used as comparitors
    for digit_group in &all_digits {
        let mut digit_map = HashMap::new();
        for digit in digit_group {
            match digit.len() {
                2 => {
                    if !digit_map.contains_key(&1) {
                        digit_map.insert(1, digit.chars().sorted().collect::<String>());
                    }
                },
                3 => {
                    if !digit_map.contains_key(&7) {
                        digit_map.insert(7, digit.chars().sorted().collect::<String>());
                    }
                },
                4 => {
                    if !digit_map.contains_key(&4) {
                        digit_map.insert(4, digit.chars().sorted().collect::<String>());
                    }
                },
                7 => {
                    if !digit_map.contains_key(&8) {
                        digit_map.insert(8, digit.chars().sorted().collect::<String>());
                    }
                },
                _ => ()

            }
        }
        digit_maps.push(digit_map);
    }

    // get all the other digits for each instance
    for (i, digit_map) in digit_maps.iter_mut().enumerate() {
        // get char for top bar of segement, char in 7 which is not in 1
        let one_map = digit_map[&1].to_string();
        let seven_map = digit_map[&7].to_string();
        let four_map = digit_map[&4].to_string();
        let mut top_char = '\0';

        for digit_char in seven_map.chars() {
            if !one_map.contains(digit_char) {
                top_char = digit_char;
                break;

            }
        }

        // next we can find all the rest!
        let mut false_nine_map = digit_map[&4].to_string() ;
        false_nine_map.push(top_char);

        for digit in &all_digits[i] {
            match digit.len() {
                6 => {
                    // check if all values for nine are there
                    if false_nine_map.chars().all(|c| digit.contains(c)) {
                        if !digit_map.contains_key(&9) {
                            digit_map.insert(9, digit.chars().sorted().collect::<String>());
                        }
                    // check if all values for 0 are there
                    } else if one_map.chars().all(|c| digit.contains(c)) {
                        if !digit_map.contains_key(&0) {
                            digit_map.insert(0, digit.chars().sorted().collect::<String>());
                        }
                    // otherwise must be 6
                    } else {
                        if !digit_map.contains_key(&6) {
                            digit_map.insert(6, digit.chars().sorted().collect::<String>());
                        }
                    }
                },
                5 => {
                    // check if all values for three are there
                    if one_map.chars().all(|c| digit.contains(c)) {
                        if !digit_map.contains_key(&3) {
                            digit_map.insert(3, digit.chars().sorted().collect::<String>());
                        }
                    // 5 will have three segments in common with 4, 2 only two
                    } else if four_map.chars().filter(|&c| digit.contains(c)).count() == 3 {
                        if !digit_map.contains_key(&5) {
                            digit_map.insert(5, digit.chars().sorted().collect::<String>());
                        }
                    // 2 is all that is left!
                    } else {
                        if !digit_map.contains_key(&2) {
                            digit_map.insert(2, digit.chars().sorted().collect::<String>());
                        }
                    }
                },
                _ => ()
            }
        }
        // we've filled out our hashmap for this line, now we need to reverse it
        let mut digit_map_reversed = HashMap::new();
        for (digit, strings) in digit_map {
            digit_map_reversed.insert(strings, *digit);
        }

        // get the current output
        let output: i32 = output_digits[i]
            .split_whitespace()
            .map(|s| s.trim().to_string().chars().sorted().collect::<String>())
            .map(|s| digit_map_reversed[&s].to_string())
            .collect::<String>()
            .parse()
            .unwrap();

        final_output += output;
    }
    return final_output
}

// day 9 part 1
fn get_seafloor_risk(seafloor_map: &[String]) -> i32 {
    let seafloor_width = seafloor_map[0].len();
    let mut risk_level = 0;

    let seafloor_array: Vec<i32> = seafloor_map
                                .iter()                                
                                .map(|s| s.chars().map(|c| c.to_string().parse().unwrap()).collect::<Vec<i32>>())
                                .flatten()
                                .collect();

    let total_length = seafloor_array.len();

    let check_index_offsets = [-1, 1, -(seafloor_width as i32), seafloor_width as i32];

    for (i, depth) in seafloor_array.iter().enumerate() {
        let check_indices: Vec<i32> = check_index_offsets
                                .iter()
                                .map(|index| (i as i32) + index)
                                // we don't want any indices before the start
                                .filter(|index| index >= &0)
                                // we don't want any indices after the end
                                .filter(|index| index < &(total_length as i32))
                                // if we are on the left edge, we don't want vals
                                // one to the left of the current index
                                .filter(|index| !((i % seafloor_width == 0) & (*index == i as i32 - 1)))
                                // if we are on the right edge, we don't want one
                                // to the right of the current index
                                .filter(|index| !((i % seafloor_width == seafloor_width - 1) & (*index == i as i32 + 1)))
                                .collect();

        let is_deepest = check_indices
                            .iter()
                            .all(|depth_index| depth < &seafloor_array[*depth_index as usize]);
        
        if is_deepest {
            risk_level += depth + 1;
        }
    }
    
    return risk_level
}

// day 9 function - recursion!
fn check_basin_neighbours(basin_index: &usize, seafloor_array: &Vec<i32>, seafloor_width: &usize, total_length: &usize) -> Vec<i32> {
    let check_index_offsets = [-1, 1, -(*seafloor_width as i32), *seafloor_width as i32];
    let check_indices: Vec<i32> = check_index_offsets
                                .iter()
                                .map(|index| (*basin_index as i32) + index)
                                // we don't want any indices before the start
                                .filter(|index| index >= &0)
                                // we don't want any indices after the end
                                .filter(|index| index < &(*total_length as i32))
                                // if we are on the left edge, we don't want vals
                                // one to the left of the current index
                                .filter(|index| !((basin_index % seafloor_width == 0) & (*index == *basin_index as i32 - 1)))
                                // if we are on the right edge, we don't want one
                                // to the right of the current index
                                .filter(|index| !((basin_index % seafloor_width == seafloor_width - 1) & (*index == *basin_index as i32 + 1)))
                                .collect();
    let mut basin_indexes: Vec<i32> = Vec::new();
    for check_index in check_indices {
        if seafloor_array[check_index as usize] == 9 {
            // basin does not include 9s
            continue
        } else {
            // we need to check the value and it's neighbours
            basin_indexes.push(check_index as i32);
            // if we don't do this we fall into infinite loops
            if seafloor_array[check_index as usize] > seafloor_array[*basin_index] {
                // it's happening!
                basin_indexes.extend(check_basin_neighbours(&(check_index as usize), seafloor_array, seafloor_width, total_length));
            }
        }
    }
    // we need to strip out duplicate indices, because it can happen
    return basin_indexes.into_iter().unique().collect();
}

// day 9 part 2
fn get_seafloor_basin_risk(seafloor_map: &[String]) -> i32 {
    let seafloor_width = seafloor_map[0].len();

    let seafloor_array: Vec<i32> = seafloor_map
                                .iter()                                
                                .map(|s| s.chars().map(|c| c.to_string().parse().unwrap()).collect::<Vec<i32>>())
                                .flatten()
                                .collect();

    let total_length = seafloor_array.len();

    let mut basin_sizes: Vec<i32> = Vec::new();

    let check_index_offsets = [-1, 1, -(seafloor_width as i32), seafloor_width as i32];

    

    for (i, depth) in seafloor_array.iter().enumerate() {
        let check_indices: Vec<i32> = check_index_offsets
                                .iter()
                                .map(|index| (i as i32) + index)
                                // we don't want any indices before the start
                                .filter(|index| index >= &0)
                                // we don't want any indices after the end
                                .filter(|index| index < &(total_length as i32))
                                // if we are on the left edge, we don't want vals
                                // one to the left of the current index
                                .filter(|index| !((i % seafloor_width == 0) & (*index == i as i32 - 1)))
                                // if we are on the right edge, we don't want one
                                // to the right of the current index
                                .filter(|index| !((i % seafloor_width == seafloor_width - 1) & (*index == i as i32 + 1)))
                                .collect();

        let is_deepest = check_indices
                            .iter()
                            .all(|depth_index| depth < &seafloor_array[*depth_index as usize]);

        if is_deepest {
            basin_sizes.push(check_basin_neighbours(&i, &seafloor_array, &seafloor_width, &total_length).iter().count() as i32);
        }
    }

    basin_sizes.sort();
    basin_sizes.reverse();
    
    return basin_sizes[..3].iter().product();
}

// day 10 part 1
fn get_error_score_parse_nav_chunks(nav_chunks: &[String]) -> i32 {
    let mut error_score = 0;
    let pair_map = HashMap::from([
        ('(',')'),
        ('[',']'),
        ('{','}'),
        ('<','>')
    ]);
    let error_score_map = HashMap::from([
        (')',3),
        (']',57),
        ('}',1197),
        ('>',25137)
    ]);
    for line in nav_chunks {
        let mut unmatched_chars: Vec<char> = Vec::new();

        for c in line.chars() {
            match c {
                '(' => unmatched_chars.push(c),
                '[' => unmatched_chars.push(c),
                '<' => unmatched_chars.push(c),
                '{' => unmatched_chars.push(c),
                _ => {
                    if c == pair_map[unmatched_chars.last().unwrap()] {
                        unmatched_chars.pop();
                    } else {
                        error_score += error_score_map[&c];
                        break;
                    }
                }
            }
        }
    }
    return error_score
}

// day 10 part 2
fn get_autocomplete_score_parse_nav_chunks(nav_chunks: &[String]) -> i64 {
    let mut autocomplete_scores: Vec<i64> = Vec::new();
    let pair_map = HashMap::from([
        ('(',')'),
        ('[',']'),
        ('{','}'),
        ('<','>')
    ]);
    let error_score_map = HashMap::from([
        ('(',1),
        ('[',2),
        ('{',3),
        ('<',4)
    ]);
    'outer: for line in nav_chunks {
        let mut autocomplete_score = 0;
        let mut unmatched_chars: Vec<char> = Vec::new();

        for c in line.chars() {
            match c {
                '(' => unmatched_chars.push(c),
                '[' => unmatched_chars.push(c),
                '<' => unmatched_chars.push(c),
                '{' => unmatched_chars.push(c),
                _ => {
                    if c == pair_map[unmatched_chars.last().unwrap()] {
                        unmatched_chars.pop();
                    } else {
                        continue 'outer;
                    }
                }
            }
        }

        // calculate syntax score
        unmatched_chars.reverse();
        unmatched_chars.iter().for_each(|c| autocomplete_score = (autocomplete_score * 5) + error_score_map[c]);
        autocomplete_scores.push(autocomplete_score);
    }

    // get the middle score value
    autocomplete_scores.sort();
    let num_scores = autocomplete_scores.len();
    let score_index = ((num_scores as f32) / 2.0).floor() as usize;
    return autocomplete_scores[score_index]
}

// day 11 part 1
fn get_squid_flashes(squid_energy: &[String], steps: usize) -> i64 {
    let mut flashes: i64 = 0;
    let squid_width = squid_energy[0].len();
    let mut squid_array: Vec<i32> = squid_energy.iter()                                
                                                .map(|s| s.chars().map(|c| c.to_string().parse().unwrap()).collect::<Vec<i32>>())
                                                .flatten()
                                                .collect();

    let squid_count = squid_array.len();

    let check_index_offsets = [-1, 1, -(squid_width as i32), squid_width as i32, -(squid_width as i32) + 1, squid_width as i32 + 1, -(squid_width as i32) - 1, squid_width as i32 - 1];

    for i in 0..steps {
        let mut squids_finished = false;

        // we do step 1
        squid_array.iter_mut().for_each(|s| *s += 1);

        // let's just naively try to loop through these then
        while squids_finished == false {
            for j in 0..squid_count {
                let check_indices: Vec<i32> = check_index_offsets.iter()
                                                                 .map(|index| (j as i32) + index)
                                                                 // we don't want any indices before the start
                                                                 .filter(|index| index >= &0)
                                                                 // we don't want any indices after the end
                                                                 .filter(|index| index < &(squid_count as i32))
                                                                 // if we are on the left edge, we don't want vals
                                                                 // one to the left of the current index
                                                                 .filter(|index| !(
                                                                     (j % squid_width == 0) & (*index == j as i32 - 1)
                                                                    ))
                                                                 .filter(|index| !(
                                                                     (j % squid_width == 0) & (*index == j as i32 - (squid_width + 1) as i32)
                                                                    ))
                                                                 .filter(|index| !(
                                                                     (j % squid_width == 0) & (*index == j as i32 + (squid_width - 1) as i32)
                                                                    ))
                                                                 // if we are on the right edge, we don't want any
                                                                 // to the right of the current index
                                                                 .filter(|index| !(
                                                                     (j % squid_width == squid_width - 1) & (*index == j as i32 + 1)
                                                                    ))
                                                                 .filter(|index| !(
                                                                     (j % squid_width == squid_width - 1) & (*index == j as i32 - (squid_width - 1) as i32)
                                                                    ))
                                                                 .filter(|index| !(
                                                                     (j % squid_width == squid_width - 1) & (*index == j as i32 + (squid_width + 1) as i32)
                                                                    ))
                                                                 .collect();
                
                if squid_array[j] > 9 {
                    // this is very hacky, i should really create some sort of data structure for the squids
                    squid_array[j] = -1000;
                    for check_index in check_indices {
                        squid_array[check_index as usize] += 1;
                    }
                }
            }

            squids_finished = squid_array.iter().all(|s| *s <= 9);

            if squids_finished {
                squid_array.iter_mut().filter(|s| **s < 0).for_each(|s| *s = 0);
                flashes += squid_array.iter().filter(|s| **s == 0).count() as i64;
                // println!("After Step {}:", i + 1);
                // for (j, squid) in squid_array.iter().enumerate() {
                //     print!("{}",squid);
                //     if j % squid_width == 9 {
                //         print!("\n");
                //     }
                // }                
            }            
        }
    }
    return flashes
}

// day 11 part 2
fn get_squid_steps_until_sync_flashes(squid_energy: &[String]) -> i64 {
    let mut flashes: i64 = 0;
    let squid_width = squid_energy[0].len();
    let mut squid_array: Vec<i32> = squid_energy.iter()                                
                                                .map(|s| s.chars().map(|c| c.to_string().parse().unwrap()).collect::<Vec<i32>>())
                                                .flatten()
                                                .collect();

    let squid_count = squid_array.len();

    let check_index_offsets = [-1, 1, -(squid_width as i32), squid_width as i32, -(squid_width as i32) + 1, squid_width as i32 + 1, -(squid_width as i32) - 1, squid_width as i32 - 1];
    let mut sync_flashes = false;
    let mut num_steps = 0;

    while sync_flashes == false {
        let mut squids_finished = false;
        num_steps += 1;

        // we do step 1
        squid_array.iter_mut().for_each(|s| *s += 1);

        // let's just naively try to loop through these then
        while squids_finished == false {
            for j in 0..squid_count {
                let check_indices: Vec<i32> = check_index_offsets.iter()
                                                                 .map(|index| (j as i32) + index)
                                                                 // we don't want any indices before the start
                                                                 .filter(|index| index >= &0)
                                                                 // we don't want any indices after the end
                                                                 .filter(|index| index < &(squid_count as i32))
                                                                 // if we are on the left edge, we don't want vals
                                                                 // one to the left of the current index
                                                                 .filter(|index| !(
                                                                     (j % squid_width == 0) & (*index == j as i32 - 1)
                                                                    ))
                                                                 .filter(|index| !(
                                                                     (j % squid_width == 0) & (*index == j as i32 - (squid_width + 1) as i32)
                                                                    ))
                                                                 .filter(|index| !(
                                                                     (j % squid_width == 0) & (*index == j as i32 + (squid_width - 1) as i32)
                                                                    ))
                                                                 // if we are on the right edge, we don't want any
                                                                 // to the right of the current index
                                                                 .filter(|index| !(
                                                                     (j % squid_width == squid_width - 1) & (*index == j as i32 + 1)
                                                                    ))
                                                                 .filter(|index| !(
                                                                     (j % squid_width == squid_width - 1) & (*index == j as i32 - (squid_width - 1) as i32)
                                                                    ))
                                                                 .filter(|index| !(
                                                                     (j % squid_width == squid_width - 1) & (*index == j as i32 + (squid_width + 1) as i32)
                                                                    ))
                                                                 .collect();
                
                if squid_array[j] > 9 {
                    // this is very hacky, i should really create some sort of data structure for the squids
                    squid_array[j] = -1000;
                    for check_index in check_indices {
                        squid_array[check_index as usize] += 1;
                    }
                }
            }

            squids_finished = squid_array.iter().all(|s| *s <= 9);

            if squids_finished {
                squid_array.iter_mut().filter(|s| **s < 0).for_each(|s| *s = 0);
                flashes += squid_array.iter().filter(|s| **s == 0).count() as i64;
                // println!("After Step {}:", i + 1);
                // for (j, squid) in squid_array.iter().enumerate() {
                //     print!("{}",squid);
                //     if j % squid_width == 9 {
                //         print!("\n");
                //     }
                // }                
            }            
        }

        // lets check if all synced
        sync_flashes = squid_array.iter().all(|s| *s == 0);

    }
    return num_steps
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

    // day 2 start
    println!("Advent of Code 2021 Day 2");
    let path = "./data/day2.txt";
    let readings = read_txt_pairs(path).expect("Something went wrong with my file parsing");
    let first_calc = get_depth_distance_multiple(&readings);
    println!("Multiple of final depth and position is {}", first_calc);
    let second_calc = get_depth_distance_aim_multiple(&readings);
    println!("Multiple of aimed depth and position is {}", second_calc);

    // day 3 start
    println!("Advent of Code 2021 Day 3");
    let path = "./data/day3.txt";    
    let readings = read_txt_strings(path).expect("Something went wrong with my file parsing");
    let (gamma, epsilon) = get_gamma_and_epsilon(&readings);
    println!("Multiple of gamma and epsilon are {}", gamma * epsilon);
    let (o2, co2) = get_o2_co2(&readings);
    println!("Multiple of o2 and co2 are {}", o2 * co2);

    // day 4 start
    println!("Advent of Code 2021 Day 4");
    let path = "./data/day4.txt";
    let readings = read_txt_strings(path).expect("Something went wrong with my file parsing");
    let bingo_score = get_bingo_score(&readings);
    println!("Bingo score is {}", bingo_score);
    let last_bingo_score = get_bingo_score_last(&readings);
    println!("Last winning bingo score is {}", last_bingo_score);

    // day 5 start
    println!("Advent of Code 2021 Day 5");
    let path = "./data/day5.txt";
    let readings = read_txt_strings(&path).expect("Something went wrong reading input data");
    let coordinate_pairs = parse_coordinate_pairs(&readings);

    // for part one only look at the horizontal + vertical vectors
    let reduced_coordinate_pairs = coordinate_pairs
        .iter()
        .filter(|pair| (pair[0] == pair[2]) | (pair[1] == pair[3]))
        .map(|pair| pair.to_vec())
        .collect::<Vec<Vec<i32>>>();

    let crossings = get_pair_crossings(&reduced_coordinate_pairs);
    println!("There are {} h+v vent crossings", crossings);
    let crossings = get_pair_crossings(&coordinate_pairs);
    println!("There are {} h+v+d vent crossings", crossings);

    // day 6 start
    println!("Advent of Code 2021 Day 6");
    let path = "./data/day6.txt";
    let fish_times = read_txt_strings(&path).expect("Something went wrong reading input data");
    let num_fish = get_fish_population_faster(&fish_times, 80);
    println!("There are {} fish after 80 days", num_fish);
    let num_fish = get_fish_population_faster(&fish_times, 256);
    println!("There are {} fish after 256 days", num_fish);

    // day 7 start
    println!("Advent of Code 2021 Day 7");
    let path = "./data/day7.txt";
    let crab_pos = read_txt_strings(&path).expect("Something went wrong reading input data");
    let crab_fuel_cost = get_crab_fuel_cost(&crab_pos);
    println!("Crab fuel costs are {}", crab_fuel_cost);
    let crab_fuel_cost = get_crab_fuel_cost_exp(&crab_pos);
    println!("Crab exponential fuel costs are {}", crab_fuel_cost);

    // day 8 start
    println!("Advent of Code 2021 Day 8");
    let path = "./data/day8.txt";
    let sub_num = read_txt_strings(&path).expect("Something went wrong reading input data");
    let sub_digit_count = get_sub_display_number_count(&sub_num);
    println!("Number of 1, 4, 7, 8 digits are {}", sub_digit_count);
    let sub_number_sum = get_sub_display_number_values_and_sums(&sub_num);
    println!("Sum of outputs are {}", sub_number_sum);

    // day 9 start
    println!("Advent of Code 2021 Day 9");
    let path = "./data/day9.txt";
    let seafloor_map = read_txt_strings(&path).expect("Something went wrong reading input data");
    let seafloor_risk = get_seafloor_risk(&seafloor_map);
    println!("Sum of seafloor risk is {}", seafloor_risk);
    let basin_risk = get_seafloor_basin_risk(&seafloor_map);
    println!("Product of biggest three basins are {}", basin_risk);

    // day 10 start
    println!("Advent of Code 2021 Day 10");
    let path = "./data/day10.txt";
    let nav_chunks = read_txt_strings(&path).expect("Something went wrong reading input data");
    let syntax_error_score = get_error_score_parse_nav_chunks(&nav_chunks);
    println!("Nav syntax error score is {}", syntax_error_score);
    let autocomplete_score = get_autocomplete_score_parse_nav_chunks(&nav_chunks);
    println!("Middle autocomplete score is {}", autocomplete_score);

    // day 11 start
    println!("Advent of Code 2021 Day 11");
    let path = "./data/day11.txt";
    let squid_energy = read_txt_strings(&path).expect("Something went wrong reading input data");
    let squid_flashes = get_squid_flashes(&squid_energy, 100);
    println!("There are {} flashes after 100 steps", squid_flashes);
    let steps_to_sync = get_squid_steps_until_sync_flashes(&squid_energy);
    println!("Squid flashes syncronise after {} steps", steps_to_sync);
}