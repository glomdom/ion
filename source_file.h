#pragma once
#include <string>

#include "syntax/token_stream.h"

class source_file
{
public:
    std::string path;
    std::string source;

    explicit source_file(const std::string& file_path);

    static token_stream tokenize();
    // static expression parse();
};
