#include <stdexcept>
#include "lexer.h"

lexer::lexer(const source_file& file)
    : source(file.source), lexeme_start_location(location::empty(file.path))
{
}

token_stream lexer::tokenize()
{
    while (!is_eof())
        lex();

    return token_stream(tokens);
}

void lexer::lex()
{
    const char character = current_char();
    lexeme_start_location = current_location();
    advance();

    switch (character)
    {
    case '+': return push_token(syntax_kind::plus);
    case '-': return push_token(syntax_kind::minus);
    case '*': return push_token(syntax_kind::star);
    case '/': return push_token(syntax_kind::slash);
    case '(': return push_token(syntax_kind::r_paren);
    case ')': return push_token(syntax_kind::l_paren);
        
    case '\'':
    case '"': return read_string();
        
    default:
        {
            if (std::isspace(character))
                return skip_whitespace();

            if (std::isalpha(character))
                return read_identifier_or_keyword();

            if (std::isdigit(character) || character == '.')
                return read_number();
            
            throw std::runtime_error("Unexpected character '{}'");
        }
    }
}

void lexer::read_identifier_or_keyword()
{
    while (!is_eof() && (std::isalnum(current_char()) || current_char() == '_'))
        advance();

    std::string lexeme = current_lexeme();
    if (lexeme == "true" || lexeme == "false")
        return push_token(syntax_kind::bool_literal, lexeme == "true");

    if (lexeme == "null")
        return push_token(syntax_kind::null_literal);

    // TODO: keywords
    push_token(syntax_kind::identifier);
}

void lexer::read_string()
{
}

void lexer::read_number()
{
}

void lexer::skip_whitespace()
{
    while (!is_eof() && std::isspace(current_char()))
    {
        char current = current_char();
        if (current == '\n')
            advance_new_line();
        else
            advance();
    }

    lexeme_start_location = current_location();
}

void lexer::push_token(syntax_kind kind, const token::value_t& value)
{
    tokens.emplace_back(kind, current_span(), current_lexeme(), value);
}

std::string lexer::current_lexeme() const
{
    const span span = current_span();
    return source.substr(span.start.position, span.end.position - span.start.position);
}

span lexer::current_span() const
{
    const location end_location = current_location();
    return span(lexeme_start_location, end_location);
}

location lexer::current_location() const
{
    return location(lexeme_start_location.file_name, line, column, position);
}

char lexer::current_char() const
{
    return peek_char(0);
}

char lexer::peek_char(const size_t offset) const
{
    if (is_eof(offset))
        return '\0';

    return source[position + offset];
}

bool lexer::match_char(char expected)
{
    const bool is_match = !is_eof() && current_char() == expected;
    if (is_match)
        advance();

    return is_match;
}

void lexer::advance(const size_t amount)
{
    position += amount;
    column += amount;
}

void lexer::advance_new_line()
{
    position += 1;
    line += 1;
    column = 0;
}

bool lexer::is_eof(const size_t offset) const
{
    return position + offset >= source.length();
}
