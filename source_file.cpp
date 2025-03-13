#include <fstream>
#include <sstream>
#include "source_file.h"

source_file::source_file(const std::string& file_path)
{
    std::ifstream input_file(file_path);
    if (!input_file)
        throw std::exception("Could not open source file!");
    
    std::stringstream buffer;
    buffer << input_file.rdbuf();

    std::string contents = buffer.str();
    input_file.close();

    path = file_path;
    source = contents;
}

token_stream source_file::tokenize()
{
    return token_stream(std::vector<token>());
}
