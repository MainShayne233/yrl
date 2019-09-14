pub fn strip_extra(src: &str) -> String {
    enum Mode {
        Normal,
        CharList,
        MaybeBinary,
        Binary,
        MaybeNotBinary,
        Comment,
    };

    let mut buffer = String::with_capacity(src.len());
    let mut mode = Mode::Normal;
    let mut chars = src.chars();
    while let Some(c) = chars.next() {
        match mode {
            Mode::Normal => match c {
                '\'' => {
                    mode = Mode::CharList;
                    buffer.push(c);
                }
                '<' => {
                    mode = Mode::MaybeBinary;
                    buffer.push(c);
                }
                '%' => {
                    mode = Mode::Comment;
                    buffer.push(' ');
                    buffer.push(' ');
                }
                _ => buffer.push(c),
            },

            Mode::MaybeBinary => match c {
                '<' => {
                    mode = Mode::Binary;
                    buffer.push(c);
                }
                _ => {
                    mode = Mode::Normal;
                    buffer.push(c);
                }
            },

            Mode::Binary => match c {
                '>' => {
                    mode = Mode::MaybeNotBinary;
                    buffer.push(c);
                }
                _ => {
                    buffer.push(c);
                }
            },

            Mode::MaybeNotBinary => match c {
                '>' => {
                    mode = Mode::Normal;
                    buffer.push(c);
                }
                _ => {
                    mode = Mode::Binary;
                    buffer.push(c);
                }
            },

            Mode::CharList => match c {
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
