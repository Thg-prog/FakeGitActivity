use chrono::prelude::*;
use std::process::{Command, exit};
use std::fs::File;
use std::io::prelude::*;
use tokio::fs;
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
    let file_name = format!("files_src\\{}.txt", date);
    let mut file = File::create(file_name.clone());
    println!("{}",file_name);

    file?.write_all(format!("Added at {}",Local::now().format("%Y-%m-%d %H:%M:%S").to_string()).as_bytes());
    println!("Успешно создан и записан файл:{}", file_name);
    Ok(())
}

async fn delete_files_path(folder_name: &str)->Result<(),std::io::Error>{
    fs::remove_dir_all(folder_name).await
}

async fn make_files_path(folder_name: &str)->Result<(),std::io::Error>{
    fs::create_dir(folder_name).await
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    let first_interval = Duration::from_secs(60);
    let mut ticker_files = interval(first_interval);
    let mut count_files = 0;

    loop {
        ticker_files.tick().await;


        if let Err(e) = make_new_git_file(){
            eprintln!("An error occured: {}", e);
        }
        count_files+=1;
        make_git_request();
        if count_files == 15{
            if let Err(e) = delete_files_path("files_src").await{
                eprintln!("Error with deliting files path")
            }
            if let Err(e) = make_files_path("files_src").await{
                eprintln!("Error with make files path");
            }
            count_files = 0;
            println!("File_path remaked");
            exit(0x0100);
        }
    }

}
