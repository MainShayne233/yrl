pub fn strip_extra(src: &str) -> String {
    enum Mode {
        Normal,
        String,
        Comment,
    };

    let mut buffer = String::with_capacity(src.len());
    let mut mode = Mode::Normal;
    let mut chars = src.chars();
    while let Some(c) = chars.next() {
        match mode {
            Mode::Normal => match c {
                '\'' => {
                    mode = Mode::String;
                    buffer.push(c);
                }
                '%' => {
                    mode = Mode::Comment;
                    buffer.push(' ');
                    buffer.push(' ');
                }
                _ => buffer.push(c),
            },

            Mode::String => match c {
                '\'' => {
                    mode = Mode::Normal;
                    buffer.push(c);
                }
                _ => buffer.push(c),
            },

            Mode::Comment => match c {
                '\n' => {
                    mode = Mode::Normal;
                    buffer.push('\n');
                }
                _ => buffer.push(' '),
            },
        }
    }
    buffer
}
