/* To remove duplication of code - write code without repetition that works in many different situations -
- Generic type parameters T let you apply the code to different types -
- Traits and trait bounds, like Display, ensure that even though the types are generic, they’ll have the behavior the code needs - 
- Generic lifetime parameters or lifetime annotations ensure that this flexible code won’t have any dangling references -   */ 

fn main() {
use std::fmt::Display; 

fn longest_with_an_announcement <'a, T> (x: &'a str, y: &'a str, ann: T) -> &'a str         // lifetimes are a type of generic, thus their declarations go in the same list <>
    where T: Display                                                                         // T can be of any type that implements the Display trait
{
    println! ("Announcement: {}", ann); 
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let ann= "it can be any type that implements the Display trait!"; 

    let result = longest_with_an_announcement(string1.as_str(), string2, ann);
    println!("The longest string is {}.", result);
}

// And all of this analysis happens at compile time, which doesn’t affect runtime performance!






