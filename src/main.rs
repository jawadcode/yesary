use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};

fn main() {
    //yes, yeS, yEs, yES, Yes, YeS, YEs, YES
    let map: HashMap<u8, &str> = [
        (0, "yes"),
        (1, "yeS"),
        (2, "yEs"),
        (3, "yES"),
        (4, "Yes"),
        (5, "YeS"),
        (6, "YEs"),
        (7, "YES"),
    ]
    .iter()
    .cloned()
    .collect();

    print!("Please enter the name of your file: ");
    io::stdout().flush().unwrap();

    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();

    print!("Please enter the name of the output file: ");
    io::stdout().flush().unwrap();

    let mut output_filename = String::new();
    io::stdin().read_line(&mut output_filename).unwrap();

    let contents = fs::read(filename.trim()).expect("Error Occurred while reading file");
    let mut output_file =
        File::create(output_filename.trim()).expect("Error Occurred while creating output file");

    for byte in contents {
        let octal_digits = (byte / 64, byte % 64 / 8, byte % 8);
        output_file
            .write_fmt(format_args!(
                "{}{}{}",
                map[&octal_digits.0], map[&octal_digits.1], map[&octal_digits.2],
            ))
            .unwrap();
    }
}
