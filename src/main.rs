fn main() {
    let mut parser = Parser::new("paper ~scIssors\nrock-rock".into());
    println!("{:?}", parser.parse_tokens());
}

pub struct Parser {
    text: String,
    index: usize,
    tokens: Vec<Token>,
}

#[derive(Clone, Debug)]
pub enum TokenType {
    Scissors,
    Rock,
    Paper,
    Duel,
    Rivals,
    Whitespace,
}

#[derive(Clone, Debug)]
pub struct Token {
    r#type: TokenType,
    text: String,
}

fn can_be_in_move(ch: char) -> bool {
    ch.is_ascii_alphabetic()
}

fn is_duel_sign(ch: char) -> bool {
    ch == '-' || ch == '~'
}

impl Parser {
    pub fn new(text: String) -> Self {
        Self {
            text: text,
            index: 0,
            tokens: Vec::new(),
        }
    }

    fn parse_tokens(&mut self) -> Vec<Token> {
        // let tokens: &mut Vec<Token> = &mut Vec::new();
        let mut possbile_ch = self.peek();
        println!("{:?}", possbile_ch);
        while possbile_ch.is_some() {
            let ch = possbile_ch.unwrap();
            println!("Checking: {}", ch);
            if can_be_in_move(ch) {
                let mut new_token_text = self.consume().unwrap().to_string();
                let mut next_token_ch = self.peek();
                while next_token_ch.is_some() && can_be_in_move(next_token_ch.unwrap()) {
                    new_token_text.push(self.consume().unwrap());
                    next_token_ch = self.peek();
                }
                let normalized_token_text = new_token_text.to_lowercase();
                let token_type = match normalized_token_text.as_str() {
                    "rock" => TokenType::Rock,
                    "paper" => TokenType::Paper,
                    "scissors" => TokenType::Scissors,
                    unrecognized_token => panic!("unrecognized token {}!", unrecognized_token),
                };
                let token = Token {
                    text: new_token_text,
                    r#type: token_type,
                };
                self.tokens.push(token)
            } else if is_duel_sign(ch) {
                println!("is rival separator");
                let consumed = self.consume().unwrap();
                self.tokens.push(Token {
                    text: String::from(consumed),
                    r#type: TokenType::Duel,
                })
            } else if ch.is_whitespace() {
                let mut new_token_text = String::new();
                let mut next_token_ch = self.peek();
                while next_token_ch.is_some() && next_token_ch.unwrap().is_whitespace() {
                    new_token_text.push(self.consume().unwrap());
                    next_token_ch = self.peek();
                }
                self.tokens.push(Token {
                    text: new_token_text,
                    r#type: TokenType::Whitespace,
                });
            } else {
                panic!("Unrecognized characters");
            }
            possbile_ch = self.peek();
        }
        self.tokens.to_vec()
    }

    fn peek(&self) -> Option<char> {
        self.text.chars().nth(self.index)
    }

    fn consume(&mut self) -> Option<char> {
        self.index += 1;
        self.text.chars().nth(self.index - 1)
    }

    pub fn raw_text(&self) -> String {
        self.text.clone()
    }
}
