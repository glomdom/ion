#include "token.h"

token::token(syntax_kind kind, ::span span, std::string text)
    : kind(kind), span(span), text(text)
{
}

token::token(syntax_kind kind, ::span span, std::string text, std::string value)
    : kind(kind), span(span), text(text), value(value)
{
}

token::token(syntax_kind kind, ::span span, std::string text, bool value)
    : kind(kind), span(span), text(text), value(value)
{
}

token::token(syntax_kind kind, ::span span, std::string text, int value)
    : kind(kind), span(span), text(text), value(value)
{
}

token::token(syntax_kind kind, ::span span, std::string text, float value)
    : kind(kind), span(span), text(text), value(value)
{
}
