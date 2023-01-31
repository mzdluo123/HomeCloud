use pcap::{Error, Precision};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self},
    process::{self, Command, Stdio},
    sync::{Arc, RwLock},
    thread::{self},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    filter: String,
    device: String,
    timeout: i32,
    start: String,
    stop: String,
}

// #[derive(Parser, Debug)]
// #[command(author, version, about = "在有网络流量时执行程序")]
// struct Args {
//     #[arg(short = 'c', long, help = "配置文件")]
//     config: String,
// }

fn read_config() -> Config {
    // let arg = Args::parse();
    let commands: Vec<String> = env::args().collect();
    if commands.len() < 2 {
        println!("请提供配置文件路径");
        process::exit(-1);
    }
    let path = &commands[1];

    let file = match fs::read_to_string(&path) {
        Ok(_f) => _f,
        Err(_e) => {
            println!("无法读取文件 {}", &path);
            process::exit(-1);
        }
    };

    match toml::from_str::<Config>(&file) {
        Ok(_c) => _c,
        Err(_e) => {
            println!("无法解析文件 {}", _e);
            process::exit(-1);
        }
    }
}

fn now_sec() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs()
}

fn run_command(command: &String) {
    let local_command = command.to_owned();
    thread::spawn(move || {
        let _ = Command::new("sh")
            .arg("-c")
            .arg(local_command)
            .stdout(Stdio::inherit())
            .spawn();
    });
}

fn check_time(config: &Config, shared_time: Arc<RwLock<u64>>) {
    let mut is_started = true;
    loop {
        if let Ok(_l) = shared_time.try_read() {
            let now = now_sec();
            if _l.abs_diff(now) > config.timeout.try_into().unwrap() {
                // 超时
                if is_started {
                    run_command(&config.stop);
                    is_started = false;
                    continue;
                }
            } else {
                if !is_started {
                    //上一个包时间很短并且没启动
                    run_command(&config.start);
                    is_started = true;
                }
            }
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn start_monitor(config: &Config, shared_time: Arc<RwLock<u64>>) {
    let mut time_cache = now_sec();

    let mut cap = pcap::Capture::from_device(&*config.device)
        .unwrap()
        .immediate_mode(true)
        .precision(Precision::Micro)
        .snaplen(32) // 32应该能减小性能影响？大概
        .open()
        .unwrap();
    cap.filter(&config.filter, true).unwrap();
    println!("listening...");
    loop {
        match cap.next_packet() {
            Ok(_) => {
                let now = now_sec(); // 减少获取锁次数提高性能
                if time_cache.abs_diff(now) > 1 {
                    //1秒更新一次
                    if let Ok(mut _l) = shared_time.try_write() {
                        *_l = now;
                        time_cache = now;
                    }
                }
            }
            Err(_e) => {
                if _e == Error::NoMorePackets {
                    println!("no more packets!");
                    process::exit(-1);
                }
            }
        }
    }
}

fn main() {
    // ???
    let config = read_config();
    let config2 = config.clone();
    let last_packet = Arc::new(RwLock::new(now_sec()));
    let last_apcket2 = last_packet.clone();
    thread::spawn(move || {
        start_monitor(&config, last_packet);
    });
    thread::spawn(move || {
        check_time(&config2, last_apcket2);
    })
    .join()
    .unwrap();
}