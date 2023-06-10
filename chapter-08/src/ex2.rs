pub mod ex2 {

fn pig_latin(phrase: &str) -> String {

    fn is_starts_with_vowel(word: &str, vowels: &[char]) -> bool {
    //     for vowel in vowels {
    //         if word.starts_with(*vowel) {
    //             return true;
    //         }
    //     }
    //     return false;
        return vowels.contains(&word.chars().next().unwrap());
    }

    // if phrase.len() == 0 {
    if phrase.is_empty() {
        return "".to_string();
    }

    let mut local_org = String::from(phrase).to_lowercase();

    if local_org.ends_with('.') {
        local_org.pop();
    }

    let mut pigged_phrase = String::new(); 

    let vowel_arr: [char; 5] = ['a','e','i','o','u'];

    for word in local_org.split_whitespace() {
        let mut new_word: String = String::new();

        if is_starts_with_vowel(word, &vowel_arr) {
            //move nothing, add hay to end
            new_word.push_str(&format!("{word}hay"));

        } else {
            //move first char to end 
            let first_char: char = word.chars().next().unwrap();
            let rest_of_word = word.get(1..).unwrap_or("");
            let pigged_word = format!("{rest_of_word}{first_char}ay");
            new_word.push_str(&pigged_word);           
        }
        
        pigged_phrase.push_str(&format!("{new_word} "));
    }

    pigged_phrase.pop();
    
    if phrase.ends_with('.') {
        pigged_phrase.push('.');
    }

    return pigged_phrase;
}

pub fn solve_ex2() -> () {
    
    println!("\n>> Chapter 8, exercise 2: Convert strings to pig latin. <<");
    println!(">> Encoded pig latin phrases are returned in lowercase without dashes. \n<<");

    let str1 = "Pig Latin is hard to speak anyway.";
    let str2 = "";

    println!("Original String: {}", str1);
    assert_eq!(pig_latin(str1), "igpay atinlay ishay ardhay otay peaksay anywayhay.");
    println!("In Pig Latin: {}\n", pig_latin(str1));

    println!("Original String: {}", str2);
    assert_eq!(pig_latin(str2), "");
    println!("In Pig Latin: {}\n", pig_latin(str2));
}

}