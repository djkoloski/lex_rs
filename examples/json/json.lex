LBRACE      {
RBRACE      }
LBRACKET    \[
RBRACKET    ]
COMMA       ,
COLON       :
TRUE        true
FALSE       false
LIT_NULL    null
NUMBER      ([0-9]|[1-9][0-9]+|-[0-9]|-[1-9][0-9]+)(\.[0-9]+)?([eE][+-]?[0-9]+)?
STRING      "([ -!#-\]\]-~]|\\["\\bfnrt/]|\\u[0-9a-f][0-9a-f][0-9a-f][0-9a-f])*"
WHITESPACE  [ \t\v\n\f]+