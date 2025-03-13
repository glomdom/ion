#pragma once
#include <string>

class location {
public:
    std::string file_name;
    size_t line;
    size_t column;
    size_t position;

    location(const std::string& file_name, size_t line, size_t column, size_t position);

    static location empty(const std::string& name = "?");
};
