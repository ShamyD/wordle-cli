use std::io;
use std::collections::HashSet;
use colored::*;
use rand::Rng;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn format_output(output_character: &char, output_colour: &char) -> ColoredString{
    match output_colour {
        'r' => return output_character.to_string().red(),
        'y' => return output_character.to_string().yellow(),
        'g' => return output_character.to_string().green(),
        _ => output_character.to_string().purple().strikethrough()
    }
}


// fn format_output_string(guess_attempt: &String, guess_colours: &[char]) -> ColoredString{
//     let formated_string = String::new();

//     // let output_len = guess_colours.len();
//     // let mut output_string: String = "".to_string();
//     // for  i in 0..output_len{
//     //     output_string.push_str(format_output(guess_attempt.chars().nth(i).unwrap(), guess_colours[i]))
//     // }
//     // return output_string
// }



// fn format_output_string(guess_attempt: &String, guess_colours: &[char]) -> String {
//     let mut formatted_string = String::new();

//     for (i, c) in guess_attempt.chars().enumerate() {
//         let color = guess_colours.get(i).cloned().unwrap_or(' '); // Get the color or use a default space if color is not found
//         let colored_char = format_output(&c, &color);
//         formatted_string.push_str(&colored_char);
//     }

//     formatted_string
// }

fn main() {
    let possible_words: Vec<String> = read_lines("../other_utils/word_list.txt"); 
    let num = rand::thread_rng().gen_range(0..possible_words.len());
    let goal_word: String = possible_words[num].to_uppercase();//String::from("BABIES");//FIX
    let goal_word_letters: HashSet<char> = HashSet::from_iter(goal_word.chars());
    // println!("{}", goal_word);
    // let goal_word_letters = HashSet::from(goal_word.chars());
    // println!("{}",goal_word_letters);

    let word_length = goal_word.len();
    println!("Welcome! Guess the {}-letter word!", word_length);
    let marking_string: String = std::iter::repeat("[*]").take(word_length).collect();
    println!("{}", marking_string);
    let mut guess: String;// = String::new();
    let mut have_won: bool;// = true;
    println!("Make a {}-letter guess:", word_length);
    //Main loop of program
    loop{
        
        //Take intput and check whether it is valid
        loop{ 
            guess = String::from("");
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            guess = guess.trim().to_string().to_uppercase(); //trim() returns &str

            //println!("You guessed {}", guess);
            if guess.chars().count() == word_length {
                break
            } else {
                println!("Provided a word of length {} instead of {}", guess.chars().count(), word_length);
                // guess = String::from("");
            }
        }
        let mut count: usize = 0;
        let mut char_array: Vec<char> = std::iter::repeat('r').take(word_length).collect();//['r', word_length]; // TODO: make length dynamic //FIX
        for guess_char in guess.chars(){
            if goal_word_letters.contains(&guess_char){
                char_array[count] = 'y';
            }
            if goal_word.chars().nth(count).unwrap() == guess_char{ //unrwap extraxts from Some-value
                char_array[count] = 'g';
            }
            count += 1;
        }
        // println!("[{}][{}][{}][{}][{}]", format_output(&guess.chars().nth(0).unwrap(), &char_array[0]), format_output(&guess.chars().nth(1).unwrap(), &char_array[1]), char_array[2], char_array[3], char_array[4]);
        // println!("{}", format_output_string(&guess, &char_array));
        have_won = true;
        for i in 0..char_array.len(){
            if char_array[i] != 'g'{
                have_won = false;
            }
            print!("[{}]", format_output(&guess.chars().nth(i).unwrap(), &char_array[i]));
        }
        print!("\n");

        if have_won{
            println!("Congratulations! You were correct!");
            break
        }
        //Compare guess with goal_word:
        /*Make goal word into a set of characters? Then iterate through chars in guess. Check if in set and if same index in goal_word and assign values corresponding to colours in a vector. 
        Then use vector with guess to form output
        */
        
        
    }
}
