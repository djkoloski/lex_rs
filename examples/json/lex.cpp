enum TokenType {
    INVALID,
    TRUE,
    LIT_NULL,
    LBRACKET,
    WHITESPACE,
    NUMBER,
    FALSE,
    RBRACKET,
    LBRACE,
    STRING,
    RBRACE,
    COMMA,
    COLON
};

struct Token {
    const char *begin;
    const char *end;
    TokenType token_type;
};

Token lex(const char *&s) {
    Token result;
    result.begin = s;

    goto STATE_0;

    STATE_0:
    switch (*s++) {
    case 49: goto STATE_3;
    case 57: goto STATE_3;
    case 45: goto STATE_5;
    case 102: goto STATE_4;
    case 50: goto STATE_3;
    case 55: goto STATE_3;
    case 125: goto STATE_14;
    case 51: goto STATE_3;
    case 116: goto STATE_11;
    case 34: goto STATE_6;
    case 54: goto STATE_3;
    case 123: goto STATE_2;
    case 9: goto STATE_8;
    case 11: goto STATE_8;
    case 12: goto STATE_8;
    case 52: goto STATE_3;
    case 110: goto STATE_9;
    case 10: goto STATE_8;
    case 91: goto STATE_1;
    case 32: goto STATE_8;
    case 48: goto STATE_10;
    case 44: goto STATE_7;
    case 58: goto STATE_12;
    case 93: goto STATE_13;
    case 56: goto STATE_3;
    case 53: goto STATE_3;
    default: result.token_type = TokenType::INVALID; goto CLEANUP;
    }

    STATE_1:
    result.token_type = TokenType::LBRACKET; goto END;

    STATE_2:
    result.token_type = TokenType::LBRACE; goto END;

    STATE_3:
    switch (*s++) {
    case 56: goto STATE_3;
    case 101: goto STATE_18;
    case 57: goto STATE_3;
    case 54: goto STATE_3;
    case 52: goto STATE_3;
    case 48: goto STATE_3;
    case 46: goto STATE_17;
    case 51: goto STATE_3;
    case 69: goto STATE_18;
    case 49: goto STATE_3;
    case 55: goto STATE_3;
    case 50: goto STATE_3;
    case 53: goto STATE_3;
    default: result.token_type = TokenType::NUMBER; goto CLEANUP;
    }

    STATE_4:
    switch (*s++) {
    case 97: goto STATE_25;
    default: result.token_type = TokenType::INVALID; goto CLEANUP;
    }

    STATE_5:
    switch (*s++) {
    case 53: goto STATE_3;
    case 54: goto STATE_3;
    case 48: goto STATE_10;
    case 56: goto STATE_3;
    case 50: goto STATE_3;
    case 57: goto STATE_3;
    case 52: goto STATE_3;
    case 51: goto STATE_3;
    case 55: goto STATE_3;
    case 49: goto STATE_3;
    default: result.token_type = TokenType::INVALID; goto CLEANUP;
    }

    STATE_6:
    switch (*s++) {
    case 42: goto STATE_6;
    case 53: goto STATE_6;
    case 69: goto STATE_6;
    case 49: goto STATE_6;
    case 36: goto STATE_6;
    case 108: goto STATE_6;
    case 111: goto STATE_6;
    case 39: goto STATE_6;
    case 68: goto STATE_6;
    case 75: goto STATE_6;
    case 115: goto STATE_6;
    case 45: goto STATE_6;
    case 48: goto STATE_6;
    case 38: goto STATE_6;
    case 47: goto STATE_6;
    case 126: goto STATE_6;
    case 50: goto STATE_6;
    case 63: goto STATE_6;
    case 61: goto STATE_6;
    case 34: goto STATE_24;
    case 77: goto STATE_6;
    case 123: goto STATE_6;
    case 109: goto STATE_6;
    case 33: goto STATE_6;
    case 112: goto STATE_6;
    case 64: goto STATE_6;
    case 94: goto STATE_6;
    case 113: goto STATE_6;
    case 83: goto STATE_6;
    case 35: goto STATE_6;
    case 107: goto STATE_6;
    case 32: goto STATE_6;
    case 86: goto STATE_6;
    case 80: goto STATE_6;
    case 62: goto STATE_6;
    case 119: goto STATE_6;
    case 87: goto STATE_6;
    case 101: goto STATE_6;
    case 97: goto STATE_6;
    case 56: goto STATE_6;
    case 121: goto STATE_6;
    case 120: goto STATE_6;
    case 54: goto STATE_6;
    case 46: goto STATE_6;
    case 70: goto STATE_6;
    case 71: goto STATE_6;
    case 79: goto STATE_6;
    case 44: goto STATE_6;
    case 110: goto STATE_6;
    case 89: goto STATE_6;
    case 125: goto STATE_6;
    case 85: goto STATE_6;
    case 81: goto STATE_6;
    case 90: goto STATE_6;
    case 43: goto STATE_6;
    case 41: goto STATE_6;
    case 104: goto STATE_6;
    case 52: goto STATE_6;
    case 65: goto STATE_6;
    case 118: goto STATE_6;
    case 122: goto STATE_6;
    case 102: goto STATE_6;
    case 98: goto STATE_6;
    case 114: goto STATE_6;
    case 84: goto STATE_6;
    case 105: goto STATE_6;
    case 37: goto STATE_6;
    case 99: goto STATE_6;
    case 100: goto STATE_6;
    case 72: goto STATE_6;
    case 91: goto STATE_6;
    case 124: goto STATE_6;
    case 73: goto STATE_6;
    case 74: goto STATE_6;
    case 95: goto STATE_6;
    case 88: goto STATE_6;
    case 82: goto STATE_6;
    case 60: goto STATE_6;
    case 58: goto STATE_6;
    case 66: goto STATE_6;
    case 59: goto STATE_6;
    case 92: goto STATE_29;
    case 96: goto STATE_6;
    case 51: goto STATE_6;
    case 57: goto STATE_6;
    case 76: goto STATE_6;
    case 55: goto STATE_6;
    case 103: goto STATE_6;
    case 78: goto STATE_6;
    case 67: goto STATE_6;
    case 93: goto STATE_6;
    case 106: goto STATE_6;
    case 40: goto STATE_6;
    case 117: goto STATE_6;
    case 116: goto STATE_6;
    default: result.token_type = TokenType::INVALID; goto CLEANUP;
    }

    STATE_7:
    result.token_type = TokenType::COMMA; goto END;

    STATE_8:
    switch (*s++) {
    case 32: goto STATE_8;
    case 11: goto STATE_8;
    case 12: goto STATE_8;
    case 10: goto STATE_8;
    case 9: goto STATE_8;
    default: result.token_type = TokenType::WHITESPACE; goto CLEANUP;
    }

    STATE_9:
    switch (*s++) {
    case 117: goto STATE_21;
    default: result.token_type = TokenType::INVALID; goto CLEANUP;
    }

    STATE_10:
    switch (*s++) {
    case 69: goto STATE_18;
    case 46: goto STATE_17;
    case 101: goto STATE_18;
    default: result.token_type = TokenType::NUMBER; goto CLEANUP;
    }

    STATE_11:
    switch (*s++) {
    case 114: goto STATE_30;
    default: result.token_type = TokenType::INVALID; goto CLEANUP;
    }

    STATE_12:
    result.token_type = TokenType::COLON; goto END;

    STATE_13:
    result.token_type = TokenType::RBRACKET; goto END;

    STATE_14:
    result.token_type = TokenType::RBRACE; goto END;

    STATE_15:
    switch (*s++) {
    case 101: goto STATE_16;
    default: result.token_type = TokenType::INVALID; goto CLEANUP;
    }

    STATE_16:
    result.token_type = TokenType::TRUE; goto END;

    STATE_17:
    switch (*s++) {
    case 51: goto STATE_20;
    case 49: goto STATE_20;
    case 50: goto STATE_20;
    case 57: goto STATE_20;
    case 55: goto STATE_20;
    case 56: goto STATE_20;
    case 53: goto STATE_20;
    case 52: goto STATE_20;
    case 54: goto STATE_20;
    case 48: goto STATE_20;
    default: result.token_type = TokenType::INVALID; goto CLEANUP;
    }

    STATE_18:
    switch (*s++) {
    case 57: goto STATE_19;
    case 51: goto STATE_19;
    case 45: goto STATE_32;
    case 54: goto STATE_19;
    case 52: goto STATE_19;
    case 55: goto STATE_19;
    case 53: goto STATE_19;
    case 48: goto STATE_19;
    case 43: goto STATE_32;
    case 50: goto STATE_19;
    case 49: goto STATE_19;
    case 56: goto STATE_19;
    default: result.token_type = TokenType::INVALID; goto CLEANUP;
    }

    STATE_19:
    switch (*s++) {
    case 48: goto STATE_19;
    case 52: goto STATE_19;
    case 53: goto STATE_19;
    case 54: goto STATE_19;
    case 55: goto STATE_19;
    case 51: goto STATE_19;
    case 56: goto STATE_19;
    case 49: goto STATE_19;
    case 50: goto STATE_19;
    case 57: goto STATE_19;
    default: result.token_type = TokenType::NUMBER; goto CLEANUP;
    }

    STATE_20:
    switch (*s++) {
    case 52: goto STATE_20;
    case 69: goto STATE_18;
    case 55: goto STATE_20;
    case 50: goto STATE_20;
    case 49: goto STATE_20;
    case 57: goto STATE_20;
    case 56: goto STATE_20;
    case 101: goto STATE_18;
    case 51: goto STATE_20;
    case 48: goto STATE_20;
    case 53: goto STATE_20;
    case 54: goto STATE_20;
    default: result.token_type = TokenType::NUMBER; goto CLEANUP;
    }

    STATE_21:
    switch (*s++) {
    case 108: goto STATE_23;
    default: result.token_type = TokenType::INVALID; goto CLEANUP;
    }

    STATE_22:
    result.token_type = TokenType::LIT_NULL; goto END;

    STATE_23:
    switch (*s++) {
    case 108: goto STATE_22;
    default: result.token_type = TokenType::INVALID; goto CLEANUP;
    }

    STATE_24:
    result.token_type = TokenType::STRING; goto END;

    STATE_25:
    switch (*s++) {
    case 108: goto STATE_27;
    default: result.token_type = TokenType::INVALID; goto CLEANUP;
    }

    STATE_26:
    switch (*s++) {
    case 53: goto STATE_6;
    case 100: goto STATE_6;
    case 48: goto STATE_6;
    case 116: goto STATE_6;
    case 32: goto STATE_6;
    case 90: goto STATE_6;
    case 39: goto STATE_6;
    case 82: goto STATE_6;
    case 111: goto STATE_6;
    case 44: goto STATE_6;
    case 87: goto STATE_6;
    case 123: goto STATE_6;
    case 120: goto STATE_6;
    case 113: goto STATE_6;
    case 51: goto STATE_6;
    case 41: goto STATE_6;
    case 40: goto STATE_6;
    case 36: goto STATE_6;
    case 112: goto STATE_6;
    case 125: goto STATE_6;
    case 86: goto STATE_6;
    case 78: goto STATE_6;
    case 106: goto STATE_6;
    case 65: goto STATE_6;
    case 119: goto STATE_6;
    case 89: goto STATE_6;
    case 63: goto STATE_6;
    case 71: goto STATE_6;
    case 73: goto STATE_6;
    case 77: goto STATE_6;
    case 61: goto STATE_6;
    case 76: goto STATE_6;
    case 60: goto STATE_6;
    case 83: goto STATE_6;
    case 35: goto STATE_6;
    case 126: goto STATE_6;
    case 38: goto STATE_6;
    case 79: goto STATE_6;
    case 80: goto STATE_6;
    case 107: goto STATE_6;
    case 118: goto STATE_6;
    case 81: goto STATE_6;
    case 52: goto STATE_6;
    case 104: goto STATE_6;
    case 88: goto STATE_6;
    case 92: goto STATE_29;
    case 64: goto STATE_6;
    case 103: goto STATE_6;
    case 59: goto STATE_6;
    case 43: goto STATE_6;
    case 99: goto STATE_6;
    case 121: goto STATE_6;
    case 72: goto STATE_6;
    case 70: goto STATE_6;
    case 58: goto STATE_6;
    case 47: goto STATE_6;
    case 69: goto STATE_6;
    case 55: goto STATE_6;
    case 85: goto STATE_6;
    case 62: goto STATE_6;
    case 33: goto STATE_6;
    case 54: goto STATE_6;
    case 56: goto STATE_6;
    case 50: goto STATE_6;
    case 98: goto STATE_6;
    case 49: goto STATE_6;
    case 57: goto STATE_6;
    case 45: goto STATE_6;
    case 75: goto STATE_6;
    case 95: goto STATE_6;
    case 114: goto STATE_6;
    case 102: goto STATE_6;
    case 68: goto STATE_6;
    case 101: goto STATE_6;
    case 108: goto STATE_6;
    case 97: goto STATE_6;
    case 74: goto STATE_6;
    case 37: goto STATE_6;
    case 46: goto STATE_6;
    case 93: goto STATE_6;
    case 105: goto STATE_6;
    case 34: goto STATE_24;
    case 122: goto STATE_6;
    case 117: goto STATE_6;
    case 91: goto STATE_6;
    case 110: goto STATE_6;
    case 96: goto STATE_6;
    case 67: goto STATE_6;
    case 84: goto STATE_6;
    case 66: goto STATE_6;
    case 42: goto STATE_6;
    case 115: goto STATE_6;
    case 109: goto STATE_6;
    case 124: goto STATE_6;
    case 94: goto STATE_6;
    default: result.token_type = TokenType::STRING; goto CLEANUP;
    }

    STATE_27:
    switch (*s++) {
    case 115: goto STATE_31;
    default: result.token_type = TokenType::INVALID; goto CLEANUP;
    }

    STATE_28:
    result.token_type = TokenType::FALSE; goto END;

    STATE_29:
    switch (*s++) {
    case 116: goto STATE_6;
    case 59: goto STATE_6;
    case 33: goto STATE_6;
    case 72: goto STATE_6;
    case 89: goto STATE_6;
    case 104: goto STATE_6;
    case 125: goto STATE_6;
    case 56: goto STATE_6;
    case 93: goto STATE_6;
    case 84: goto STATE_6;
    case 94: goto STATE_6;
    case 44: goto STATE_6;
    case 97: goto STATE_6;
    case 34: goto STATE_26;
    case 60: goto STATE_6;
    case 118: goto STATE_6;
    case 80: goto STATE_6;
    case 79: goto STATE_6;
    case 57: goto STATE_6;
    case 54: goto STATE_6;
    case 126: goto STATE_6;
    case 111: goto STATE_6;
    case 68: goto STATE_6;
    case 105: goto STATE_6;
    case 114: goto STATE_6;
    case 81: goto STATE_6;
    case 75: goto STATE_6;
    case 50: goto STATE_6;
    case 87: goto STATE_6;
    case 61: goto STATE_6;
    case 90: goto STATE_6;
    case 101: goto STATE_6;
    case 71: goto STATE_6;
    case 51: goto STATE_6;
    case 123: goto STATE_6;
    case 36: goto STATE_6;
    case 78: goto STATE_6;
    case 73: goto STATE_6;
    case 69: goto STATE_6;
    case 85: goto STATE_6;
    case 95: goto STATE_6;
    case 35: goto STATE_6;
    case 98: goto STATE_6;
    case 109: goto STATE_6;
    case 115: goto STATE_6;
    case 102: goto STATE_6;
    case 37: goto STATE_6;
    case 64: goto STATE_6;
    case 82: goto STATE_6;
    case 113: goto STATE_6;
    case 49: goto STATE_6;
    case 99: goto STATE_6;
    case 110: goto STATE_6;
    case 41: goto STATE_6;
    case 66: goto STATE_6;
    case 42: goto STATE_6;
    case 117: goto STATE_6;
    case 83: goto STATE_6;
    case 58: goto STATE_6;
    case 62: goto STATE_6;
    case 106: goto STATE_6;
    case 46: goto STATE_6;
    case 65: goto STATE_6;
    case 63: goto STATE_6;
    case 48: goto STATE_6;
    case 53: goto STATE_6;
    case 112: goto STATE_6;
    case 91: goto STATE_6;
    case 119: goto STATE_6;
    case 121: goto STATE_6;
    case 122: goto STATE_6;
    case 86: goto STATE_6;
    case 96: goto STATE_6;
    case 52: goto STATE_6;
    case 120: goto STATE_6;
    case 88: goto STATE_6;
    case 107: goto STATE_6;
    case 45: goto STATE_6;
    case 39: goto STATE_6;
    case 43: goto STATE_6;
    case 47: goto STATE_6;
    case 40: goto STATE_6;
    case 74: goto STATE_6;
    case 124: goto STATE_6;
    case 38: goto STATE_6;
    case 32: goto STATE_6;
    case 77: goto STATE_6;
    case 103: goto STATE_6;
    case 92: goto STATE_29;
    case 100: goto STATE_6;
    case 76: goto STATE_6;
    case 67: goto STATE_6;
    case 108: goto STATE_6;
    case 55: goto STATE_6;
    case 70: goto STATE_6;
    default: result.token_type = TokenType::INVALID; goto CLEANUP;
    }

    STATE_30:
    switch (*s++) {
    case 117: goto STATE_15;
    default: result.token_type = TokenType::INVALID; goto CLEANUP;
    }

    STATE_31:
    switch (*s++) {
    case 101: goto STATE_28;
    default: result.token_type = TokenType::INVALID; goto CLEANUP;
    }

    STATE_32:
    switch (*s++) {
    case 57: goto STATE_19;
    case 51: goto STATE_19;
    case 52: goto STATE_19;
    case 49: goto STATE_19;
    case 54: goto STATE_19;
    case 53: goto STATE_19;
    case 48: goto STATE_19;
    case 50: goto STATE_19;
    case 55: goto STATE_19;
    case 56: goto STATE_19;
    default: result.token_type = TokenType::INVALID; goto CLEANUP;
    }

    CLEANUP:
    --s;
    END:
    result.end = s;
    return result;
}
