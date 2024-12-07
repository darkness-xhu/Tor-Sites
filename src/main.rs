use std::{fs, io::{self, Write}};
use scraper::{Html, Selector};
use reqwest;
use colored::Colorize;
use std::fs::{File, OpenOptions};

fn banner() {
    let banner = r#"
    
$$$$$$$$\                                    $$\   $$\               
\__$$  __|                                   \__|  $$ |              
   $$ | $$$$$$\   $$$$$$\           $$$$$$$\ $$\ $$$$$$\    $$$$$$\  
   $$ |$$  __$$\ $$  __$$\ $$$$$$\ $$  _____|$$ |\_$$  _|  $$  __$$\ 
   $$ |$$ /  $$ |$$ |  \__|\______|\$$$$$$\  $$ |  $$ |    $$$$$$$$ |
   $$ |$$ |  $$ |$$ |               \____$$\ $$ |  $$ |$$\ $$   ____|
   $$ |\$$$$$$  |$$ |              $$$$$$$  |$$ |  \$$$$  |\$$$$$$$\ 
   \__| \______/ \__|              \_______/ \__|   \____/  \_______|
                                                                     

    "#.red().bold();
    println!("{}", banner);
    println!("{}","This tool is not that great, but if you still want to use it, you can try. If you have any suggestions, feel free to contact me here:".white().bold());
    println!("{}{} {}","Instagram".yellow().bold(),":".purple().bold(),"darkness.xhu".red().bold())
    
}

fn keywordinput() -> String {
    loop {
        print!("\n{}{}","[*]".white().bold(), " Enter your keyword: ".yellow().bold() ) ;
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error: Could not read user input");
        let keyword = input.trim().to_string();
        if keyword.is_empty() {
            println!("Keyword cannot be empty, please try again.");
            continue
        };
        return keyword;

    }  
}

fn user_inp() -> String {
    loop {
        print!("\n{}{}","[*]".white().bold(), " Do you want to save the URL in a file? (y for yes, n for no): ".green().bold());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error: Could not read user input");
        let input = input.trim().to_string();
        if input == "y" {
            return "y".to_string();
        }else if input == "n" {
            return "n".to_string();
        }else {
            println!("\nError: Invalid input. Please enter 'y' for yes or 'n' for no.");
            continue;
        }       
    }; 
}

fn filepath() -> String{
    loop {
        print!("\n{}{}","[*]".yellow().bold(), " Enter the directory path where you want to store the output: ".red().bold());
        io::stdout().flush().unwrap();
        let mut path =  String::new();
        io::stdin().read_line(&mut path).expect("Error: Could not read user input");
        let path = path.trim().to_string();
        let pathexist = match fs::metadata(&path) {
            Ok(metadata) => metadata.is_dir(),
            Err(_) => false,
        };
        if pathexist == true {
            return path;
        }else {
            println!("\nError: Invalid directory path. Please ensure the path exists and try again.");
            continue;
        }
        
    }
}

fn scraper(keyword: &str) -> Result<(),reqwest::Error>{
    let url = format!("https://ahmia.fi/search/?q={}", keyword);
    let response = reqwest::blocking::get(url)?.text()?;
    let document = Html::parse_document(&response);
    let selector = Selector::parse("cite").unwrap();
    let elements: Vec<_> = document.select(&selector).collect();
    let mut links = Vec::new();
    let mut x = 0;
    for element in &elements {
        links.push(element.inner_html());
    }
    links.sort();
    links.dedup();

    if &*user_inp() == "y" {
        let mut dirpath: String = filepath();
        if dirpath.ends_with('/') {
            dirpath.pop();
        }
        let fullpath = format!("{}/{}.txt", &dirpath, &keyword);
        let file = File::create(&fullpath);
        match file {
            Ok(mut file) => {
                let _ = OpenOptions::new().write(true).append(true).open(&fullpath).expect("Error: Unable to open file for appending. Please check the file path and ensure you have the necessary read/write permissions.");
                for urls in &links {
                    match writeln!(file, "{}",urls) {
                        Ok(_) => {},
                        Err(e) => eprintln!("Error writing to file: {}", e),
                    }
                }
            },
            Err(e) => eprintln!("Error: {}", e)
        }      
        println!("\n{}{} {}","Output successfully saved to".white().bold(), ":".red(), &fullpath.yellow().bold());

        
    }else {
        for link in links.iter() {
            x += 1;
            println!("[{x}] {}", link);
        };
    }
    
    Ok(())  
}

fn main() {
    banner();
    let keyword = keywordinput();
    match scraper(&keyword){
        Ok(x) => x,
        Err(_) => eprintln!("There was a problem with the tool. Please contact the developer.")
    }
}