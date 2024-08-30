/// read the next word after the given `index`
/// returns the next word and the index at which it ends in `input`
/// (so that it is easier to get more words after it also)
pub(crate) fn read_next_word(input: &String, mut index: usize) -> (String, usize) {
    let mut next_word = String::new();
    for (i, c) in input.chars().enumerate().skip(index) {
        index = i;
        if c == ';' {
            break;
        }
        if c == ' ' || c == '\t' || c == '\n' || c == ',' {
            if next_word.is_empty() {
                continue;
            }
            break;
        }
        next_word.push(c);
    }
    println!("{index}");
    (next_word, index)
}

/// read the next list after the given `index`
/// returns success: bool (false denotes parse error),
/// the next list and the index at which it ends in `input`
/// (so that it is easier to get more words after it also)
pub(crate) fn read_next_list(input: &String, mut index: usize) -> (bool, Vec<String>, usize) {
    let mut list_started = false;
    let mut list_ended = false;
    let mut success: bool = false;
    let mut next_list: Vec<String> = Vec::new();
    let mut cur: String;
    while let Some(c) = input.chars().nth(index) {
        if c == ';' {
            break;
        }
        if (!list_started || list_ended) && (c == ' ' || c == '\t' || c == '\n') {
            index += 1;
            continue;
        }
        if c == '(' {
            if !list_started {
                list_started = true;
                index += 1;
                continue;
            }
            break;
        }
        if c == ')' {
            success = !next_list.is_empty();
            break;
        }
        if list_ended {
            // edge case when there is no `,` between two elements of the list
            // eg: create table_name (column1 column2) -> ParseError
            break;
        }
        (cur, index) = read_next_word(input, index);
        if let Some(last_char) = cur.chars().last() {
            if last_char == ')' {
                cur.pop();
                list_ended = true;
            }
            next_list.push(cur);
            if last_char == ')' {
                // in next iteration this loop will break with success == true
                index -= 1;
            }
        } else {
            break;
        }
    }

    (success, next_list, index)
}
