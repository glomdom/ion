#pragma once
#include <string>
#include <variant>
#include <format>

#include "span.h"
#include "syntax_kind.h"

class token
{
public:
    using value_t = std::variant<std::monostate, int, float, bool, std::string>;
    
    syntax_kind kind;
    span span;
    std::string text;
    value_t value = NULL;

    token(syntax_kind kind, ::span span, std::string text);
    token(syntax_kind kind, ::span span, std::string text, value_t value);
    token(syntax_kind kind, ::span span, std::string text, std::string value);
    token(syntax_kind kind, ::span span, std::string text, bool value);
    token(syntax_kind kind, ::span span, std::string text, int value);
    token(syntax_kind kind, ::span span, std::string text, float value);
};

inline std::string to_string(token token)
{
    return std::format("{}: {} {}    {}", token.kind, token.text, NULL, token.span);
}

template <>
struct std::formatter<token> : std::formatter<std::string_view> {
    auto format(const token& token, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(to_string(token), ctx);
    }
};
