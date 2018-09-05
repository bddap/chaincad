use std::fmt;
use std::vec::Vec;

#[derive(Debug, PartialEq)]
pub enum Clip<'a> {
    Parent(Vec<Clip<'a>>),
    Child(&'a [char]),
}

impl<'a> Clip<'a> {
    pub fn from_slice<'b>(source: &'b [char]) -> Result<Clip<'b>, ()> {
        let (ret, rest) = Self::parse_clip(source)?;
        if rest.iter().filter(|c| !c.is_whitespace()).next() == None {
            Ok(ret)
        } else {
            Err(())
        }
    }

    fn parse_clip<'b>(source: &'b [char]) -> Result<(Clip<'b>, &[char]), ()> {
        if let Some(c) = source.iter().next() {
            let rest = &source[1..];
            Ok(match c {
                '[' => {
                    let mut v: Vec<Clip> = vec![];
                    let rest = Clip::parse_parent(&mut v, rest)?;
                    (Clip::Parent(v), rest)
                }
                c if c.is_whitespace() => Clip::parse_clip(rest)?,
                _ => Clip::parse_child(source)?,
            })
        } else {
            Err(())
        }
    }

    fn parse_parent<'b>(v: &mut Vec<Clip<'b>>, source: &'b [char]) -> Result<&'b [char], ()> {
        if let Some(c) = source.iter().next() {
            let rest = &source[1..];
            match c {
                ']' => Ok(rest),
                c if c.is_whitespace() => Clip::parse_parent(v, rest),
                _ => {
                    let (clip, rest) = Clip::parse_clip(source)?;
                    v.push(clip);
                    Clip::parse_parent(v, rest)
                }
            }
        } else {
            Err(())
        }
    }

    fn parse_child<'b>(source: &'b [char]) -> Result<(Clip<'b>, &[char]), ()> {
        for i in 0..(source.len()) {
            if source[i].is_whitespace() || source[i] == ']' {
                let (word, rest) = source.split_at(i);
                return Ok((Clip::Child(word), rest));
            }
        }
        return Ok((Clip::Child(source), &[]));
    }
}

impl<'a> fmt::Display for Clip<'a> {
    fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use std::fmt::Write;
        match self {
            Clip::Parent(children) => {
                fm.write_char('[')?;
                let mut chil = children.iter();
                if let Some(child) = chil.next() {
                    child.fmt(fm)?;
                    for child in chil {
                        fm.write_char(' ')?;
                        child.fmt(fm)?;
                    }
                }
                fm.write_char(']')?;
            }
            Clip::Child(strs) => for &c in strs.iter() {
                fm.write_char(c)?;
            },
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            Clip::from_slice(&"[asdf asdf]".chars().collect::<Vec<char>>()[..]),
            Ok(Clip::Parent(vec![
                Clip::Child(&"asdf".chars().collect::<Vec<char>>()),
                Clip::Child(&"asdf".chars().collect::<Vec<char>>()),
            ]))
        );
    }

    #[test]
    fn parse2() {
        assert_eq!(
            Clip::from_slice(&"[asdf [asdf fdsa] fdsa]".chars().collect::<Vec<char>>()[..]),
            Ok(Clip::Parent(vec![
                Clip::Child(&"asdf".chars().collect::<Vec<char>>()),
                Clip::Parent(vec![
                    Clip::Child(&"asdf".chars().collect::<Vec<char>>()),
                    Clip::Child(&"fdsa".chars().collect::<Vec<char>>()),
                ]),
                Clip::Child(&"fdsa".chars().collect::<Vec<char>>()),
            ]))
        );
    }

    #[test]
    fn parse3() {
        assert_eq!(
            Clip::from_slice(&"[asdf [asdf fdsa] fdsa".chars().collect::<Vec<char>>()[..]),
            Err(())
        );
    }

    #[test]
    fn parse4() {
        assert_eq!(
            Clip::from_slice(&"[asdf [asdf fdsa] fdsa]]".chars().collect::<Vec<char>>()[..]),
            Err(())
        );
    }

    #[test]
    fn parse5() {
        assert_eq!(
            Clip::from_slice(&"[asdf [asdf fdsa] fdsa] ".chars().collect::<Vec<char>>()[..]),
            Ok(Clip::Parent(vec![
                Clip::Child(&"asdf".chars().collect::<Vec<char>>()),
                Clip::Parent(vec![
                    Clip::Child(&"asdf".chars().collect::<Vec<char>>()),
                    Clip::Child(&"fdsa".chars().collect::<Vec<char>>()),
                ]),
                Clip::Child(&"fdsa".chars().collect::<Vec<char>>()),
            ]))
        );
    }

    #[test]
    fn parse6() {
        assert_eq!(
            Clip::from_slice(&"[asdf [asdf fdsa] fdsa] a".chars().collect::<Vec<char>>()[..]),
            Err(())
        );
    }

    #[test]
    fn parse7() {
        assert_eq!(
            Clip::from_slice(&"[]".chars().collect::<Vec<char>>()[..]),
            Ok(Clip::Parent(vec![]))
        );
    }

    #[test]
    fn parse8() {
        assert_eq!(
            Clip::from_slice(&"asdf".chars().collect::<Vec<char>>()[..]),
            Ok(Clip::Child(&"asdf".chars().collect::<Vec<char>>()))
        );
    }
}
