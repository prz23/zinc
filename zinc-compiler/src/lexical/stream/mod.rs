//!
//! The lexical token stream.
//!

pub mod comment;
pub mod integer;
pub mod string;
pub mod symbol;
pub mod word;

use std::collections::VecDeque;

use crate::lexical::error::Error;
use crate::lexical::token::lexeme::identifier::Identifier;
use crate::lexical::token::lexeme::literal::string::String as StringLiteral;
use crate::lexical::token::lexeme::literal::Literal;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::location::Location;
use crate::lexical::token::Token;

use self::comment::Error as CommentParserError;
use self::integer::Error as IntegerParserError;
use self::string::Error as StringParserError;
use self::symbol::Error as SymbolParserError;

pub struct TokenStream {
    input: String,
    offset: usize,
    location: Location,
    look_ahead: VecDeque<Token>,
}

impl TokenStream {
    const DEQUE_LOOK_AHEAD_INITIAL_CAPACITY: usize = 16;

    pub fn new(input: String) -> Self {
        Self {
            input,
            offset: 0,
            location: Location::new_beginning(),
            look_ahead: VecDeque::with_capacity(Self::DEQUE_LOOK_AHEAD_INITIAL_CAPACITY),
        }
    }

    pub fn next(&mut self) -> Result<Token, Error> {
        let token = match self.look_ahead.pop_front() {
            Some(token) => token,
            None => self.advance()?,
        };
        log::debug!("{:?}", token);
        Ok(token)
    }

    ///
    /// Advances the iterator until there is `distance` elements in the look-ahead queue.
    /// Is used where there is a need to resolve an ambiguity like
    /// ```
    /// let identifier = true;
    /// if identifier {
    ///     // value: 42, a structure literal field
    ///     let value = 42; // a statement within the block
    /// }
    /// ```
    /// where `identifier` can be both a variable or structure literal type name.
    ///
    pub fn look_ahead(&mut self, distance: usize) -> Result<&Token, Error> {
        while self.look_ahead.len() < distance {
            let token = self.advance()?;
            self.look_ahead.push_back(token);
        }
        Ok(&self.look_ahead[self.look_ahead.len() - 1])
    }

    ///
    /// The function algorithm checks if the character:
    /// 1. Is contained within the alphabet
    /// 2. Is a whitespace
    /// 3. Starts a comment
    /// 4. Starts a string literal
    /// 5. Starts a symbol (operator or delimiter)
    /// 6. Starts a number (decimal or hexadecimal)
    /// 7. Starts a word (keyword, boolean literal, or identifier)
    /// 8. Panics if non of the above, thus the alphabet must contain all the characters being
    /// passed to subscanners
    ///
    fn advance(&mut self) -> Result<Token, Error> {
        while let Some(character) = self.input.chars().nth(self.offset) {
            if character.is_ascii_whitespace() {
                if character == '\n' {
                    self.location.line += 1;
                    self.location.column = 1;
                } else if character != '\r' {
                    self.location.column += 1;
                }
                self.offset += 1;
                continue;
            }

            if character == '/' {
                match self::comment::parse(&self.input[self.offset..]) {
                    Ok((size, lines, column, _comment)) => {
                        self.location.line += lines;
                        self.location.column = column;
                        self.offset += size;
                        continue;
                    }
                    Err(CommentParserError::UnexpectedEnd) => {
                        let location = Location::new(self.location.line, self.location.column);
                        return Err(Error::UnexpectedEnd(location));
                    }
                    Err(CommentParserError::NotAComment) => {}
                }
            }

            if character == '\"' {
                match self::string::parse(&self.input[self.offset..]) {
                    Ok((size, value)) => {
                        let location = Location::new(self.location.line, self.location.column);
                        self.location.column += size;
                        self.offset += size;
                        return Ok(Token::new(
                            Lexeme::Literal(Literal::String(StringLiteral::new(value))),
                            location,
                        ));
                    }
                    Err(StringParserError::UnexpectedEnd) => {
                        let location = Location::new(self.location.line, self.location.column);
                        return Err(Error::UnexpectedEnd(location));
                    }
                    Err(StringParserError::NotAString) => {}
                }
            }

            if character.is_ascii_digit() {
                match self::integer::parse(&self.input[self.offset..]) {
                    Ok((size, integer)) => {
                        let location = Location::new(self.location.line, self.location.column);
                        self.location.column += size;
                        self.offset += size;
                        return Ok(Token::new(
                            Lexeme::Literal(Literal::Integer(integer)),
                            location,
                        ));
                    }
                    Err(IntegerParserError::UnexpectedEnd) => {
                        let location = Location::new(self.location.line, self.location.column);
                        return Err(Error::UnexpectedEnd(location));
                    }
                    Err(IntegerParserError::NotAnInteger) => {}
                    Err(error) => {
                        let location = Location::new(self.location.line, self.location.column);
                        return Err(Error::InvalidInteger(location, error));
                    }
                }
            }

            if Identifier::can_start_with(character) {
                return match self::word::parse(&self.input[self.offset..]) {
                    Ok((size, lexeme)) => {
                        let location = Location::new(self.location.line, self.location.column);
                        self.location.column += size;
                        self.offset += size;
                        Ok(Token::new(lexeme, location))
                    }
                    Err(error) => {
                        let location = Location::new(self.location.line, self.location.column);
                        Err(Error::InvalidWord(location, error))
                    }
                };
            }

            match self::symbol::parse(&self.input[self.offset..]) {
                Ok((size, symbol)) => {
                    let location = Location::new(self.location.line, self.location.column);
                    self.location.column += size;
                    self.offset += size;
                    return Ok(Token::new(Lexeme::Symbol(symbol), location));
                }
                Err(SymbolParserError::UnexpectedEnd) => {
                    let location = Location::new(self.location.line, self.location.column);
                    return Err(Error::UnexpectedEnd(location));
                }
                Err(SymbolParserError::NotASymbol) => {}
                Err(error) => {
                    let location = Location::new(self.location.line, self.location.column);
                    return Err(Error::InvalidSymbol(location, error));
                }
            }
        }

        Ok(Token::new(Lexeme::Eof, self.location))
    }
}
