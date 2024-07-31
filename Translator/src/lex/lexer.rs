#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
//  [a-zA-Z_][a-zA-Z0-9_]*
    Id {id: String},
//  [0-9]*           [0-9]*.[0-9]*       '[\]+[a-zA-Z0-9]'* 
    IntLit {i: i32}, DoubleLit{ f: f64}, CharLit {c: u8}, 
//  (   )   {   }   [   ]
    LP, RP, LC, RC, LB, RB,
//   .       ;        ,
    Dot, Semicolon, Comma, 
//    +     -     *     /    %
    Plus, Minus, Star, Div, Mod,
//      +=         -=        *=        /=         %=
    PlusEqual, MinusEqual, StarEqual, DivEqual, ModEqual,
//   ==      !=         >       <
    Equal, NotEqual, Bigger, Lesser,
//      >=              <=
    BiggerOrEqual, LesserOrEqual,
//     =
    Assignment, 
//  true  false
    True, False,
//      ++         --
    Increment, Decrement,
//   &&  ||     !
    And, Or, Negation,
//       &          |
    BitwiseAnd, BitwiseOr,
//         &=               |=   
    BitwiseAndEqual, BitwiseOrEqual,   
//  if  else  for  while
    If, Else, For, While,
//  break  continue  return 
    Break, Continue, Return,
//  int   doble    char   void
    IntT, DoubleT, CharT, VoidT
}

pub fn lexical_analysis(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chr = s.chars().peekable();

    while let Some(ch) = chr.next() {
        if ch.is_whitespace() || ch == '\n' { continue; }
        match ch {
            '(' => tokens.push(Token::LP),
            ')' => tokens.push(Token::RP),
            '{' => tokens.push(Token::LC),
            '}' => tokens.push(Token::RC),
            '[' => tokens.push(Token::LB),
            ']' => tokens.push(Token::RB),
            ';' => tokens.push(Token::Semicolon),
            ',' => tokens.push(Token::Comma),
            '.' => tokens.push(Token::Dot),
            '>' => match chr.peek() {
                Some(&'=') => {
                    tokens.push(Token::BiggerOrEqual);
                    chr.next();
                },
                _ => tokens.push(Token::Bigger),
            },
            '<' => match chr.peek() {
                Some(&'=') => {
                    tokens.push(Token::LesserOrEqual);
                    chr.next();
                },
                _ => tokens.push(Token::Lesser),
            },
            '=' => match chr.peek() {
                Some(&'=') => {
                    tokens.push(Token::Equal);
                    chr.next();
                },
                _ => tokens.push(Token::Assignment),
            },
            '!' => match chr.peek() {
                Some(&'=') => {
                    tokens.push(Token::NotEqual);
                    chr.next();
                },
                _ => tokens.push(Token::Negation),
            },
            '+' => match chr.peek() {
                Some(&'=') => {
                    tokens.push(Token::PlusEqual);
                    chr.next();
                },
                Some(&'+') => {
                    tokens.push(Token::Increment);
                    chr.next();
                },
                _ => tokens.push(Token::Plus),
            },
            '-' => match chr.peek() {
                Some(&'=') => {
                    tokens.push(Token::MinusEqual);
                    chr.next();
                },
                Some(&'-') => {
                    tokens.push(Token::Decrement);
                    chr.next();
                },
                _ => tokens.push(Token::Minus),
            },
            '*' => match chr.peek() {
                Some(&'=') => {
                    tokens.push(Token::StarEqual);
                    chr.next();
                },
                _ => tokens.push(Token::Star),
            },
            '/' => match chr.peek() {
                Some(&'=') => {
                    tokens.push(Token::DivEqual);
                    chr.next();
                },
                Some(&'/') => {
                    while Some(&'\n') != chr.peek() {
                        chr.next();
                    }
                    chr.next();
                },
                _ => tokens.push(Token::Div),
            },
            '%' => match chr.peek() {
                Some(&'=') => {
                    tokens.push(Token::ModEqual);
                    chr.next();
                },
                _ => tokens.push(Token::Mod),
            },
            '&' => match chr.peek() {
                Some(&'=') => {
                    tokens.push(Token::BitwiseAndEqual);
                    chr.next();
                },
                Some(&'&') => {
                    tokens.push(Token::And);
                    chr.next();
                },
                _ => tokens.push(Token::BitwiseAnd),
            },
            '|' => match chr.peek() {
                Some(&'=') => {
                    tokens.push(Token::BitwiseOrEqual);
                    chr.next();
                },
                Some(&'|') => {
                    tokens.push(Token::Or);
                    chr.next();
                },
                _ => tokens.push(Token::BitwiseOr),
            },
            '\'' => {
                let temp_ch;
                match chr.next() {
                    Some('\\') => match chr.next() {
                        Some('\'') => panic!(),
                        Some(x) => temp_ch = x,
                        _ => panic!()
                    },
                    Some(x) => temp_ch = x,
                    _ => panic!()
                }

                match chr.next() {
                    Some('\'') => tokens.push(Token::CharLit { c: temp_ch as u8 }),
                    _ => panic!()
                }
            },
            _ if ch.is_alphabetic() || ch == '_' => {
                let mut lex = String::new();
                lex.push(ch);
                while let Some(&temp_ch) = chr.peek() {
                    match temp_ch {
                        _ if temp_ch.is_alphabetic() || temp_ch.is_numeric() || temp_ch == '_' => lex.push(temp_ch),
                        _ => break,
                    } chr.next();
                }

                match lex.as_str() {
                    "if" => tokens.push(Token::If),
                    "elif" => {tokens.push(Token::Else); tokens.push(Token::If)},
                    "else" => tokens.push(Token::Else),
                    "for" => tokens.push(Token::For),
                    "while" => tokens.push(Token::While),
                    "break" => tokens.push(Token::Break),
                    "continue" => tokens.push(Token::Continue),
                    "return" => tokens.push(Token::Return),
                    "int" => tokens.push(Token::IntT),
                    "double" => tokens.push(Token::DoubleT),
                    "char" => tokens.push(Token::CharT),
                    "void" => tokens.push(Token::VoidT),
                    "true" => tokens.push(Token::True),
                    "false" => tokens.push(Token::False),
                    _ => tokens.push(Token::Id{id: lex}),
                }
            },
            _ if ch.is_numeric() => {
                let mut points: bool = false;
                let mut lex = String::new(); 
                lex.push(ch);
                while let Some(&temp_ch) = chr.peek() {
                    match temp_ch {
                        _ if temp_ch == '.' => {
                            if points {
                                panic!();
                            }
                            lex.push(temp_ch);
                            points = true;
                        },
                        _ if temp_ch.is_numeric() => lex.push(temp_ch),
                        _ => break
                    } chr.next();
                }

                if !points {
                    match lex.parse::<i32>() {
                        Ok(num) => tokens.push(Token::IntLit { i: num }),
                        Err(_) => panic!(),
                    }
                } else { 
                    match lex.parse::<f64>() {
                        Ok(num) => tokens.push(Token::DoubleLit { f: num }),
                        Err(_) => panic!(),
                    }
                }
            },
            _ => panic!(),
        }
    }
    return tokens;
}