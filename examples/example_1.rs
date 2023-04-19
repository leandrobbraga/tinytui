use tinytui::{HorizontalAlignment, Terminal, VerticalAlignment, Widget};

fn main() {
    let mut terminal = Terminal::new(100, 25);

    let screen = terminal.area();
    let (left, right) = screen.split_horizontally();
    let (top, bottom) = right.split_vertically();
    let (bottom_left, bottom_right) = bottom.split_horizontally();
    let (bottom_right_top, bottom_right_bottom) = bottom_right.split_vertically();

    let text_1 = top.text(
        "Center".to_string(),
        VerticalAlignment::Center,
        HorizontalAlignment::Center,
    );

    let text_2 = bottom_left.text(
        "Top Left".to_string(),
        VerticalAlignment::Top,
        HorizontalAlignment::Left,
    );

    let text_3 = bottom_right_top.text(
        "Bottom Right".to_string(),
        VerticalAlignment::Bottom,
        HorizontalAlignment::Right,
    );

    let item_list = bottom_right_bottom.item_list(
        vec![
            "Hello, World!".to_string(),
            "Hello, Sailor!".to_string(),
            "Hello, Seaman!".to_string(),
        ],
        VerticalAlignment::Center,
        HorizontalAlignment::Center,
    );

    let table = left.table(
        vec![
            vec![
                "Hello World".to_string(),
                "Hello".to_string(),
                "Hello".to_string(),
            ],
            vec![
                "World!".to_string(),
                "Sailor!".to_string(),
                "Seaman!".to_string(),
            ],
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
        ],
        VerticalAlignment::Center,
        HorizontalAlignment::Center,
    );

    text_1.render(&mut terminal);
    text_2.render(&mut terminal);
    item_list.render(&mut terminal);
    text_3.render(&mut terminal);
    table.render(&mut terminal);

    terminal.render();
}
