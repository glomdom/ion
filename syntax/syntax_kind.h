#pragma once
#include <cstdint>
#include <format>

enum class syntax_kind : uint16_t
{
    plus, // operators
    minus,
    star,
    slash,
    slash_slash,
    percent,
    carat,
    tilde,
    ampersand,
    pipe,
    ampersand_ampersand,
    pipe_pipe,
    bang,
    lt,
    lte,
    gt,
    gte,
    equals,
    equals_equals,
    bang_equals,
    colon,
    l_paren,
    r_paren,
    l_brace,
    r_brace,
    l_bracket,
    r_bracket,

    identifier,
    let_keyword,
    fn_keyword,

    string_literal, // literals
    int_literal,
    float_literal,
    bool_literal,
    null_literal,
};

inline std::string to_string(const syntax_kind kind)
{
    switch (kind)
    {
    case syntax_kind::plus: return "plus";
    case syntax_kind::minus: return "minus";
    case syntax_kind::star: return "star";
    case syntax_kind::slash: return "slash";
    case syntax_kind::slash_slash: return "slash_slash";
    case syntax_kind::percent: return "percent";
    case syntax_kind::carat: return "carat";
    case syntax_kind::tilde: return "tilde";
    case syntax_kind::ampersand: return "ampersand";
    case syntax_kind::pipe: return "pipe";
    case syntax_kind::ampersand_ampersand: return "ampersand_ampersand";
    case syntax_kind::pipe_pipe: return "pipe_pipe";
    case syntax_kind::bang: return "bang";
    case syntax_kind::lt: return "lt";
    case syntax_kind::lte: return "lte";
    case syntax_kind::gt: return "gt";
    case syntax_kind::gte: return "gte";
    case syntax_kind::equals: return "equals";
    case syntax_kind::equals_equals: return "equals_equals";
    case syntax_kind::bang_equals: return "bang_equals";
    case syntax_kind::colon: return "colon";
    case syntax_kind::l_paren: return "l_paren";
    case syntax_kind::r_paren: return "r_paren";
    case syntax_kind::l_brace: return "l_brace";
    case syntax_kind::r_brace: return "r_brace";
    case syntax_kind::l_bracket: return "l_bracket";
    case syntax_kind::r_bracket: return "r_bracket";
    case syntax_kind::identifier: return "identifier";
    case syntax_kind::let_keyword: return "let_keyword";
    case syntax_kind::fn_keyword: return "fn_keyword";
    case syntax_kind::string_literal: return "string_literal";
    case syntax_kind::int_literal: return "int_literal";
    case syntax_kind::float_literal: return "float_literal";
    case syntax_kind::bool_literal: return "bool_literal";
    case syntax_kind::null_literal: return "null_literal";
    default: return "unknown";
    }
}

template <>
struct std::formatter<syntax_kind> : std::formatter<std::string_view> {
    auto format(const syntax_kind kind, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(to_string(kind), ctx);
    }
};
