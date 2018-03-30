
use std::vec::Vec;
mod peekable;

enum Clip {
    Parent(Vec<Clip>),
    Child(String)
}

// fn child(a: String) -> String {
    
// }

fn clisp_parse(a: String) -> Clip {
    let i = a.chars();

    // while let Some(c) = i.next() {
    //     match c {
    //         _ => 
    //     }
    // }
    
    // consume all " "
    // readUntil: " ", "[", "]"
    
    
    Clip::Child("adsf".to_owned())
}
