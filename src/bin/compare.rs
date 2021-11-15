use indicatif::{ProgressBar, ProgressStyle};
use std::io::Read;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
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
    let wrong_count = Arc::new(AtomicI32::new(0));
    let huge_wrong_count = Arc::new(AtomicI32::new(0));
    for _ in 0..threads_count {
        bar.tick();
        let bar = bar.clone();
        let wrong_count =  wrong_count.clone();
        let huge_wrong_count = huge_wrong_count.clone();
        threads.push(spawn(move || {
            for _ in 0..total_number / threads_count {
                let mut input_data = String::new();
                Exec::shell("./target/release/generate-map")
                    .stream_stdout()
                    .unwrap()
                    .read_to_string(&mut input_data)
                    .unwrap();
                // println!("{}", input_data);
                let mut output_sa = String::new();
                let begin_time = std::time::Instant::now();
                output_sa = Exec::shell("./target/release/simulated-annealing")
                    .stdin(&input_data[..])
                    .stderr(NullFile)
                    .stdout(Redirection::Pipe)
                    .capture()
                    .unwrap()
                    .stdout_str();
                let sa_end_time = std::time::Instant::now();
                let mut output_dp = String::new();
                output_dp = Exec::shell("./dynamic-programming")
                    .stdin(&input_data[..])
                    .stdout(Redirection::Pipe)
                    .capture()
                    .unwrap()
                    .stdout_str();
                let dp_end_time = std::time::Instant::now();
                let sa_read: f64;
                let dp_read: f64;
                scan!(output_sa.bytes() => "{}", sa_read);
                scan!(output_dp.bytes() => "{}", dp_read);
                println!("{} {}", (sa_end_time - begin_time).as_secs_f32(), (dp_end_time - sa_end_time).as_secs_f32());
                // println!("{}  {}  {}", sa_read, dp_read, (dp_read - sa_read).abs());
                if (dp_read - sa_read).abs() > 1e-6 {
                    wrong_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                if (dp_read - sa_read).abs() / dp_read > 0.05 {
                    huge_wrong_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                bar.set_message(format!("{} / {} / {}", wrong_count.load(Ordering::SeqCst), huge_wrong_count.load(Ordering::SeqCst), bar.position()));
                bar.inc(1)
            }
        }))
    }
    for thread in threads {
        thread.join();
    }
    bar.finish();
    println!("wrong_count: {}, huge_wrong_count: {}", wrong_count.load(Ordering::SeqCst), huge_wrong_count.load(Ordering::SeqCst));
}
