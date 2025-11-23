use tmux_ship::render::render_from_json;

#[test]
fn renders_styles_and_plain_text() {
    let json = r#"[
        {"content":"user","style":"fg:blue bold"},
        {"content":"@","style":null},
        {"text":"host","style":"bg:#222222 italic"}
    ]"#;

    let rendered = render_from_json(json).unwrap();
    assert_eq!(
        rendered,
        "#[fg=blue,bold]user#[default]@#[bg=#222222,italics]host#[default]"
    );
}

#[test]
fn ignores_unknown_tokens() {
    let json = r#"[{"content":"x","style":"weird fg:unknown"}]"#;
    let rendered = render_from_json(json).unwrap();
    assert_eq!(rendered, "x");
}
