//! Integer To Roman 
//! https://leetcode.com/problems/integer-to-roman/description/
//! Roman numerals are formed by appending the conversions of decimal place values from highest to lowest. 
//! Converting a decimal place value into a Roman numeral has the following rules:
//! 1. If the value does not start with 4 or 9, select the symbol of the maximal value 
//!     that can be subtracted from the input, append that symbol to the result, subtract its value, 
//!     and convert the remainder to a Roman numeral.
//! 2. If the value starts with 4 or 9 use the subtractive form representing one symbol subtracted from 
//!     the following symbol, 
//!     for example, 4 is 1 (I) less than 5 (V): IV and 9 is 1 (I) less than 10 (X): IX. 
//!     Only the following subtractive forms are used: 4 (IV), 9 (IX), 40 (XL), 90 (XC), 400 (CD) and 900 (CM).
//! 3. Only powers of 10 (I, X, C, M) can be appended consecutively at most 3 times to represent multiples of 10. \
//!     You cannot append 5 (V), 50 (L), or 500 (D) multiple times. 
//!     If you need to append a symbol 4 times use the subtractive form.




fn get_max_substract(n: u32) -> (char, u32) {
    match n {
        1..=4           => ('I', 1),
        5..=9           => ('V', 5),        
        10..=49         => ('X', 10),
        50..=99         => ('L', 50),
        100..=499       => ('C', 100),
        500..=999       => ('D', 500),
        _               => ('M', 1000)
    }
}

fn get_next_max(n: u32) -> (char, u32) {
    match n {        
        4               => ('V', 5),        
        9               => ('X', 10),
        40..=49              => ('L', 50),
        90..=99              => ('C', 100),
        400..=499             => ('D', 500),
        900..=999             => ('M', 1000),
        _ => unreachable!("not max for n: {}", n)
    }
}


pub fn int_to_roman(num: i32) -> String {
    let mut num: u32 = num as u32;
    let mut partial_res: String = "".to_owned();
    let mut res: String = "".to_owned();
    // 445 = CDXLV
    
    let mut i = 0u32;
    loop {
        if num == 0 {
            break;
        }        
        let mut x = num % (10 * 10u32.pow(i));
        // starting num h -> examples 
        // 400  -> h = 4 
        //   5  -> h = 5
        // 1000 -> h = 1
        let h = x / (10u32.pow(i));        
        //println!("num: {} i: {} pow {} - x: {}, h: {}",num, i, 10 * 10u32.pow(i), x, h);
        num = num - x;
        while x > 0 {   
            if h == 4 || h == 9 {                
                let (sym1, val1) = get_next_max(x);                
                let (sym2, val2) = get_max_substract(val1 - x);                                
                partial_res.push(sym2);
                partial_res.push(sym1);                
                
                x = 0;
            } else {
                let (sym, val) = get_max_substract(x);
                x = x - val;
                partial_res.push(sym);     
            }
            //println!("res: {}", res);        
        }

        res.extend(partial_res.chars().rev());
        partial_res = "".to_owned();
    
        i += 1;
    }
    
    res.chars().rev().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let result = int_to_roman(445);
        assert!(result == "CDXLV");

        let result = int_to_roman(505);
        assert!(result == "DV");        

        let result = int_to_roman(58);
        assert!(result == "LVIII");
        

        let result = int_to_roman(3749);
        assert!(result == "MMMDCCXLIX");

    }
}
