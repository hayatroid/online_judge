use std::{fs::File, io::{BufReader, Read}, process::{Command, Stdio}, time};

// メモリの取得に使う
use wait4::Wait4;

fn main() {
    // パスの指定
    let path_to_program = "problems/aplusb/program_ac.py";
    // let path_to_program = "problems/aplusb/program_mle.py";
    // let path_to_program = "problems/aplusb/program_re.py";
    // let path_to_program = "problems/aplusb/program_tle.py";
    // let path_to_program = "problems/aplusb/program_wa.py";
    let path_to_input = "problems/aplusb/input.txt";
    let path_to_output = "problems/aplusb/output.txt";

    // メモリ，実行時間制限（additional を加えたリソースで行う）
    let memory_limit = 1024 * 1024;
    let memory_additional = 10 * 1024;
    let time_limit = 2;
    let time_additional = 1;

    // ジャッジコマンドの定義（Python しかないよ；；）
    let mut cmd = Command::new("sh")
        .arg("-c")
        .arg(format!("ulimit -m {} & timeout {} python3 {} < {}", memory_limit + memory_additional, time_limit + time_additional, path_to_program, path_to_input))
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn");

    // コマンド実行 & 実行時間の測定
    let now = time::Instant::now();
    let res = cmd.wait4().expect("failed to execute command");
    let duration = now.elapsed().as_millis();

    // メモリ，ステータスコードの取得
    let memory = res.rusage.maxrss / 1024;
    let status_code = res.status.code().expect("failed to get status code");

    // ユーザーの出力の取得
    let mut user_output_text = String::new();
    cmd.stdout.expect("failed to get your output").read_to_string(&mut user_output_text).expect("failed to parse your output");

    // 正答の出力の取得
    let output_file = File::open(path_to_output).expect("failed to open output.txt");
    let mut buf_reader = BufReader::new(output_file);
    let mut output_text = String::new();
    buf_reader.read_to_string(&mut output_text).expect("failed to read output.txt");

    // ステータスの判定
    let status = if status_code == 0 {
        if user_output_text == output_text {
            "AC"
        } else {
            "WA"
        }
    } else {
        let is_mle = memory > memory_limit;
        let is_tle = duration > time_limit * 1000;
        if is_mle && is_tle {
            "MLE and TLE"
        } else if is_mle {
            "MLE"
        } else if is_tle {
            "TLE"
        } else {
            "RE"
        }
    };

    // ステータス，ユーザーの出力，正答の出力，実行時間，メモリの表示
    println!("{}", status);
    println!("your output\n{}", user_output_text);
    println!("expected output\n{}", output_text);
    println!("duration: {}ms", duration);
    println!("memory: {}KB", memory);
}
