use std::fs::read_to_string;

const INPUT_FILE_PATH: &str = "input.txt";

fn main() {
    let mut sum: u64 = 0;

    for line in read_to_string(INPUT_FILE_PATH).unwrap().lines() {
        let line = line.as_bytes().into_iter();
        let reversed_line = line.clone().rev();
        for character in line {
            if character >= &b'0' && character <= &b'9' {
                sum += (*character as u64 - u64::from('0')) * 10;
                break;
            }
        }

        for character in reversed_line {
            if character >= &b'0' && character <= &b'9' {
                sum += *character as u64 - u64::from('0');
                break;
            }
        }
    }

    println!("{}", sum);
}
