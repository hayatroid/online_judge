use std::{fs::File, io::{BufReader, Read}, process::{Command, Stdio}};

fn main() {
    let path_to_program = "problems/aplusb/program_ac.py";
    // let path_to_program = "problems/aplusb/program_re.py";
    // let path_to_program = "problems/aplusb/program_tle.py";
    // let path_to_program = "problems/aplusb/program_wa.py";
    let path_to_input = "problems/aplusb/input.txt";
    let path_to_output = "problems/aplusb/output.txt";

    let user_output = Command::new("sh")
        .arg("-c")
        .arg(format!("timeout 2 python3 {} < {}", path_to_program, path_to_input))
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute python3 command");
    let user_output_text = String::from_utf8(user_output.stdout).expect("failed to parse your output");
    let status_code = user_output.status.code().expect("failed to get status code");

    let output_file = File::open(path_to_output).expect("failed to open output.txt");
    let mut buf_reader = BufReader::new(output_file);
    let mut output_text = String::new();
    buf_reader.read_to_string(&mut output_text).expect("failed to read output.txt");

    if status_code == 1 {
        println!("RE");
    } else if status_code == 124 {
        println!("TLE");
    } else if user_output_text == output_text {
        println!("AC");
    } else {
        println!("WA");
        println!("your output: {}", user_output_text);
        println!("correct output: {}", output_text);
    }
}
