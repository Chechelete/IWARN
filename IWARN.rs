
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read};

fn call_read(global: &mut Vec<Vec<i64>>, current_pos: &mut i64, saved: &mut i64, contents: &Vec<char>, index:&mut usize,buffer:&mut String,direct:&mut bool,array_index:&mut u32) {
    let mut start_index = *index + 5; // Start searching after the current 'L'
    let mut end_index = start_index;

    while end_index < contents.len()-1 && contents[end_index] != ']' {
        end_index += 1;
    }
    while start_index <= end_index {
        read(global, current_pos, saved, contents, &mut start_index,buffer,direct,array_index);
        start_index += 1;
    }
    *index = end_index+1;
}

fn read(global: &mut Vec<Vec<i64>>, current_pos: &mut i64, saved: &mut i64, contents: &Vec<char>, index:&mut usize,buffer:&mut String,direct:&mut bool,array_index:&mut u32) -> usize {
    let letter = contents[*index];
    let mut total_size:usize = global.len();
    let transformed = *array_index as usize;
    let mut size:usize = global[transformed].len();
    match letter {
        '+' => global[transformed][*current_pos as usize] += 1,
        '-' => global[transformed][*current_pos as usize] -= 1,
        '*' => global[transformed][*current_pos as usize] *= 2,
        '/' => global[transformed][*current_pos as usize] /= 2,
        '>' => {
            (*current_pos)+=1;
            let new_pos = *current_pos as usize;

            if new_pos >= size - 1 {
                global[transformed].resize(new_pos + 1, 0);
            }
        }
        '<' => {
            if (*current_pos) == 0 {
                println!("Cant go to a negative square");
                (*current_pos) = 0;
            }
            else { (*current_pos) = *current_pos - 1; }
        }
        '?' => println!("Vector {} index {}",*array_index, *current_pos),
        '^' => *saved = global[transformed][*current_pos as usize],
        '=' => global[transformed][*current_pos as usize] = *saved,
        '!' => println!("Saved {}", *saved),
        ')' => {
            global[transformed].push(0);
            size+=1;
        }
        '(' => {
            // Check if the vector is not empty before removing the last element
            if let Some(_) = global[transformed].pop() {
                if size >= 2 { size-=1; } else { println!("Cannot pull, vector too small"); }
            } else {
                println!("Vector is empty cant pull");
            }
        }
        ';' => {
            println!("𝔹 {}",buffer);
        }
        '@' => println!("Value {}", global[transformed][*current_pos as usize]),
        '%' => if (*saved) > 1 && (*saved) < (size as i64) - 1 { (*current_pos) = *saved; },
        '#' => global[transformed][*current_pos as usize] += *saved,
        '$' => global[transformed][*current_pos as usize] -= *saved,
        'e' => global[transformed][*current_pos as usize] *= *saved,
        'r' => global[transformed][*current_pos as usize] /= *saved,
        '.' => println!("Vector {} size {}", *array_index,size),
        't' => println!("Global size is {}",total_size),
        '~' => {
            let start_index = *index + 1; // Start searching after the current 'L'
            let mut end_index = start_index;

            while end_index < contents.len() && contents[end_index] != '~' {
                end_index += 1;
            }
            let substring: String = contents[start_index..end_index].iter().collect();
            *buffer = substring;
            *index = end_index;
        }
        '[' => {
            let start_index = *index + 1; // Start searching after the current 'L'
            let mut end_index = start_index;

            while end_index < contents.len()-1 && contents[end_index] != ']' {
                end_index += 1;
            }

            // If 'L' is found, create the substring and process the content between 'L' and 'L'
            let _substring: String = contents[start_index..end_index].iter().collect();
            // Repeat the content between 'L' and 'L' *saved times
            for _ in 0..*saved {
                let mut inner_index = start_index;
                while inner_index <= end_index {
                    read(global, current_pos, saved, contents, &mut inner_index,buffer,direct,array_index);
                    inner_index += 1;
                }
            }
            *index = end_index+1;
        }
        'a' | 's' | 'd' | 'f' | 'g' | 'h' => {
            if (*index) + 4 > contents.len() {
                println!("Invalid input to if statement, check you putted an operator and number to be compared separated by spaces");
            } else {
                let operator = contents[*index + 2];
                let number = contents[*index + 4];
                let condition_met = match contents[*index] {
                    'a' => match operator {
                        '=' => global[transformed][*current_pos as usize] == number.to_digit(10).unwrap() as i64,
                        '<' => global[transformed][*current_pos as usize] < number.to_digit(10).unwrap() as i64,
                        '>' => global[transformed][*current_pos as usize] > number.to_digit(10).unwrap() as i64,
                        '!' => global[transformed][*current_pos as usize] != number.to_digit(10).unwrap() as i64,
                        _ => false,
                    },
                    's' => match operator {
                        '=' => *saved == number.to_digit(10).unwrap() as i64,
                        '<' => *saved < number.to_digit(10).unwrap() as i64,
                        '>' => *saved > number.to_digit(10).unwrap() as i64,
                        '!' => *saved != number.to_digit(10).unwrap() as i64,
                        _ => false,
                    },
                    'd' => match operator {
                        '=' => *current_pos == number.to_digit(10).unwrap() as i64,
                        '<' => *current_pos < number.to_digit(10).unwrap() as i64,
                        '>' => *current_pos > number.to_digit(10).unwrap() as i64,
                        '!' => *current_pos != number.to_digit(10).unwrap() as i64,
                        _ => false,
                    },
                    'f' => match operator {
                        '=' => *array_index == number.to_digit(10).unwrap() as u32,
                        '<' => *array_index < number.to_digit(10).unwrap() as u32,
                        '>' => *array_index > number.to_digit(10).unwrap() as u32,
                        '!' => *array_index != number.to_digit(10).unwrap() as u32,
                        _ => false,
                    },
                    'g' => match operator {
                        '=' => size == number.to_digit(10).unwrap() as usize,
                        '<' => size < number.to_digit(10).unwrap() as usize,
                        '>' => size > number.to_digit(10).unwrap() as usize,
                        '!' => size != number.to_digit(10).unwrap() as usize,
                        _ => false,
                    },
                    'h' => match operator {
                        '=' => total_size == number.to_digit(10).unwrap() as usize,
                        '<' => total_size < number.to_digit(10).unwrap() as usize,
                        '>' => total_size > number.to_digit(10).unwrap() as usize,
                        '!' => total_size != number.to_digit(10).unwrap() as usize,
                        _ => false,
                    },
                    _ => false,
                };

                if condition_met {
                    call_read(global, current_pos, saved, contents, index, buffer, direct, array_index);
                } else {
                    let mut end_index = *index + 5;
                    while end_index < contents.len() - 1 && contents[end_index] != ']' {
                        end_index += 1;
                    }
                    *index = end_index + 1;
                }
            }
        }
        'j' => {
            if (*index)+4 > contents.len() {
                println!("Invalid input to if statement, check you putted an operator and number to be compared separated by spaces");
            }
            else {
                let operator = contents[*index+2];
                let mut end_index = *index + 5;

                while end_index < contents.len()-1 && contents[end_index] != '~' {
                    end_index += 1;
                }
                // If 'L' is found, create the substring and process the content between 'L' and 'L'
                let substring: String = contents[*index+5..end_index].iter().collect();
                *index = end_index;
                if operator == '=' && *buffer == substring {
                    call_read(global,current_pos, saved, contents,index,buffer,direct,array_index);
                }
                else if operator == '!' && *buffer != substring {
                    call_read(global,current_pos, saved, contents, index,buffer,direct,array_index);
                }
                else {
                    while end_index < contents.len()-1 && contents[end_index] != ']' {
                        end_index += 1;
                    }
                    *index = end_index+1;
                }
            }
        }
        '&' => {
            // Access and print the memory address of the element at current_pos
            let address = global.as_ptr() as usize + *current_pos as usize * std::mem::size_of::<i64>();
            println!("Memory Address: {:X}", address);
        }
        '`' => {
            let formatted_values: String = global[transformed]
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            println!("Vector {}: [{}]",*array_index, formatted_values);
        }
        'p' => {
            for row in global.iter() {
                let formatted_values: String = row
                    .iter()
                    .map(|&x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                println!("[{}]",formatted_values);
            }
        }
        '|' => {
            buffer.clear();
            std::io::stdin().read_line(buffer).expect("Invalid Input");
        }
        'q' => {
            let last_index = global.len()-1;
            let buffer_trimmed = buffer.trim();
            let buffer_len = buffer_trimmed.chars().count();

            // Resize the global vector to accommodate the additional characters
            global[transformed].resize(last_index + buffer_len, 0);

            let mut i: usize = 0;
            for letter in buffer_trimmed.chars() {
                global[transformed][last_index + i] = letter as i64;
                i += 1;
            }
            *index = *index + buffer_len;
        }
        'm' => {
            buffer.clear(); // Clear the buffer before appending new characters
            for num in 0..(*saved).min(size as i64 - 1) {
                let digit_char = char::from_digit(global[transformed][num as usize] as u32, 10).unwrap();
                buffer.push(digit_char);
            }
        }
        'y' => {
            // Check if the vector is not empty before removing the last element
            if let Some(_) = global.pop() {
                if size >= 2 { size-=1; } else { println!("Cannot pull, too few vectors remain"); }
            } else {
                println!("Global is completly empty");
            }
            total_size-=1;
        }
        'u' => {
            global.push(vec![0]);
            total_size+=1;
        }
        'i' => {
            (*array_index)+=1;
            if (*array_index) >= total_size as u32 {
                global.push(vec![0]);
            }
        }
        'o' => {
            if (*array_index) == 0 {
                println!("Cant go to a negative vector position");
                (*array_index) = 0;
            }
            else { (*array_index) = *array_index - 1; }
        }
        '\\' => *direct = true,
        ',' => println!("PID {}",std::process::id()),
        _ if letter.is_digit(10) => {
            let mut total = String::new();
            total.push(letter);
            let mut i: i16 = 1;
            while contents[*index+i as usize].is_digit(10) {
                total.push(contents[*index+i as usize]);
                i+=1;
            }
            let digit:i64 = total.parse::<i64>().unwrap();
            if *direct {
                *saved = digit;
                *direct = false;
                *index+=i as usize;
            }
            else {
                global[transformed][*current_pos as usize ] = digit;
                *index+=i as usize;
            }
        }
        _ => (),
    }
    return *index+1;
}

fn add_replacements(contents: &str) -> String {
    // Define the replacement map
    let mut replacement_map = HashMap::new();
    replacement_map.insert("add saved", "#");
    replacement_map.insert("minus saved", "$");
    replacement_map.insert("times saved", "e");
    replacement_map.insert("over saved", "r");
    replacement_map.insert("add", "+");
    replacement_map.insert("minus", "-");
    replacement_map.insert("duplicate", "*");
    replacement_map.insert("fold", "/");
    replacement_map.insert("go right", ">");
    replacement_map.insert("go left", "<");
    replacement_map.insert("go up", "o");
    replacement_map.insert("go down", "i");
    replacement_map.insert("show saved", "!");
    replacement_map.insert("save is", "\\");
    replacement_map.insert("save here", "^");
    replacement_map.insert("show buffer", ";");
    replacement_map.insert("what", "@");
    replacement_map.insert("go saved", "%");
    replacement_map.insert("loop {", "[");
    replacement_map.insert("}", "]");
    replacement_map.insert("where in pc", "&");
    replacement_map.insert("where", "?");
    replacement_map.insert("show all", "p");
    replacement_map.insert("show this", "`");
    replacement_map.insert("input", "|");
    replacement_map.insert("pid", ",");
    replacement_map.insert("show size", ".");
    replacement_map.insert("show global", "t");
    replacement_map.insert("push vector", "u");
    replacement_map.insert("pull vector", "y");
    replacement_map.insert("push", ")");
    replacement_map.insert("pull", "(");
    replacement_map.insert("place buffer", "q");
    replacement_map.insert("change buffer", "m");
    replacement_map.insert("if here", "a");
    replacement_map.insert("if saved", "s");
    replacement_map.insert("if index", "d");
    replacement_map.insert("if vector", "f");
    replacement_map.insert("if size", "g");
    replacement_map.insert("if global", "h");
    replacement_map.insert("if buffer", "j");

    // Perform replacements using the HashMap
    let mut replaced_contents = contents.to_string();
    for (key, value) in &replacement_map {
        replaced_contents = replaced_contents.replace(key, value);
    }

    replaced_contents
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 { println!("Invalid amount of arguments"); std::process::exit(1); }
    let last_six = &args[1][args[1].len() - 6..];
    if last_six != ".iwarn" { println!("Invalid extension of file"); std::process::exit(1); }


    let mut contents = String::new();
    let mut file = File::open(&args[1])?;
    file.read_to_string(&mut contents)?;
    // APL's nightmare
    let contents = add_replacements(&contents);
    let contents: Vec<char> = contents.chars().collect(); // Convert String to Vec<char>

    let mut global: Vec<Vec<i64>> = vec![vec![0]];
    let mut current_pos: i64 = 0;
    let mut saved: i64 = 0;

    let mut i = 0;
    let mut buffer:String = String::new();
    let mut direct:bool = false;
    let mut array_index:u32 = 0;
    while i < contents.len() {
        i = read(&mut global, &mut current_pos, &mut saved, &contents, &mut i,&mut buffer,&mut direct,&mut array_index);
    }

    Ok(())
}
