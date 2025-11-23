use tmux_ship::render::render_from_ansi;

#[test]
fn renders_styles_and_plain_text() {
    let ansi = "\u{1b}[34;1muser\u{1b}[0m@\u{1b}[48;2;34;34;34;3mhost\u{1b}[0m";
    let rendered = render_from_ansi(ansi);
    assert_eq!(
        rendered,
        "#[fg=blue,bold]user#[default]@#[bg=#222222,italics]host#[default]"
    );
}

#[test]
fn resets_on_clear() {
    let ansi = "\u{1b}[31mred\u{1b}[0mplain";
    let rendered = render_from_ansi(ansi);
    assert_eq!(rendered, "#[fg=red]red#[default]plain");
}
