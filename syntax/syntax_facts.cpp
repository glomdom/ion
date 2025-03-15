#include "syntax_facts.h"

std::optional<syntax_kind> get_keyword_kind(const std::string& keyword)
{
    if (!keyword_map.has_value(keyword))
        return std::nullopt;
    
    return keyword_map.get_value(keyword);
}

std::optional<std::string> get_keyword_lexeme(const syntax_kind kind)
{
    if (!keyword_map.has_key(kind))
        return std::nullopt;
    
    return keyword_map.get_key(kind);
}