#pragma once
#include <cstdint>

enum class syntax_kind : uint8_t
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

    identifier,
    let_keyword,
    fn_keyword,

    string_literal, // literals
    int_literal,
    float_literal,
    bool_literal,
    null_literal,
};
