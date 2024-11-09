const command_input_element = document.getElementById('command_input');
const command_history_element = document.getElementById('command_history');
let command_history = [''];
let command_history_index = 0;
let token_index = 0;
const bindings = {};
const state = {};
const keywords = {
    'true': true,
    'false': false,
    'pi': Math.PI,
    'PI': Math.PI,
    'e': Math.E
}

let tokens;

const adjust_input_element_height = function () {
    let num_lines = command_input_element.value.split(/\n/).length;
    command_input_element.setAttribute('style', 'height: ' + num_lines + 'em');
    if (num_lines > 1) {
        command_input_element.setAttribute('class', 'multiline');
    } else {
        command_input_element.setAttribute('class', 'single_line');
    }
}

// command_input_element.onkeypress = function handle_key_input(event) {
//     if (event.key === 'Enter') {
//         event.preventDefault();
//     }
// }

command_input_element.onkeyup = function handle_key_input(event) {
    adjust_input_element_height();
    if (event.key === 'c' && event.ctrlKey) {
        command_input_element.value = '';
    }
    if (event.key === 'ArrowUp' && !event.shiftKey) {
        if (command_history_index > -1) {
            command_input_element.value = command_history[command_history_index];
            if (command_history_index > 0) {
                command_history_index -= 1;
            }
        }
    }
    if (event.key === 'ArrowDown' && !event.shiftKey) {
        if (command_history_index < command_history.length - 1) {
            command_history_index += 1;
            command_input_element.value = command_history[command_history_index];
        } else {
            command_input_element.value = '';
        }
    }
    if (event.key === 'Enter') {
        handle_enter();
    }
};

const handle_enter = function () {
    let command = command_input_element.value.trim();
    command_input_element.value = '';
    adjust_input_element_height();

    if (command.length > 0) {
        command_history_element.innerText += command + "\n";
        command_input_element.value = '';
        command_history_index = command_history.length;

        scan(command);
        let statement = parse();
        let value = evaluate(statement);

        if (value !== undefined) {
            let binding;
            if (value.is_binding) {                         // if it's declaration work with the initializer
                binding = value.name;                       // but we also need the name of the bound variable
                value = state[value.name];                     // lookup the value for the binding
            }

            // if (value.is_visual) {
            //
            // } else {
            //     if (binding && bindings[binding].previous && bindings[binding].previous.is_visual) {
            //         label(bindings[binding].previous, '@' + bindings[binding].previous.id);
            //     }
            // }
            // if (value.description) {
            //     value = value.description;
            // }
            command_history_element.innerText += value.toString() + "\n";
            command_history.push(command);
            command_history_element.scrollTo(0, command_history_element.scrollHeight);
        }

    }
}

const evaluate = function (expr) {
    switch (expr.type) {
        case 'declaration': {
            let value = evaluate(expr.initializer);
            let binding_name = expr.var_name.value;
            bindings[binding_name] = {
                is_binding: true,
                name: binding_name,
            };
            state[binding_name] = value;                                // assign new value to binding

            return bindings[binding_name];                              // don't return the value itself, but the binding_object
        }                                                               // with which you can lookup the value

        case 'group':                                                   // expression within parentheses
            return evaluate(expr.expression);
        case 'unary': {
            let right_operand = evaluate(expr.right);
            if (expr.operator === token_types.MINUS) {
                return -right_operand; //TODO create negate function (because now it only works for numbers)
            } else if (expr.operator === token_types.NOT) {
                return !right_operand;
            } else {
                throw {message: 'illegal unary operator'};
            }
        }
        case 'binary': {
            switch (expr.operator) {
                case token_types.MINUS:
                    return subtraction(evaluate(expr.left), evaluate(expr.right));
                case token_types.PLUS:
                    return add(evaluate(expr.left), evaluate(expr.right));
                case token_types.STAR:
                    return multiply(evaluate(expr.left), evaluate(expr.right));
                case token_types.SLASH:
                    return division(evaluate(expr.left), evaluate(expr.right));
                case token_types.EQUALS_EQUALS:
                    return test_equal(evaluate(expr.left), evaluate(expr.right));
                case token_types.AND:
                    return logical_and(evaluate(expr.left), evaluate(expr.right));
                case token_types.OR:
                    return logical_or(evaluate(expr.left), evaluate(expr.right));
            }
            throw {message: 'illegal binary operator'};
        }
        case 'identifier': {
            if (expr.name in keywords) {
                return keywords[expr.name];
            } else {
                if (state[expr.name]) {
                    return state[expr.name];
                } else {
                    return undefined;
                }
            }
        }
        case 'literal': {
            return expr.value;
        }
    }
}

function parse() {
    token_index = 0;
    if (check(token_types.IDENTIFIER, token_index) && check(token_types.EQUALS, token_index + 1)) {
        let var_name = current_token();
        advance();
        advance();
        return {type: 'declaration', var_name: var_name, initializer: expression()};
    } else {
        return expression();
    }
}

function expression() {
    return equality();
}

function equality() {
    let expr = comparison()

    while (match([token_types.EQUALS_EQUALS, token_types.NOT_EQUALS])) {
        let operator = previous_token();
        let right = unary();
        expr = {type: 'binary', left: expr, operator: operator, right: right};
    }

    return expr;
}

function comparison() {
    let expr = add_sub();

    while (match([token_types.LESS, token_types.LESS_OR_EQUAL, token_types.GREATER, token_types.GREATER_OR_EQUAL])) {
        let operator = previous_token();
        let right = add_sub();
        expr = {type: 'binary', left: expr, operator: operator, right: right};
    }

    return expr;
}

function add_sub() {
    let expr = mult_div();

    while (match([token_types.OR, token_types.MINUS, token_types.PLUS])) {
        let operator = previous_token();
        let right = mult_div();
        expr = {type: 'binary', left: expr, operator: operator, right: right};
    }

    return expr;
}

function mult_div() {
    let expr = unary();

    while (match([token_types.AND, token_types.SLASH, token_types.STAR, token_types.DOT])) {
        let operator = previous_token();
        let right = unary();
        expr = {type: 'binary', left: expr, operator: operator, right: right};
    }

    return expr;
}

function unary() {
    if (match([token_types.NOT, token_types.MINUS])) {
        let operator = previous_token();
        let right = unary();
        return {type: 'unary', operator: operator, right: right};
    } else {
        return primary();
    }
}

function primary() {
    if (match([token_types.NUMERIC, token_types.STRING])) {
        return {type: 'literal', value: previous_token().value, value_type: previous_token().type};
    } else if (match([token_types.LEFT_PAREN])) {
        let expr = expression();
        if (expr && match([token_types.RIGHT_PAREN])) {
            return {
                type: 'group',
                expression: expr
            };
        } else {
            throw {message: 'expected expression or )'};
        }
    } else if (check(token_types.IDENTIFIER, token_index)) {
        let identifier = {
            type: 'identifier',
            name: current_token().value
        };
        advance();
        return identifier;
    } else if (match([token_types.LEFT_BRACKET])) {
        let array = [];
        if (!check(token_types.RIGHT_BRACKET, token_index)) {
            let result;
            do {
                result = expression();
                if (result) {
                    array.push(result);
                } else {
                    throw {message: "Expect ']' after array elements."};
                }
                match([token_types.COMMA]);
            } while (!match([token_types.RIGHT_BRACKET]));
        }

        return {type: 'array', elements: array};

    }
}

/**
 * matches token against array of tokens to check for equality (matching type)
 * @param tokens_to_match array of tokens
 * @returns {boolean}
 */
function match(tokens_to_match) {
    for (let i = 0; i < tokens_to_match.length; i++) {
        if (are_same(tokens_to_match[i], current_token())) {
            advance();
            return true;
        }
    }
    return false;
}

/**
 * Checks if token at position index matches the given
 * @param token_to_check expected token type
 * @param index of token to check
 * @returns {boolean}
 */
function check(token_to_check, index) {
    let token = tokens[index];
    if (!token) {
        return false;
    }
    return are_same(token_to_check, token);

}

/**
 * checks if 2 tokens have same type
 * @param token_1
 * @param token_2
 * @returns {boolean}
 */
function are_same(token_1, token_2) {
    if (is_at_end()) {
        return false;
    } else {
        return token_1.type === token_2.type;
    }

}

function is_at_end() {
    return token_index >= tokens.length;
}

function advance() {
    token_index += 1;
}

function previous_token() {
    return tokens[token_index - 1];
}

function current_token() {
    return tokens[token_index];
}


/**
 * Creates an array of tokens from a line of input.
 *
 * @returns {token_types[]}
 * @param command
 */
const scan = function (command) {
    tokens = [];
    let current_index = 0, // current index of char to look at in the command string
        word_start_index = 0; // marker for start of a literal or identifier

    while (!is_at_end()) {
        word_start_index = current_index;
        let token = scan_token();
        if (token) { // undefined mostly means whitespace
            tokens.push(token);
        }
    }

    function scan_token() {
        let next_char = advance();
        switch (next_char) {
            case '(':
                return token_types.LEFT_PAREN;
            case ')':
                return token_types.RIGHT_PAREN;
            case '[':
                return token_types.LEFT_BRACKET;
            case ']':
                return token_types.RIGHT_BRACKET;
            case ',':
                return token_types.COMMA;
            case '.':
                return token_types.DOT;
            case '-':
                return token_types.MINUS;
            case '+':
                return token_types.PLUS;
            case '*':
                return token_types.STAR;
            case '/':
                return token_types.SLASH;
            case '>':
                if (expect('=')) {
                    return token_types.GREATER_OR_EQUAL;
                } else {
                    return token_types.GREATER;
                }
            case '<':
                if (expect('=')) {
                    return token_types.LESS_OR_EQUAL;
                } else {
                    return token_types.LESS;
                }
            case '!':
                if (expect('=')) {
                    return token_types.NOT_EQUALS;
                } else {
                    return token_types.NOT;
                }
            case '=':
                if (expect('=')) {
                    return token_types.EQUALS_EQUALS;
                } else {
                    return token_types.EQUALS;
                }
            case '&':
                return token_types.AND;
            case '|':
                return token_types.OR;
            case '\'':
                return string();
            case ' ':
            case '\t':
            case '\r':
                advance();
                break;
        }
        if (is_digit(next_char)) {
            let token = Object.assign({}, token_types.NUMERIC);
            token.value = parse_number();
            return token;
        } else {
            if (is_alpha_or_underscore(next_char)) {
                let token = Object.assign({}, token_types.IDENTIFIER);
                token.value = parse_identifier();
                return token;
            }
        }
    }

    function expect(expected_char) {
        if (is_at_end()) {
            return false;
        }
        if (current_char() === expected_char) {
            advance();
            return true;
        } else {
            return false;
        }
    }

    function advance() {
        if (current_index < command.length) {
            current_index += 1;
        }
        return command[current_index - 1];
    }

    function is_at_end() {
        return current_index >= command.length;
    }

    function current_char() {
        return command[current_index];
    }

    function is_digit(char) {
        return char >= '0' && char <= '9';
    }

    function is_part_of_number(char) {
        return is_digit(char) || char === '.'; // no scientific notation for now
    }

    function parse_reference() {
        while (current_char() === '@' || is_digit(current_char())) {
            advance();
        }
        return command.substring(word_start_index, current_index);
    }

    function parse_number() {
        while (is_part_of_number(current_char())) {
            advance();
        }
        let number_string = command.substring(word_start_index, current_index);
        return Number.parseFloat(number_string);
    }

    function is_alpha_or_underscore(char) {
        return (char >= 'a' && char <= 'z') || (char >= 'A' && char <= 'Z') || char === '_';
    }

    function is_alphanumeric_or_underscore(char) {
        return (char >= 'a' && char <= 'z') || (char >= 'A' && char <= 'Z') || is_digit(char) || char === '_';
    }

    function parse_identifier() {
        while (is_alphanumeric_or_underscore(current_char())) {
            advance();
        }
        return command.substring(word_start_index, current_index);
    }

    function string() { // as of yet strings may not unclude escaped quotes that are also the start/end quote
        while (current_char() !== '\'' && !is_at_end()) {
            advance();
        }
        if (is_at_end() && current_char() !== '\'') {
            throw {message: 'unterminated string'}
        } else {
            let string_token = Object.assign({}, token_types.STRING);
            string_token.value = command.substring(word_start_index + 1, current_index);
            advance();
            return string_token;
        }
    }
};

const token_types = {
    LEFT_PAREN: {type: 'left_paren'},
    RIGHT_PAREN: {type: 'right_paren'},
    LEFT_BRACKET: {type: 'left_bracket'},
    RIGHT_BRACKET: {type: 'right_bracket'},
    COMMA: {type: 'comma'},
    DOT: {type: 'dot'},
    MINUS: {type: 'minus'},
    PLUS: {type: 'plus'},
    STAR: {type: 'star'},
    SLASH: {type: 'slash'},
    EQUALS: {type: 'equals'},
    EQUALS_EQUALS: {type: 'equals_equals'},
    NOT_EQUALS: {type: 'not_equals'},
    NOT: {type: 'not'},
    GREATER: {type: 'greater'},
    GREATER_OR_EQUAL: {type: 'greater_or_equal'},
    LESS: {type: 'less'},
    LESS_OR_EQUAL: {type: 'less_or_equal'},
    NUMERIC: {type: 'number', value: undefined},
    IDENTIFIER: {type: 'identifier', value: undefined},
    STRING: {type: 'string', value: undefined},
    AND: {type: 'logical_and'},
    OR: {type: 'logical_or'}
};

const multiply = function (left, right) {
    return left * right;
}

const division = function (left, right) {
    return left / right;
}

const add = function (left, right) {
    return left + right;
}

const subtraction = function (left, right) {
    return left - right;
}

const test_equal = function (left, right) {
    if (left.is_vector && right.is_vector) {
        return left.equals(right);
    } else {
        return left === right;
    }
}

const logical_and = function (left, right) {
    return left && right;
}

const logical_or = function (left, right) {
    return left || right;
}

const create_2d_id_matrix = function () {
    return {
        data: [[1, 0], [0, 1]],
        id: index_sequence++,
        is_visual: true,
        is_vector: false,         // for type comparison
        is_matrix: true,
        type: 'matrix',          // for showing type to user
        is_new: true,            // to determine view action
        visible: true,
        toString: function () {
            return `matrix@${this.id}`;
        },
        hide: function () {
            return hide(this);
        },
        label: function (text) {
            return label(this, text);
        },
        show: function () {
            return show(this);
        },
        equals: function (other) {
            return (this.id === other.id || (this.type === other.type && this.data === other.data)); // TODO
        },
        row: function (index) {
            return this.data[index];
        }
    }

}