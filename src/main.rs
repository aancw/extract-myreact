// Copyright (c) 2022 Petruknisme
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

extern crate clap;
extern crate zip;
extern crate colored;

use std::{fs, fs::File, io::Write, io::Read};
use clap::Parser;
use colored::Colorize;
use zip::read::ZipArchive;



#[derive(Parser)]
#[clap(name = "Extract My React")]
#[clap(author = "Petruknisme <me@petruknisme.com>")]
#[clap(version = "1.0")]
#[clap(about = "Extracting React Native app source code from apk file.", long_about = None)]

struct Cli {
    /// React app APK file location
    #[clap(short, long)]
    file: String,

    /// Directory location to store extracted file. Pass only directory.
    #[clap(short, long)]
    out: String,
}

/* fn format_js(filename: &Path, text: &str) -> String {
    let config = dprint_plugin_typescript::configuration::ConfigurationBuilder::new()
        .line_width(80)
        .prefer_hanging(true)
        .prefer_single_line(false)
        .quote_style(QuoteStyle::PreferSingle)
        .next_control_flow_position(NextControlFlowPosition::SameLine)
        .build();
    std::panic::catch_unwind(|| {
        dprint_plugin_typescript::format_text(filename, text, &config).unwrap()
    })
    .unwrap_or_else(|_| text.to_string())
}  */


fn main() {
    let cli = Cli::parse();
    let file_loc = cli.file;
    let out = format!("{}/result-react.js", cli.out);
    let mut out_file = File::create(&out).expect("Unable to create");
    
    println!("{}", "\t\tExtract My React\nExtracting React Native app source code from apk file\n".yellow());
    println!("{} {} {}",
                "[*] Using".blue(),
                file_loc.red(),
                "as input file ".blue() 
            );
    println!("{} {} {} {}",
                "[*] Backup".blue(),
                file_loc.red(),
                "to".blue(),
                format!("{}.master", file_loc)
            );
    fs::copy(&file_loc, format!("{}.master", file_loc)).unwrap();
    let zipfile = std::fs::File::open(&file_loc).unwrap();
    let mut archive = ZipArchive::new(zipfile).unwrap();
    let mut contents = String::new();
     
    println!("{}", "[*] Extracting content from react native assets".blue());
    archive
            .by_name("assets/index.android.bundle")
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
    
    println!("{} {}", "[*] Saving result as".blue(), out);
    out_file.write_all(&contents.as_bytes()).unwrap();
    // JS Formatter when beautify minified js is hang
    // https://github.com/dprint/dprint-plugin-typescript/issues/129
    println!("{}", "[*] Done. Enjoy~".blue());
    println!("{}", "[*] Note: If you want to beautify the js file, consider installing https://github.com/beautify-web/js-beautify".red());

}