use std::fs::read_to_string;

const INPUT_FILE_PATH: &str = "input.txt";

const NUMBER_NAMES: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_number_in_string(s: &String) -> Option<u64> {
    let mut index = 0;
    for number_name in NUMBER_NAMES {
        if s.find(number_name).is_some() {
            return Some(index);
        }
        index += 1;
    }
    None
}

fn main() {
    let mut sum: u64 = 0;
    let mut word_buffer: String;

    for line in read_to_string(INPUT_FILE_PATH).unwrap().lines() {
        word_buffer = String::from("");

        let line = line.as_bytes().into_iter();
        let reversed_line = line.clone().rev();
        for character in line {
            if character >= &b'0' && character <= &b'9' {
                let number_in_string = *character as u64 - u64::from('0');
                sum += number_in_string * 10;
                break;
            }

            word_buffer.push(*character as char);

            if let Some(number_in_string) = get_number_in_string(&word_buffer) {
                sum += number_in_string * 10;
                break;
            }
        }

        word_buffer = String::from("");

        for character in reversed_line {
            if character >= &b'0' && character <= &b'9' {
                let number_in_string = *character as u64 - u64::from('0');
                sum += number_in_string;
                break;
            }

            word_buffer.insert(0, *character as char);

            if let Some(number_in_string) = get_number_in_string(&word_buffer) {
                sum += number_in_string;
                break;
            }
        }
    }

    println!("{}", sum);
}
