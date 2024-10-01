use std::{io::Write, str};

use clap::Parser;

/// Replace all instancess of a word or blah blah blah
#[derive(Parser)]
struct CLi {
    /// The style path
    style: std::path::PathBuf,
    /// The input path
    input: std::path::PathBuf,
    /// the name that will be given to the output file
    #[arg(default_value("output.txt"))]
    output: String,
}

#[derive(Debug)]
enum Modifier {
    IgnoreCase,
}

fn main() {
    let args = CLi::parse();
    
    let input = std::fs::read_to_string(&args.input).expect("could not read file");
    let style_file = std::fs::read_to_string(&args.style).expect("could not read file");
    
    let mut style: Vec<(&str, Vec<Modifier>, String)> = Vec::new();

    for line in style_file.lines() {
        if !line.contains("{") {panic!("Wrong syntax used at style");}

        let mut line = line.split_whitespace();
        let key = line.next().expect("no key");
        let mut modifiers: Vec<Modifier> = Vec::new();
        let mut next = line.next().expect("no more next");

        if next.contains("/") {
            next = line.next().expect("Syntax Error");
            while !next.contains("{") {
                match next {
                    "ignore_case" => modifiers.push(Modifier::IgnoreCase),
                    "/" => {/* do nothing */},
                    _ => panic!("Wrong modifier {:?}", next),
                }
                next = line.next().expect("An error occured while reading the modifiers");
            }
        }

        let value = next.replace("{", "").replace("}", "");


        style.push((key, modifiers, value));
    }

    println!("{:?}", style);

    let mut output = std::fs::File::create(&args.output).expect("could not create file");

    for line in input.lines() {
        let mut nl: String = String::from(line);
        for element in &style {
            let mut key = String::from(element.0);

            for modifier in &element.1 {
                match modifier {
                    Modifier::IgnoreCase => {
                        key = key.to_ascii_lowercase();
                        let lower = nl.to_ascii_lowercase().replace(&key, &element.2);
                        let mut og = nl.split("").collect::<Vec<&str>>().into_iter();
                        let mut new = String::new();
                        let mut cog = og.next().expect("Error");
                        for letter in lower.split("") {
                            if letter == cog.to_ascii_lowercase() {
                                new += cog;
                            } else {
                                new += letter;
                            }

                            let ncog = og.next();
                            cog = match ncog {
                                Option::None => "~",
                                _ => ncog.expect("msg"),
                            }
                        }
                        nl = new;
                    }
                    _ => {}
                }
            }
            nl = nl.replace(&key, &element.2);
        }
        output.write_all(&nl.as_bytes()).expect("Error writing output");
        output.write_all(b"\n").expect("Error writing putout");
    }
}