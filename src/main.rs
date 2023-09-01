use std::env;
use entropy::Entropy;
use std::fs::File;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 { return; }
 
  let file = File::open(args[1].trim()).expect("Couldn't open the file at the provided path.");
  let entropy_calc = Entropy::new(&file);

  let ent_strng = entropy_calc.shannon_entropy().to_string();
  let ent_f32 = match ent_strng.parse::<f32>() {
    Ok(o) => o,
    Err(e) => { println!("{}", e); return; }
  };
  let mut j: f32 = 0.0;
  loop {
    if j == 8.5 { break; }
    if j < ent_f32 {
      print!("   ");
    }
    j += 0.5;
  }
  print!("   ");
  entropy_print(ent_f32);  
  print!("\x1b[38;5;5m0.0\x1b[0m");
  let mut i: f32 = 0.0;
  loop {
    if i == 8.5 { break; }
    if i <= ent_f32 {
      let colored_string = ansi_rgb_string_special(i);
      print!("{}",colored_string);
    }
    else {
      let colored_string = ansi_black();
      print!("{}",colored_string);
    }
    i += 0.5;
  }
  print!("\x1b[38;5;5m8.0\x1b[0m");
  println!();
  print!("                     |_______|               |_______|\n");
  print!(" \x1b[38;5;7m                     English              Encrypted/Compressed\x1b[0m");
  println!(); 
}

fn entropy_print(entropy: f32) {
  let mut ansi_code = String::from("\x1b[38;5;");
  let r: u8 = 0;
  let g: u8 = 5;
  let b: u8 = 5;
  let color: u8 = 16 + (r * 36) + (g * 6) + b;
  ansi_code.push_str(&color.to_string());
  ansi_code.push_str("m");
  ansi_code.push_str(&entropy.to_string());
  ansi_code.push_str("\x1b[0m");
  println!("{}", ansi_code);
}

fn ansi_black () -> String {
  let mut ansi_code = String::from("\x1b[48;5;");
  let r: u8 = 0;
  let g: u8 = 0;
  let b: u8 = 0;
  let color: u8 = 16 + (r * 36) + (g * 6) + b;
  ansi_code.push_str(&color.to_string());
  ansi_code.push_str("m");
  ansi_code.push_str("   ");
  ansi_code.push_str("\x1b[0m");
  return ansi_code;
}

fn ansi_rgb_string_special (e: f32) -> String {
  let mut ansi_code = String::from("\x1b[48;5;");
  let mut r: u8 = 0;
  let mut g: u8 = 0;
  let mut b: u8 = 0;
  if e < 0.5 { r = 0; g = 1; b = 0; }
  if e >= 0.5 && e < 1.0 { r = 0; g = 2; b = 0; }
  if e >= 1.0 && e < 1.5 { r = 0; g = 3; b = 0; }
  if e >= 1.5 && e < 2.0 { r = 0; g = 4; b = 0; }
  if e >= 2.0 && e < 2.5 { r = 0; g = 5; b = 0; }
  if e >= 2.5 && e < 3.0 { r = 2; g = 5; b = 0; }
  if e >= 3.0 && e < 3.5 { r = 3; g = 5; b = 0; }
  if e >= 3.5 && e < 4.0 { r = 4; g = 5; b = 0; }
  if e >= 4.0 && e < 4.5 { r = 5; g = 5; b = 1; }
  if e >= 4.5 && e < 5.0 { r = 5; g = 4; b = 0; }
  if e >= 5.0 && e < 5.5 { r = 5; g = 3; b = 0; }
  if e >= 5.5 && e < 6.0 { r = 5; g = 2; b = 0; }
  if e >= 6.0 && e < 6.5 { r = 5; g = 1; b = 0; }
  if e >= 6.5 && e < 7.0 { r = 5; g = 0; b = 0; }
  if e >= 7.0 && e < 7.5 { r = 4; g = 0; b = 0; }
  if e >= 7.5 && e < 8.0 { r = 3; g = 0; b = 0; }
  if e >= 8.0 && e < 8.5 { r = 2; g = 0; b = 0; }
  let color: u8 = 16 + (r * 36) + (g * 6) + b;
  ansi_code.push_str(&color.to_string());
  ansi_code.push_str("m");
  ansi_code.push_str("   ");
  ansi_code.push_str("\x1b[0m");
  return ansi_code;
}
