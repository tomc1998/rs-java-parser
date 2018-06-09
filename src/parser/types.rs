use super::*;
use lexer::TokenType;

#[allow(dead_code)]
fn parse_basic_type(tokens: &mut TokenIter, _src: &str) -> ParseRes {
    Ok(nterm(NTermType::BasicType, vec![term(*tokens.next().unwrap())]))
}

#[allow(dead_code)]
pub fn parse_type(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let children = match tokens.clone().next().ok_or(
        ParseErr::Raw("Unexpected EOF, expected type".to_owned()))? {
        tok if tok.val(src) == "byte" ||
            tok.val(src) == "int" ||
            tok.val(src) == "short" ||
            tok.val(src) == "char" ||
            tok.val(src) == "long" ||
            tok.val(src) == "float" ||
            tok.val(src) == "double" ||
            tok.val(src) == "boolean" => vec![parse_basic_type(tokens, src)?],
        _ => vec![parse_reference_type(tokens, src)?],
    };
    Ok(nterm(NTermType::Type, children))
}

fn parse_reference_type(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![assert_term_with_type(tokens, TokenType::Ident)?];
    if is_type_args_next(tokens, src) {
        children.push(parse_type_arguments(tokens, src)?);
    }
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "." {
            tokens.next(); // Skip '.'
            children.push(assert_term_with_type(tokens, TokenType::Ident)?);
            if is_type_args_next(tokens, src) {
                children.push(parse_type_arguments(tokens, src)?);
            }
        } else {
            break;
        }
    }
    Ok(nterm(NTermType::ReferenceType, children))
}

fn is_type_args_next(tokens: &TokenIter, src: &str) -> bool {
    match tokens.clone().next() {
        Some(tok) if tok.val(src) == "<" => true,
        _ => false
    }
}

pub fn parse_type_argument(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut clone = tokens.clone();
    Ok(nterm(NTermType::TypeArgument,
          match clone.next().ok_or(ParseErr::Raw("Unexpected EOF in type args".to_owned()))? {
              // ? extends T
              tok if tok.val(src) == "?" => match clone.next() {
                  Some(tok) if tok.val(src) == "super" || tok.val(src) == "extends" =>
                      vec![term(*tokens.next().unwrap()), term(*tokens.next().unwrap()),
                           parse_reference_type(tokens, src)?],
                  _ => vec![term(*tokens.next().unwrap())],
              }
              _ => vec![parse_reference_type(tokens, src)?]
          }))
}

#[allow(dead_code)]
pub fn parse_non_wildcard_type_arguments(tokens: &mut TokenIter, src: &str) -> ParseRes {
    Ok(nterm(NTermType::TypeArguments,
             vec![assert_term(tokens, src, "<")?,
                  parse_type_list(tokens, src)?,
                  assert_term(tokens, src, ">")?]))
}

pub fn parse_type_arguments(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![assert_term(tokens, src, "<")?,
                            parse_type_argument(tokens, src)?];
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "," {
            tokens.next(); // Skip ','
            children.push(parse_type_argument(tokens, src)?);
        } else {
            break;
        }
    }
    children.push(assert_term(tokens, src, ">")?);
    Ok(nterm(NTermType::TypeArguments, children))
}

#[allow(dead_code)]
pub fn parse_non_wildcard_type_arguments_or_diamond(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut clone = tokens.clone();
    Ok(nterm(NTermType::NonWildcardTypeArgumentsOrDiamond, match clone.next() {
        Some(tok) if tok.val(src) == "<" => match clone.next() {
            Some(tok) if tok.val(src) == ">" => vec![term(*tokens.next().unwrap()),
                                                     term(*tokens.next().unwrap())],
            _ => vec![parse_non_wildcard_type_arguments(tokens, src)?]
        }
        _ => vec![parse_non_wildcard_type_arguments(tokens, src)?],
    }))
}

#[allow(dead_code)]
pub fn parse_type_arguments_or_diamond(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut clone = tokens.clone();
    Ok(nterm(NTermType::TypeArgumentsOrDiamond, match clone.next() {
        Some(tok) if tok.val(src) == "<" => match clone.next() {
            Some(tok) if tok.val(src) == ">" => vec![term(*tokens.next().unwrap()),
                                                     term(*tokens.next().unwrap())],
            _ => vec![parse_type_arguments(tokens, src)?]
        }
        _ => vec![parse_type_arguments(tokens, src)?],
    }))
}

#[allow(dead_code)]
pub fn parse_type_list(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = vec![parse_reference_type(tokens, src)?];
    while let Some(tok) = tokens.clone().next() {
        if tok.val(src) == "," {
            tokens.next(); // Skip ','
            children.push(parse_reference_type(tokens, src)?);
        } else {
            break;
        }
    }
    Ok(nterm(NTermType::TypeList, children))
}

#[cfg(test)]
mod tests {
    use lexer::lex;
    use super::*;
    use super::node::NTermType;

    #[test]
    fn test_parse_type() {
        let src = "boolean";
        let node = parse_type(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 1);
        match node.children[0].node_type {
            NodeType::NTerm(NTermType::BasicType) => (),
            ref t => panic!("Incorrect nterm type: {:?}", t),
        }

        let src = "SomeReferenceType";
        let node = parse_type(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 1);
        match node.children[0].node_type {
            NodeType::NTerm(NTermType::ReferenceType) => (),
            ref t => panic!("Incorrect nterm type: {:?}", t),
        }
    }

    #[test]
    fn test_parse_type_argument() {
        let src = "? extends T<Bar>.Foo";
        let node = parse_type_argument(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 3);
        assert_eq!(node.children[0].val(src), Some("?"));
        assert_eq!(node.children[1].val(src), Some("extends"));
        match node.children[2].node_type {
            NodeType::NTerm(NTermType::ReferenceType) => (),
            ref t => panic!("Incorrect nterm type: {:?}", t),
        }

        let src = "T<Foo>.Bar";
        let node = parse_type_argument(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 1);
        match node.children[0].node_type {
            NodeType::NTerm(NTermType::ReferenceType) => (),
            ref t => panic!("Incorrect nterm type: {:?}", t),
        }
    }

    #[test]
    fn test_parse_reference_type() {
        let src = "T<Foo>.Bar";
        let node = parse_reference_type(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 3);
        assert_eq!(node.children[0].val(src).unwrap(), "T");
        match node.children[1].node_type {
            NodeType::NTerm(NTermType::TypeArguments) => (),
            ref t => panic!("Incorrect nterm type: {:?}", t),
        }
        assert_eq!(node.children[2].val(src).unwrap(), "Bar");
    }

    #[test]
    fn test_parse_type_arguments() {
        let src = "<T<Foo>.Bar, N<MyVar>, ? extends X>";
        let node = parse_type_arguments(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 5);

        let src = "<>";
        let node = parse_type_arguments_or_diamond(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 2);
    }

    #[test]
    fn test_parse_type_list() {
        let src = "U<Foo>, V<Bar>, MyClass";
        let node = parse_type_list(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 3);
    }

    #[test]
    fn test_parse_non_wildcard_type_arguments() {
        let src = "<U<Foo>, V<Bar>, MyClass>";
        let node = parse_non_wildcard_type_arguments(&mut lex(src, "").unwrap().iter(), src).unwrap();
        assert_eq!(node.children.len(), 3);

        let src = "<>";
        let node = parse_non_wildcard_type_arguments_or_diamond(&mut lex(src, "").unwrap().iter(),
                                                               src).unwrap();
        assert_eq!(node.children.len(), 2);
    }
}
