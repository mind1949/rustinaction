use std::process;
use std::thread::sleep;
use std::time;

/// SIGSTOP 和 SIGKILL 无法由应用程序处理
/// SIGSTOP        会暂停程序执行，可通过 SIGCONT 恢复执行
/// SIGKILL        会强制杀死程序
///
/// SIGHUP  Ctrl+D 要求程序重新读取配置文件
/// SIGINT  Ctr+C  要求终止一个正在运行的程序
/// SIGTERM        要求程序优雅退出
fn main() {
    let delay = time::Duration::from_secs(1);

    let pid = process::id();
    println!("{}", pid);

    for i in 1..=60 {
        sleep(delay);
        println!(". {}", i);
    }
}
