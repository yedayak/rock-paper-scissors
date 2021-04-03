use std::iter::Peekable;

fn main() {
    let mut parser = TokenParser::new(
        "'yedaya' vs 'john' paper ~scIssors\nrock-rock 'jake'  vs  'percy' paper-paper rock-rock"
            .into(),
    );
    let tokens = parser.parse_tokens();
    println!("{:?}", tokens);
    let mut play_creator = PlayGenerator::new(&tokens);
    let plays = play_creator.create_plays();
    println!("{:?}", plays);
}

pub struct TokenParser {
    text: String,
    index: usize,
    tokens: Vec<Token>,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum TokenType {
    Move(Move),
    PlaySeparator,
    Rival,
    Vs,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    ttype: TokenType,
    text: String,
}

#[derive(Debug)]
pub struct Play {
    player1: String,
    player2: String,
    move1: Move,
    move2: Move,
}

impl Play {
    pub fn new(player1: String, player2: String, move1: Move, move2: Move) -> Self {
        Self {
            player1,
            player2,
            move1,
            move2,
        }
    }
}

fn can_be_in_move(ch: char) -> bool {
    ch.is_ascii_alphabetic()
}

fn is_duel_sign(ch: char) -> bool {
    ch == '-' || ch == '~'
}

impl TokenParser {
    pub fn new(text: String) -> Self {
        Self {
            text,
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
                    // Valid moves
                    "rock" => TokenType::Move(Move::Rock),
                    "paper" => TokenType::Move(Move::Paper),
                    "scissors" => TokenType::Move(Move::Scissors),
                    // Other possible tokens
                    "vs" => TokenType::Vs,
                    unrecognized_token => panic!("unrecognized token \"'{}\"!", unrecognized_token),
                };
                let token = Token {
                    text: new_token_text,
                    ttype: token_type,
                };
                self.tokens.push(token)
            } else if is_duel_sign(ch) {
                println!("is rival separator");
                let consumed = self.consume().unwrap();
                self.tokens.push(Token {
                    text: String::from(consumed),
                    ttype: TokenType::PlaySeparator,
                })
            } else if ch.is_whitespace() {
                let mut possible_whitespace = self.peek();
                while possible_whitespace.is_some() && possible_whitespace.unwrap().is_whitespace()
                {
                    self.consume();
                    possible_whitespace = self.peek();
                }
            } else if ch == '\'' {
                // Getting the name in quotes (Without the quotes..)
                let mut rival_name = String::new();
                // quote
                self.consume();
                while self.peek().is_some() && self.peek().unwrap() != '\'' {
                    rival_name.push(self.consume().unwrap());
                }
                if self.peek().is_none() {
                    panic!("Unclosed single quotes: Rival name must be inclosed in single quotes")
                }
                // Getting the closing quote
                self.consume();
                self.tokens.push(Token {
                    text: rival_name,
                    ttype: TokenType::Rival,
                });
            } else {
                panic!("Unrecognized characters");
            }
            possbile_ch = self.peek();
        }
        self.tokens.to_vec()
    }

    // TODO: maybe change peek() and consume() to use a stored iterator
    // instead of recreating one each time
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

pub struct PlayGenerator<'a> {
    token_iterator: Peekable<std::slice::Iter<'a, Token>>,
}

impl<'a> PlayGenerator<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            token_iterator: tokens.iter().peekable(),
        }
    }

    pub fn create_plays(&mut self) -> Vec<Play> {
        let mut plays = Vec::<Play>::new();
        let mut current_player1 = None;
        let mut current_player2 = None;
        while self.token_iterator.peek().is_some() {
            let token = self.token_iterator.next().unwrap();
            match token.ttype {
                TokenType::Rival => {
                    current_player1 = Some(&token.text);
                    if self.token_iterator.next().unwrap().ttype == TokenType::Vs {
                        current_player2 = Some(&self.token_iterator.next().unwrap().text);
                    }
                }
                TokenType::Move(move1) => {
                    if self
                        .token_iterator
                        .next()
                        .expect("Tokens ended unexpectedly")
                        .ttype
                        != TokenType::PlaySeparator
                    {
                        panic!(
                            "No play separator between each move: use ~ or - between your plays."
                        );
                    }
                    if let TokenType::Move(move2) = self
                        .token_iterator
                        .next()
                        .expect("Only one play found - You can't play against yourself ;}")
                        .ttype
                    {
                        plays.push(Play::new(
                            current_player1.unwrap().to_string(),
                            current_player2.unwrap().to_string(),
                            move1,
                            move2,
                        ));
                    }
                    println!("{:?}", move1)
                }
                TokenType::PlaySeparator => {}
                TokenType::Vs => {}
            }
        }
        plays
    }
}
