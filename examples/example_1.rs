use tinytui::{Color, HorizontalAlignment, Terminal, VerticalAlignment, Widget};

fn main() {
    let mut terminal = Terminal::try_new().unwrap();

    let screen = terminal.area();
    let (left, right) = screen.split_horizontally();
    let (left_top, left_bottom) = left.split_vertically_at(0.7);
    let (top, bottom) = right.split_vertically();
    let (bottom_left, bottom_right) = bottom.split_horizontally();
    let (bottom_right_top, bottom_right_bottom) = bottom_right.split_vertically();

    let mut text_1 = top.text(
        "Center".to_string(),
        VerticalAlignment::Center,
        HorizontalAlignment::Center,
    );
    text_1.set_title(Some("[ Center Alignment ]".into()));

    let mut text_2 = bottom_left.text(
        "Top Left".to_string(),
        VerticalAlignment::Top,
        HorizontalAlignment::Left,
    );
    text_2.set_title(Some("[ Top Left Alignment ]".into()));

    let mut text_3 = bottom_right_top.text(
        "Bottom Right".to_string(),
        VerticalAlignment::Bottom,
        HorizontalAlignment::Right,
    );
    text_3.set_title(Some("[ Bottom Right Alignment ]".into()));

    let mut item_list = bottom_right_bottom.item_list(
        vec![
            "This item is not selected".to_string(),
            "This item is selected".to_string(),
            "This item is also not selected".to_string(),
        ],
        VerticalAlignment::Center,
        HorizontalAlignment::Center,
    );
    item_list.set_title(Some("[ Item List ]".into()));
    item_list.set_selected(Some(1));
    item_list.set_border_color(Color::Green);

    let mut table = left_bottom.table(
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
    table.set_title(Some("[ Table ]".into()));

    left_top.render(&mut terminal);
    text_1.render(&mut terminal);
    text_2.render(&mut terminal);
    item_list.render(&mut terminal);
    text_3.render(&mut terminal);
    table.render(&mut terminal);

    terminal.draw();

    std::thread::sleep(std::time::Duration::from_secs(1))
}
