use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  name: String,

  #[arg(short, long, default_value_t = 1)]
  count: u8,
}

fn main() {

  let args = Args::parse();

  // let matches = App::new("patcat")
  //             .version("0.1")
  //             .author("Gary Armstrong")
  //             .about("Like cat but for pattern visibility")
  //             .arg(arg!(--file <FILENAME>))
  //             .get_matches();

  println!("name {:?}", args.name)
}
