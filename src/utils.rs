pub fn get_arg(content: &mut String) -> String {
    let mut arg = String::new();
    let mut quoted = false;
    loop {
        if content.is_empty() {
            break;
        }
        let ch = content.remove(0);
        if ch == ' ' {
            if quoted {
                arg.push(ch);
            } else {
                break;
            }
        } else if ch == '"' && quoted {
            if !content.is_empty() && content.starts_with(' ') {
                content.remove(0);
            }
            break;
        } else if ch == '"' {
            quoted = true;
        } else {
            arg.push(ch)
        }
    }
    arg
}
