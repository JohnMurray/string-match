/**
 * Public: Using the QuickSilver algorithm, determine the match between two
 *         strings. Algorithm orginally ported from Lachie Cox's implementation
 *         for jQuery.
 * 
 * str1 -         The string that is being compared _against_ (the baseline 
 *                string). If this were called in an OO fashion, it would look
 *                like:
 *                    str1.qs_match(abbreviation, offset);
 * abbreviation - The string that is being checked for match-ness to str1
 * offset -       The point in the string from which the match checking(?) will
 *                start, typically zero.
 *
 * Notes: QS jQuery: http://static.railstips.org/orderedlist/demos/quicksilverjs/javascripts/quicksilver.js
 *
 * Returns a f32 
 */
pub fn qs_score(str1 : &str, abbreviation: &str, offset : uint) -> f32 {
    if abbreviation.len() == 0 { return 0.9f32; }
    if abbreviation.len() > str1.len() { return 0.0f32; }

    let mut i = abbreviation.len() + 1;
    while i > 0 {
        i = i - 1;

        let sub_abbreviation : &str = abbreviation.substr(0, i);
        let index : i16 = match str::find_str(str1, sub_abbreviation) {
            Some(n)        => n as i16,
            None           => -1i16
        };

        if index < 0 { loop; }
        let uindex : uint = index as uint;
        if (uindex + abbreviation.len()) > str1.len() + offset { loop; }

        let next_string = str1.substr(uindex + sub_abbreviation.len(), 
                                      str1.len());
        let next_abbreviation : &str;

        if i > abbreviation.len() {
            next_abbreviation = &"";
        }
        else {
            next_abbreviation = abbreviation.substr(i, abbreviation.len());
        }

        let remaining_score = qs_score(next_string, next_abbreviation, 
                                       offset + uindex);

        if remaining_score > 0f32 {
            let mut score : f32 = (str1.len() - next_string.len()) as f32;

            if index != 0 {
                let mut j : i16 ;

                let mut c : u8 = str1.to_bytes()[index - 1];
                if c == 32u8 || c == 9u8 {
                    j = index - 1;
                    while j >= 0 {
                        j = j - 1;
                        c = str1.to_bytes()[j];
                        if c == 32u8 || c == 9u8 { score -= 1f32; }
                        else { score -= 0.15; }
                    }
                }
                else {
                    score -= index as f32;
                }
            }

            score += remaining_score * (next_string.len() as f32);
            score /= str1.len() as f32;
            return score;
        }
    }

    0.0f32
}