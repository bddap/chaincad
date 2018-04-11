
use std::vec::Vec;
mod peekable;

enum Clip {
    Parent(Vec<Clip>),
    Child(String)
}

fn child<I>(mut iter: &peekable::Peekable<I>) -> String
where I: Iterator<Item = char>
{
    // let mut s = String::new();
    while let Some(c) = iter.peek() {
        if c != ' ' || c != ']' {
            break;
        }
    }
    "".to_owned()
}

fn clisp_parse(a: String) -> Clip {
    // let i = a.chars();

    // while let Some(c) = i.next() {
    //     match c {
    //         _ => 
    //     }
    // }
    
    // consume all " "
    // readUntil: " ", "[", "]"
    
    
    Clip::Child("adsf".to_owned())
}
