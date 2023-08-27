use chrono::prelude::*;
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use tokio::time::{Duration, interval};

fn make_git_request(){
    let commit_message = format!("New commit added at {}",Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
    let adding = Command::new("git")
	.args(&["add","."])
	.status()
	.expect("Adding problem");
    let commit = Command::new("git")
        .args(&["commit", "-m", &commit_message])
        .status()
        .expect("Smth went wrong");
    if commit.success() {
        let push = Command::new("git")
            .arg(&mut "push")
            .status()
            .expect("Smth went wrong with push data");
    }
}
fn make_new_git_file()->std::io::Result<()>{
    let date =  Local::now().format("%Y%m%d%H%M%S").to_string();
    let file_name = format!("{}.txt", date);
    let mut file = File::create(file_name.clone());
    println!("{}",file_name);

    file?.write_all(format!("Added at {}",Local::now().format("%Y-%m-%d %H:%M:%S").to_string()).as_bytes());
    println!("Успешно создан и записан файл:{}", file_name);
    Ok(())
}

#[tokio::main]
async fn main() {
    let first_interval = Duration::from_secs(30);
    let mut ticker_files = interval(first_interval);
    let second_interval = Duration::from_secs(5);
    let mut ticker_git = interval(second_interval);

    loop {
        ticker_files.tick().await;
        if let Err(e) = make_new_git_file(){
            eprintln!("An error occured: {}", e);
        }
    make_git_request();
    }

}
