use std::{fs::{create_dir, File}, io::{self, Write}, process::Command};
use std::env;


use colored::Colorize;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.is_empty(){
        eprintln!("{}", "Error! please enter a(some) valid arg(s).".red());
        println!("{} {}", "To see manual or help, please visit: ".bright_green() , "https://crates.io/crates/rohanasan".blue());
        return Ok(());
    }
    if args.len() > 1 {
        if &args[1] == "--help"{
            println!("{} {}", "To see manual or help, please visit: ".green(), "https://crates.io/crates/rohanasan".blue());
            return Ok(());
        }
        else if &args[1] == "--version"{
            println!("{}", "v0.0.2".yellow());
            return Ok(());
        }
        else if &args[1] != "new" {
            eprintln!("{} {}", "Error! unknown command:".red(), args[1].red().bold());
            return Ok(());
        }
    }
    if args.len() < 3 {
        eprintln!("{}", "Args are not of suitable length.".red());
        println!("{} {}", "To see manual or help, please visit: ".bright_green() , "https://crates.io/crates/rohanasan".blue());
        return Ok(());
    }
    let buffer = args[2].to_string().clone();

    println!("{} {}\n{}", "Creating your project:".bright_green().bold(),buffer.bright_green().bold().underline(), "Please be patient, this can take upto 10 seconds.".cyan().bold());

    if buffer.contains(' ') || buffer.is_empty() || buffer.contains('/') || buffer.contains('\\') || buffer.contains('?') || buffer.contains('.') {
        println!("{}", "Error: you don't have a correct folder name.".red());
        return Ok(());
    }
    let output = Command::new("cargo")
        .arg("new")  // Example argument
        .arg(buffer.clone()) // Example argument
        .arg("--bin") // Example URL
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        let _ = env::set_current_dir(buffer.clone());
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed: {}", stderr);
    }

    // env::set_current_dir(buffer.clone())?;
    Command::new("cargo")
        .arg("add")  // Example argument
        .arg("rohanasan") // Example argument
        .output()
        .expect("Failed to execute command");
    let mut file = File::create("./src/main.rs")?;

    // Write data to the file
    let data = r#"use rohanasan::{
    rohanasan, send_http_response, serve, Request, DEFAULT_HTML_HEADER,
};
fn handle(req: Request) -> String {
    send_file(DEFAULT_HTML_HEADER, "./html/index.html", req.data)
}

fn main() {
    rohanasan! {
        serve(8080, handle)
    }
}
"#;
    file.write_all(data.as_bytes())?;
    create_dir("html")?;
    create_dir("static")?;
    env::set_current_dir("html").expect("TODO: panic message");
    let mut file2 = File::create("./index.html")?;
    let data2 = r#"<html>
    <head>
    <title>
    Rohanasan
    </title>
    <style>
    @import url('https://fonts.googleapis.com/css2?family=Raleway:ital,wght@0,100..900;1,100..900&display=swap')
    </style>
    </head>
    <body>
    <h1>
    Hello from Rohanasan! This is the file ./html/index.html!
    <h2>Rohanasan's logo:</h2>
    <img src="/static/rohanasan.svg" width="300"/>
    </h1>
    </body>
    </html>
    "#;
    file2.write_all(data2.as_bytes())?;


    env::set_current_dir("../static").expect("TODO: panic message");
    let mut file3 = File::create("./rohanasan.svg")?;
    let data3 = r#"<?xml version="1.0" encoding="UTF-8" standalone="no" ?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="1080" height="1080" viewBox="0 0 1080 1080" xml:space="preserve">
<desc>Created with Fabric.js 5.2.4</desc>
<defs>
</defs>
<g transform="matrix(1 0 0 1 540 540)" id="01abc5eb-e757-49ef-ae9a-6a0ab9999720"  >
<rect style="stroke: none; stroke-width: 1; stroke-dasharray: none; stroke-linecap: butt; stroke-dashoffset: 0; stroke-linejoin: miter; stroke-miterlimit: 4; fill: rgb(255,255,255); fill-rule: nonzero; opacity: 1; visibility: hidden;" vector-effect="non-scaling-stroke"  x="-540" y="-540" rx="0" ry="0" width="1080" height="1080" />
</g>
<g transform="matrix(1 0 0 1 540 540)" id="b344c75a-759b-4ded-9f32-af8fa0c6e253"  >
</g>
<g transform="matrix(15.34 0 0 15.34 540 540)" id="ca553213-3c66-4c25-948d-daa2141593f1"  >
<circle style="stroke: rgb(0,0,0); stroke-width: 0; stroke-dasharray: none; stroke-linecap: butt; stroke-dashoffset: 0; stroke-linejoin: miter; stroke-miterlimit: 4; fill: rgb(255,255,255); fill-rule: nonzero; opacity: 1;" vector-effect="non-scaling-stroke"  cx="0" cy="0" r="35" />
</g>
<g transform="matrix(10.52 0 0 10.52 540 540)" id="7ac4ee8c-54e4-43ce-b03b-2be012a68b25"  >
<path style="stroke: rgb(0,0,0); stroke-width: 0; stroke-dasharray: none; stroke-linecap: butt; stroke-dashoffset: 0; stroke-linejoin: miter; stroke-miterlimit: 4; fill: rgb(94,85,222); fill-rule: nonzero; opacity: 1;" vector-effect="non-scaling-stroke"  transform=" translate(-50, -50)" d="M 50 92.875 C 26.358 92.875 7.125 73.642 7.125 50 C 7.125 26.358000000000004 26.358 7.125 50 7.125 C 73.642 7.125 92.875 26.358 92.875 50 C 92.875 73.642 73.642 92.875 50 92.875 z M 50 9.125 C 27.461 9.125 9.125 27.461 9.125 50 C 9.125 72.538 27.461 90.875 50 90.875 C 72.538 90.875 90.875 72.538 90.875 50 C 90.875 27.461 72.538 9.125 50 9.125 z" stroke-linecap="round" />
</g>
<g transform="matrix(1 0 0 1 540 540)" style="" id="8a52fcec-9f7b-4c12-85f8-33bb3943fb22"  >
		<text xml:space="preserve" font-family="Raleway" font-size="129" font-style="normal" font-weight="900" style="stroke: none; stroke-width: 1; stroke-dasharray: none; stroke-linecap: butt; stroke-dashoffset: 0; stroke-linejoin: miter; stroke-miterlimit: 4; fill: rgb(0,0,0); fill-rule: nonzero; opacity: 1; white-space: pre;" ><tspan x="-350.3" y="40.52" >Rohanasan</tspan></text>
</g>
</svg>
    "#;
    file3.write_all(data3.as_bytes())?;
    println!("{}", "Your rohanasan project has been generated!".green().bold());
    println!("{}", "Now, you can start your project by running: ".green());
    println!("{} {}", "cd".green(), buffer.blue());
    println!("{} {} {}", "cargo".green(), "run".blue(), "--release".yellow());
    println!("{} {}", "Then you can go to:".bold().bright_magenta(),"http://localhost:8080".blue());
    println!("{} {}", "Having doubts? Join discord:".yellow().bold(), "https://discord.gg/Yg2A3mEret".blue().bold());
    println!("{} {}", "Want to support rohanasan? You can donate at: ".bright_green().bold(), "https://www.buymeacoffee.com/rohanvashisht".blue().bold());
    println!("{}", "Thanks a lot for choosing Rohanasan :)".yellow().bold());
    Ok(())
}
