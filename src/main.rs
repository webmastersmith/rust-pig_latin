fn main() {
    let s = "Tuesday is the Friday of Monday";
    let m = s.split_whitespace();
    let vowels = ['a','e','i','o','u'];
    
    for word in m {
        let mut word_vec: Vec<char> = word.chars().collect();
        let f_letter = &word_vec.remove(0);
        let first_letter: char = *f_letter;
        let mut word = String::from("");
        let mut is_vowel: bool = false;

        //check if first letter is a vowel
        for vowel in vowels {
            if first_letter == vowel {
                is_vowel = true;
                word = word_vec.clone().into_iter().collect::<String>();
                word = format!("{}{}-hay", first_letter, word);
                break;
            };
        }
        if !is_vowel {
            word = word_vec.clone().into_iter().collect();
            word = format!("{}-{}ay", word, first_letter);
        }
        dbg!(word);        
    }
    

}
