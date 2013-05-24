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
pub fn qs_score(str1 : &str, abbreviation: &str, offset : i16) -> f32 {
    if str::len(abbreviation) == 0 { return 0.9f32; }
    if str::len(abbreviation) > str::len(str1) { return 0.0f32; }

    for uint::range(0, str::len(abbreviation) - 1) |i| {

    }

    0.0f32
}