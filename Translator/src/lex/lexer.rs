
pub enum Token {
    LP, RP, LC, RC, LB, RB,

    Id {id: String},
    IntLit {i: i32}, DoubleLit{ f: f64}, CharLit {c: u8}, 

    // LongLit{l: i64}, FloatLit{ f: f32}, ShortLit {i: i16},

    Dot, Semicolon, Comma, Ampersand,

    Plus, Minus, Star, Divide, Mod,
    Equal, NotEqual, Bigger, Lesser,
    BiggerOrEqual, LesserOrEqual,

    Assign, 
    True, False,
    Increment, Decrement,
    
    And, Or, Negation,

    If, Else, For, While, 
    Break, Continue, Return,
    
    // LongT, FloatT, ShortT,
    IntT, DoubleT, CharT, VoidT
}



pub fn lexical_analysis(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chr = s.chars().peekable();

    while let Some(ch) = chr.next() {
        if ch.is_whitespace() { continue; }
        match ch {
            '(' => tokens.push(Token::LP),
            ')' => tokens.push(Token::RP),
            '{' => tokens.push(Token::LC),
            '}' => tokens.push(Token::RC),
            '[' => tokens.push(Token::LB),
            ']' => tokens.push(Token::RB),
            ';' => tokens.push(Token::RB),
            ',' => tokens.push(Token::Comma),
            '.' => tokens.push(Token::Dot),

            _ if ch.is_alphabetic() || ch == '_' => {
                let mut lex = String::new();
                lex.push(ch);
                while let Some(&temp_ch) = chr.peek() {
                    match temp_ch {
                        _ if temp_ch.is_alphabetic() || temp_ch == '_' => lex.push(temp_ch),
                        _ => break
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
                    // "long" => tokens.push(Token::LongT),
                    "int" => tokens.push(Token::IntT),
                    // "short" => tokens.push(Token::ShortT),
                    "double" => tokens.push(Token::DoubleT),
                    // "float" => tokens.push(Token::FloatT),
                    "char" => tokens.push(Token::CharT),
                    "void" => tokens.push(Token::VoidT),
                    "true" => tokens.push(Token::True),
                    "false" => tokens.push(Token::False),
                    _ => tokens.push(Token::Id{id: lex}),
                }
            },
            _ if ch.is_numeric() => {
                let mut points: u8 = 0;
                let mut lex = String::new(); 
                lex.push(ch);
                while let Some(&temp_ch) = chr.peek() {
                    match temp_ch {
                        '.' => {
                            points += 1; 
                            lex.push(temp_ch);
                            
                            if points > 1 { 
                                panic!(); 
                            }
                        },
                        _ if temp_ch.is_numeric() => lex.push(temp_ch),
                        _ => break
                    } chr.next();
                }

                match points  { 
                    0 => match lex.parse::<i32>() {
                        Ok(num) => tokens.push(Token::IntLit { i: num }),
                        Err(_) => panic!(),
                    },
                    1 => match lex.parse::<f64>() {
                        Ok(num) => tokens.push(Token::DoubleLit { f: num }),
                        Err(_) => panic!(),
                    },
                    _ => panic!()
                }
            }



            _ => panic!(),
        }
    }
    return tokens;
}