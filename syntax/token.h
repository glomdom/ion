#pragma once
#include <string>
#include <variant>

#include "span.h"
#include "syntax_kind.h"

class token
{
    using value_t = std::variant<std::monostate, int, float, bool, std::string>;
    
public:
    syntax_kind kind;
    span span;
    std::string text;
    value_t value = NULL;

    token(syntax_kind kind, ::span span, std::string text);
    token(syntax_kind kind, ::span span, std::string text, std::string value);
    token(syntax_kind kind, ::span span, std::string text, bool value);
    token(syntax_kind kind, ::span span, std::string text, int value);
    token(syntax_kind kind, ::span span, std::string text, float value);
};