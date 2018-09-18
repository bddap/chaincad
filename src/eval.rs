use clip::Clip;

#[derive(Clone, Debug, PartialEq)]
enum DeSugaredClip<'a> {
    Parent(Vec<DeSugaredClip<'a>>),
    Child(&'a [char]),
}

impl<'a> DeSugaredClip<'a> {
    fn desugar<'b>(src: &'b Clip) -> DeSugaredClip<'b> {
        match src {
            Clip::Child(c) => DeSugaredClip::Child(c), // '.' at the top level will result in '.'
            Clip::Parent(v) => DeSugaredClip::desugar_parent(&v),
        }
    }

    fn desugar_parent<'b>(parent: &'b [Clip]) -> DeSugaredClip<'b> {
        let mut out: Vec<DeSugaredClip> = Vec::new();
        let mut rest = parent;
        while let Some((head, r)) = rest.split_first() {
            rest = r;
            match head {
                Clip::Child(['.']) => {
                    let last_child = DeSugaredClip::desugar_parent(rest);
                    out.push(last_child);
                    return DeSugaredClip::Parent(out);
                }
                Clip::Child(_) | Clip::Parent(_) => out.push(DeSugaredClip::desugar(head)),
            }
        }
        DeSugaredClip::Parent(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clip::Clip;

    #[test]
    fn desugar() {
        let asdf = &['a', 's', 'd', 'f'];
        let consume = &['.'];
        let clip = Clip::Parent(vec![
            Clip::Child(asdf),
            Clip::Child(consume),
            Clip::Child(asdf),
        ]);
        let desugared = DeSugaredClip::desugar(&clip);
        assert_eq!(
            desugared,
            DeSugaredClip::Parent(vec![
                DeSugaredClip::Child(asdf),
                DeSugaredClip::Parent(vec![DeSugaredClip::Child(asdf)]),
            ])
        );
    }
}
