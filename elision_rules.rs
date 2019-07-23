/* main.rs 
LIFETIMES: Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.
- We must annotate lifetimes when the lifetimes of references could be related in a few different ways. 
- Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.
- The compiler has a borrow checker that compares scopes to determine whether all borrows are valid.    */ 

// 1) Generic lifetimes of parameters and return values in the context of functions     ------------------------------------------------------------------------- 
fn main() {

    let string1= String::from ("abcd");      // slice of a String 
    let string2= "xyz";                     // string literal

    let result= longest_a (string1.as_str(), string2);        // the fn shall take string slices (&str), which are references, so the longest fn doesn't take ownership of its parameters. 
    println!("The longest string is {}", result);
    
    
/* 
fn longest (x: &str, y: &str) -> &str {             // the compiler expects a lifetime parameter for the return value     
    if x.len() > y.len() {                          // this fn's return type contains a borrowed value, but the signature doesn't say whether it is borrowed from `x` or `y`
        x
    } else {
        y
    }
}

The return type needs a generic lifetime parameter on it because Rust can’t tell whether the reference being returned refers to x or y.
The borrow checker doesn’t know how the lifetimes of x and y relate to the lifetime of the return value. 
=> To fix this error, we add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis.

// 2) Lifetime annotation syntax    -----------------------------------------------------------------------------------------------------------------------------
- To describe the relationships of the lifetimes of multiple references to each other without affecting the lifetime. 
- fn can accept references with any lifetime by specifying a generic lifetime parameter 
- Lifetime parameters: names must start with ' and go after the & of a reference, using a space to separate the annotation from the reference's type. 

E.g.
    &i32            reference without lifetime parameter 
    &'a i32         reference with an explicit lifetime named 'a 
    &'a mut i32     mutable reference with an explicit lifetime 

- Lifetime annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other.  */ 

// 3) Function signatures with lifetime annotations     --------------------------------------------------------------------------------------------------------- 

// E.g. all the references in the parameters and the return value have the same lifetime: 
fn longest_a <'a> (x: &'a str, y: &'a str) -> &'a str {         // specifying that all the references live at least as long as lifetime 'a - the borrow checker rejects any values that don't adhere to these constraint. 
    if x.len() > y.len() {                                      // when passing concrete references into the fn, the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y. 
        x                                                       // the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y (idem for the returned reference).
    } else {
        y
    }
}

// 4) Passing in references that have different concrete lifetimes      -----------------------------------------------------------------------------------------

    let s1= String::from ("long string is long");           // s1 is valid until the end of the outer scope
    {
        let s2= String::from ("xyz");                   // s2 is valid until the end of the inner scope 
        let result= longest_a (s1.as_str(), s2.as_str());       // result references something that is valid until the end of the inner scope
        println! ("the longest string is {}", result); 
    }

    /* Lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. 
    Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.     */ 

/* 5) Lifetime annotations in Struct definitions    -------------------------------------------------------------------------------------------------------------
- It's possible for structs to hold references but they need to have lifetimes.     */ 

struct ImportantExcerpt <'a> {          // this annotation means an instance of the struct can't outlive the reference it holds in its part field. 
    part: &'a str, 
}

    let novel= String::from ("Some piece of novel. Blabla...");
    let first_sentence= novel.split (".")
        .next()
        .expect ("Could not find a '.'"); 
    let i= ImportantExcerpt {part: first_sentence};         // this instance can't outlive the variable it references (novel)

/* 6) Lifetime Elision rules    ---------------------------------------------------------------------------------------------------------------------------------
- Deterministic patterns programmed into Rust's analysis of references so you don't need to always write the lifetimes explicitly.

    *) Input lifetimes: those on function or method parameters. 
    *) Output lifetimes: those on return values. 
    
    Three rules used by the compiler to figure out what lifetimes references have when there aren't explicit annotations (these rules apply to fn definitions and impl blocks): 
        - 1st rule (on input lifetimes): each parameter that is a reference gets its own lifetime parameter (a fn gets as many lifetime parameters as parameters it has) => fn foo <'a, 'b> (x: &'a i32, y: &'b)
        - 2nd rule (on output lifetimes): if there's exactly one input lifetime parameter, that one is assigned to all output lifetime parameters => fn foo <'a> (x: &'a i32) -> &'a i32 
        - 3rd rule (on output lifetimes): if there's a &self or &mut self because it's a method definition, then the lifetime of self is assigned to all output lifetime parameters. 
    
    Examples:            
        
     1-     fn first_word (s: &str) -> &str {...}
        
        This is what the compiler does:
        - 1st rule:     fn first_word <'a> (s: &'a str) -> &str {...}
        - 2nd rule:     fn first_word <'a> (s: &'a str) -> &'a str {...}      // now all the references in the fn signature have lifetimes and the compiler can continue without explicit annotations.

     2-     fn longest (x: &str, y: &str) -> &str {...}

        - 1st rule:     fn longest <'a, 'b> (x: &'a str, y: &'b str) -> &str {...}
        - 2nd rule: it doesn't apply because there is more than one input lifetime. 
        - 3rd rule: it doesn't apply because longest is a fn rather than a method. 

        => The compiler has no way of figuring out the lifetime of the return value and thus it throws an error.    */

// 7) Lifetime annotations in method definitions    ------------------------------------------------------------------------------------------------------------- 

impl <'a> ImportantExcerpt <'a> {       // the lifetime 'a is part of the struct's type so this type of annotation is required. 
    fn level (&self) -> i32 {
        3
    }
}

    // - 3rd rule example: there are 2 input parameters, thus 2 lifetime parameters, and since one of them is &self, the return type gets the lifetime of &self.
        impl <'a> ImportantExcerpt <'a> {
            fn announce_and_return_part (&self, announcement: &str) -> &str {
                println! ("Attention please: {}", announcement); 
                self.part
            }
        } 

/* 8) The static lifetime   ------------------------------------------------------------------------------------------------------------------------------------- 
- 'static means that the reference can live for the entire duration of the program (stored directly in its binary). E.g.: string literals.  */ 

    let s: &'static str= "I have a static lifetime"; 
}
