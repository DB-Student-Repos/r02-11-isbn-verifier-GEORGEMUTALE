pub fn is_valid_isbn(isbn: &str) -> bool {
    // Removes dashes 
    let isbn = isbn.replace("-", "");

    // Checks if for the correct length
    if isbn.len() != 10 {
        return false;
    }

    // Initializes a variable
    let mut total = 0;

    // invalid x 
    let mut invalid_x = false;

    // Iterate over each character in the ISBN
    for (i, c) in isbn.chars().enumerate() {
        // For the last character, if it's 'X', assign 10 to it, otherwise convert it to integer
        let char_value = if i == 9 {
            if c == 'X' {
                10
            } else {
                match c.to_digit(10) {
                    Some(digit) => digit,
                    None => return false, // If the last character is not a digit or 'X', return False
                }
            }
        } else {
            // If 'X' appears in any position other than the last one, mark it as invalid
            if c == 'X' {
                invalid_x = true;
            }

            // Convert characters to integers
            match c.to_digit(10) {
                Some(digit) => digit,
                None => return false, // If any character is not a digit, return False
            }
        };

        // Multiply the character value by its position in the ISBN and add it to sum
        total += char_value * (10 - i as u32);
    }

    // Check if the total is divisible by 11 and 'X' is not in an invalid position
    total % 11 == 0 && !invalid_x
}

