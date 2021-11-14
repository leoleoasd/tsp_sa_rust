use indicatif::{ProgressBar, ProgressStyle};
use std::io::Read;
use std::thread::spawn;
use subprocess::{Exec, NullFile, Redirection};
use text_io::scan;

fn main() {
    let mut threads = Vec::<std::thread::JoinHandle<()>>::new();
    let threads_count = 16;
    let total_number = 16;
    let bar = ProgressBar::new(total_number);
    bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed} / {eta}]({per_sec}) {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("##-"));
    let mut wrong_count = 0;
    for _ in 0..threads_count {
        bar.tick();
        let bar = bar.clone();
        threads.push(spawn(move || {
            for _ in 0..total_number / threads_count {
                let mut input_data = String::new();
                Exec::shell("./target/debug/generate-map")
                    .stream_stdout()
                    .unwrap()
                    .read_to_string(&mut input_data)
                    .unwrap();
                // println!("{}", input_data);
                let mut output_sa = String::new();
                output_sa = Exec::shell("./target/debug/simulated-annealing")
                    .stdin(&input_data[..])
                    .stderr(NullFile)
                    .stdout(Redirection::Pipe)
                    .capture()
                    .unwrap()
                    .stdout_str();
                let mut output_dp = String::new();
                output_dp = Exec::shell("./dynamic-programming")
                    .stdin(&input_data[..])
                    .stdout(Redirection::Pipe)
                    .capture()
                    .unwrap()
                    .stdout_str();
                let sa_read: f64;
                let dp_read: f64;

                scan!(output_sa.bytes() => "{}", sa_read);
                scan!(output_dp.bytes() => "{}", dp_read);
                if dp_read - sa_read > 1e-6 {
                    wrong_count += 1;
                }
                bar.inc(1)
            }
        }))
    }
    for thread in threads {
        thread.join();
    }
    bar.finish();
    println!("{}", wrong_count);
}
