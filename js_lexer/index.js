let lexer = {"dfa":{"states":[{"edges":{"r":1,"2":2,"p":1,"u":1,"1":2,"K":1,"z":1,"A":1,"L":1,"0":3,"G":1,"Y":1,"I":1,"a":1,"f":1,"_":1,"b":1,"\t":4,"E":1,"j":1,"M":1,"s":1,"i":1,"4":2,"8":2,"t":1,"R":1,"m":1,"v":1,"l":1,"D":1," ":4,"B":1,"X":1,"3":2,"h":1,"q":1,"S":1,"e":1,"o":1,"g":1,"H":1,"w":1,"y":1,"U":1,"k":1,"5":2,"P":1,"O":1,"W":1,"c":1,"d":1,"J":1,"n":1,"x":1,"T":1,"6":2,"V":1,"9":2,"F":1,"C":1,"Z":1,"Q":1,"N":1,"7":2}},{"edges":{"D":1,"A":1,"R":1,"l":1,"h":1,"H":1,"7":1,"8":1,"x":1,"B":1,"_":1,"u":1,"O":1,"S":1,"C":1,"K":1,"y":1,"E":1,"k":1,"6":1,"X":1,"J":1,"9":1,"i":1,"3":1,"5":1,"a":1,"c":1,"1":1,"o":1,"V":1,"q":1,"t":1,"I":1,"W":1,"w":1,"r":1,"m":1,"2":1,"4":1,"Z":1,"d":1,"N":1,"L":1,"Y":1,"F":1,"v":1,"j":1,"P":1,"z":1,"f":1,"b":1,"Q":1,"G":1,"n":1,"s":1,"M":1,"p":1,"0":1,"e":1,"U":1,"T":1,"g":1}},{"edges":{"1":2,"9":2,"8":2,"2":2,"5":2,"7":2,"0":2,"3":2,"6":2,"4":2}},{"edges":{}},{"edges":{"\t":4," ":4}}]},"state_to_token":{"1":"Identifier","3":"Number","2":"Number","0":"Whitespace","4":"Whitespace"},"start":0}

function lex(text) {
    let text_index = 0
    let current_state = lexer.start
    let result = []

    while (text_index < text.length) {
        while (true) {
            let next_state = lexer.dfa.states[current_state].edges[text[text_index]]
            if (next_state !== undefined) {
                current_state = next_state
                ++text_index
            } else {
                break
            }
        }
        let token = lexer.state_to_token[current_state]
        if (token !== undefined) {
            result.push(token)
        } else {
            return undefined
        }
        current_state = lexer.start
    }

    return result
}