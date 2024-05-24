use std::error::Error;
use std::fs::File;
use std::io::{self, Read};


pub struct Canvas {
    pub images: Vec<String>,
}

impl Canvas {
    pub fn build(visuals_filepath: &'static str) -> Result<Self, Box<dyn Error>> {
        let mut filehandle = File::open(visuals_filepath)?;

        let mut contents = String::new();
        filehandle.read_to_string(&mut contents)?;

        let mut images: Vec<String> = Vec::new();
        let mut temp_image = String::new();
        for line in contents.lines() {
            if line == "%IMG_END%" {
                images.push(temp_image.clone());
                temp_image.clear();
                continue;
            }
            temp_image.push_str(&format!("\n{}", line));
        }

        Ok(Self { images })
    }

    pub fn draw(
        &mut self,
        message: &str,
        word_hidden: &Vec<char>,
        wrong_guesses_count: usize,
    ) -> Result<(), &'static str> {
        if wrong_guesses_count == self.images.len() {
            return Err("No more images to draw.");
        }
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", message);

        //No need to print image if user hasn't made any wrong guesses yet.
        if wrong_guesses_count > 0 {
            let image = &self.images[wrong_guesses_count - 1];
            println!("{}\n\n", image);
        }
        let word_hidden: String = word_hidden.iter().map(|ch| format!("{ch} ")).collect();
        println!("{}", word_hidden);

        Ok(())
    }
}
