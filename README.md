An implementation of the Hangman game.

The program expects two files to be present and accessible in your working directory:  
	- visuals.txt containing a sequence of hangman "images". Each image must end with %IMG_END% to mark the end of each individual image.  
	- words.txt with one word per line. This file serves as the input for randomly selecting a word.  

Both files can already be found in the root of this project, but feel free to download a more suitable and extensive words file from the internet in order for you to actually enjoy the game.

Build and run the binary with  
$ cargo run
