use structopt::StructOpt;
use colored::*;
use failure::ResultExt;
use exitfailure::ExitFailure;
use std::io::{self, Read};

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "MoeGirl")]
    message: String,

    #[structopt(short = "d", long = "dead")]
    dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    catfile: Option<std::path::PathBuf>,

    #[structopt(short= "i", long = "stdin")]
    stdin: bool,
}

fn main() -> Result<(), ExitFailure> {

    let options = Options::from_args();
    let mut message = String::new();

    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    }else{
        message = options.message;
    }

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.");
    }

    let eye = if options.dead { "x" } else { "o" };
    //let message = std::env::args().nth(1).expect("Missing the message. Usage: catsay <message>");
    println!("{}", message.on_bright_green());

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)//.expect(&format!("could not read file {:?}", path));
            .with_context(|_| format!("could not read file {:?}", path))?;
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", &cat_picture);
        },
        None => {
            println!(" \\");
            println!("  \\");
            println!("     /\\_/\\");
            println!("    ( {eye} {eye} )", eye = eye.red().bold());
            println!("    =( I )=");
        }
    }


    Ok(())
    

}
