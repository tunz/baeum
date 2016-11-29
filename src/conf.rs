
pub struct Conf {
  args: Vec<String>,
  input_path: String,
  pub output_dir: String,
}

impl Conf {
  pub fn new(args:Vec<&str>, output_dir:&str, input_path: &str) -> Conf {
    Conf {
      args: args.iter().map(|&s|
                              if s == "@@" { String::from(input_path) }
                              else { String::from(s) }
                            ).collect(),
      output_dir: String::from(output_dir),
      input_path: String::from(input_path)
    }
  }

  pub fn new_without_filename(args:Vec<&str>, output_dir:&str) -> Conf {
    let filepath = format!("{}/.input", output_dir);
    Conf::new(args, output_dir, &filepath)
  }
}

