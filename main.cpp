#include <iostream>
#include <ostream>

#include "source_file.h"

int main(int argc, char* argv[])
{
    const auto file = source_file("test.ion");
    for (const auto token_stream = file.tokenize(); const auto& token : token_stream.tokens)
        std::cout << to_string(token) << '\n';
    
    return 0;
}
