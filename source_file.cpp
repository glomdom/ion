#include <fstream>
#include <sstream>
#include "source_file.h"

#include "lexer.h"

source_file::source_file(const std::string& file_path)
{
    std::ifstream input_file(file_path);
    if (!input_file)
        throw std::runtime_error("Could not open source file!");
    
    std::stringstream buffer;
    buffer << input_file.rdbuf();

    std::string contents = buffer.str();
    input_file.close();

    path = file_path;
    source = contents;
}

token_stream source_file::tokenize() const
{
    return token_stream(lexer(*this).tokenize());
}
