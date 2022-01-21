use clap::{arg, App};

fn main() {
  let matches = App::new("patcat")
              .version("0.1")
              .author("Gary Armstrong")
              .about("Like cat but for pattern visibility")
              .arg(arg!(--file <FILENAME>))
              .get_matches();

  println!("File name {:?}", matches.value_of("file").expect("required"));
}
