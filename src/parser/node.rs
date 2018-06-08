use lexer::{Token, TokenType};

use super::*;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum NTermType {
    // Bit of a clusterfuck here;)
    // Taken from https://docs.oracle.com/javase/specs/jls/se7/html/jls-18.html
    Identifier, QualifiedIdentifier, QualifiedIdentifierList, CompilationUnit,
    ImportDeclaration, TypeDeclaration, ClassOrInterfaceDeclaration,
    ClassDeclaration, InterfaceDeclaration, NormalClassDeclaration, EnumDeclaration,
    NormalInterfaceDeclaration, AnnotationTypeDeclaration, Type, BasicType,
    ReferenceType, TypeArguments, TypeArgument, NonWildcardTypeArguments, TypeList,
    TypeArgumentsOrDiamond, NonWildcardTypeArgumentsOrDiamond, TypeParameters,
    TypeParameter, Bound, Modifier, Annotations, Annotation, AnnotationElement,
    ElementValuePairs, ElementValuePair, ElementValue, ElementValueArrayInitializer,
    ElementValues, ClassBody, ClassBodyDeclaration, MemberDecl, MethodOrFieldDecl,
    MethodOrFieldRest, FieldDeclaratorsRest, MethodDeclaratorRest,
    VoidMethodDeclaratorRest, ConstructorDeclaratorRest,
    GenericMethodOrConstructorDecl, GenericMethodOrConstructorRest, InterfaceBody,
    InterfaceBodyDeclaration, InterfaceMemberDecl, InterfaceMethodOrFieldDecl,
    InterfaceMethodOrFieldRest, ConstantDeclaratorsRest, ConstantDeclaratorRest,
    ConstantDeclarator, InterfaceMethodDeclaratorRest,
    VoidInterfaceMethodDeclaratorRest, InterfaceGenericMethodDecl, FormalParameters,
    FormalParameterDecls, VariableModifier, FormalParameterDeclsRest,
    VariableDeclaratorId, VariableDeclarators, VariableDeclarator,
    VariableDeclaratorRest, VariableInitializer, ArrayInitializer, Block,
    BlockStatements, BlockStatement, LocalVariableDeclarationStatement, Statement,
    StatementExpression, Catches, CatchClause, CatchType, Finally,
    ResourceSpecification, Resources, Resource, SwitchBlockStatementGroups,
    SwitchBlockStatementGroup, SwitchLabels, SwitchLabel, EnumConstantName,
    ForControl, ForVarControl, ForVarControlRest, ForVariableDeclaratorsRest,
    ForInit, ForUpdate, Expression, AssignmentOperator, Expression1,
    Expression1Rest, Expression2, Expression2Rest, InfixOp, Expression3, PrefixOp,
    PostfixOp, Primary, Literal, ParExpression, Arguments, SuperSuffix,
    ExplicitGenericInvocationSuffix, Creator, CreatedName, ClassCreatorRest,
    ArrayCreatorRest, IdentifierSuffix, ExplicitGenericInvocation, InnerCreator,
    Selector, EnumBody, EnumConstants, EnumConstant, EnumBodyDeclarations,
    AnnotationTypeBody, AnnotationTypeElementDeclarations,
    AnnotationTypeElementDeclaration, AnnotationTypeElementRest,
    AnnotationMethodOrConstantRest, AnnotationMethodRest,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum NodeType {
    NTerm(NTermType),
    Term(Token),
}

/// A parse tree node
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Node {
    /// node_type == Term(_) implies children.len() == 0
    pub node_type: NodeType,
    pub children: Vec<Node>
}

/// Helper function to create a terminal from a token
fn term(tok: Token) -> Node {
    Node {
        node_type: NodeType::Term(tok),
        children: Vec::new(),
    }
}

/// Returns the first token as a terminal given that its value matches the given
/// string. Consumes the token if it matches.
fn assert_term(tokens: &mut TokenIter, src: &str, expected: &str) -> ParseRes {
    let tok = tokens.clone().next();
    match tok {
        Some(tok) => if tok.val(src) == expected {
            Ok(term(*tokens.next().unwrap()))
        } else {
            Err(ParseErr::Point(format!("Expected {}, got {}", expected, tok.val(src)), *tok))
        }
        None => Err(ParseErr::Raw(format!("Expected {}, got EOF", expected))),
    }
}

/// Returns the first token as a terminal given that its value matches the given
/// string. Consumes the token if it matches.
fn assert_term_with_type(tokens: &mut TokenIter, expected: TokenType) -> ParseRes {
    let tok = tokens.clone().next();
    match tok {
        Some(tok) => if tok.token_type == expected {
            Ok(term(*tokens.next().unwrap()))
        } else {
            Err(ParseErr::Point(format!("Expected {:?}", expected), *tok))
        }
        None => Err(ParseErr::Raw(format!("Expected {:?}, got EOF", expected))),
    }
}
