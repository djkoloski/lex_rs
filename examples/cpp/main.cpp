#include <string>
#include <fstream>
#include <streambuf>

#include "lex.cpp"

int main(int argc, char **argv) {
    std::ifstream t(argv[1]);
    std::string str((std::istreambuf_iterator<char>(t)), std::istreambuf_iterator<char>());

    const char *ptr = str.c_str();
    while (*ptr) {
        Token token = lex(ptr);
        printf("Token %i: %.*s\n", token.token_type, token.end - token.begin, token.begin);
    }
}