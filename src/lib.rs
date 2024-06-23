pub trait Widget {
    fn render(&self, terminal: &mut Terminal);
}

// TODO: Get the actual terminal width and height
pub struct Terminal {
    buffer: Vec<u8>,
    width: usize,
    height: usize,
}

impl Terminal {
    pub fn new(width: usize, height: usize) -> Terminal {
        Terminal {
            buffer: vec![b' '; width * height],
            width,
            height,
        }
    }

    pub fn render(&self) {
        for i in (0..self.buffer.len()).step_by(self.width) {
            let line = std::str::from_utf8(&self.buffer[i..i + self.width]).unwrap();
            println!("{line}");
        }
    }

    pub fn area(&self) -> Rectangle {
        Rectangle::new(0, 0, self.width, self.height)
    }
}

#[derive(Debug)]
pub struct Rectangle {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Rectangle {
    fn new(x: usize, y: usize, width: usize, height: usize) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }

    pub fn split_horizontally(self) -> (Rectangle, Rectangle) {
        self.split_horizontally_at(0.5)
    }

    pub fn split_horizontally_at(self, percentage: f32) -> (Rectangle, Rectangle) {
        assert!(percentage > 0.0 && percentage < 1.0);

        let left_width = (self.width as f32 * percentage) as usize;
        let right_width = self.width - left_width;
        // Horizontal split without gaps        Vertical split without gaps
        //        +-----++-----+                      +------------+
        //        |     ||     |                      |            |
        //        |     ||     |                      +------------+
        //        |     ||     |                      +------------+
        //        |     ||     |                      |            |
        //        +-----++-----+                      +------------+
        //
        // Since the columns are thinner than the rows, we compensate for that in the
        // horizontal splitting by adding a gap between the two rectangles.
        //        +-----+ +----+
        //        |     | |    |
        //        |     | |    |
        //        |     | |    |
        //        |     | |    |
        //        +-----+ +----+
        let gap = 1;

        let left = Rectangle {
            x: self.x,
            y: self.y,
            width: left_width - gap,
            height: self.height,
        };
        let right = Rectangle {
            x: self.x + left_width + gap,
            y: self.y,
            width: (right_width - gap),
            height: self.height,
        };

        (left, right)
    }
    pub fn split_vertically(self) -> (Rectangle, Rectangle) {
        self.split_vertically_at(0.5)
    }
    pub fn split_vertically_at(self, percentage: f32) -> (Rectangle, Rectangle) {
        assert!(percentage > 0.0 && percentage < 1.0);

        let top_height = (self.height as f32 * percentage) as usize;
        let bottom_height = self.height - top_height;

        let top = Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: top_height,
        };
        let bottom = Rectangle {
            x: self.x,
            y: self.y + top_height,
            width: self.width,
            height: bottom_height,
        };

        (top, bottom)
    }

    pub fn text(
        self,
        text: String,
        vertical_alignment: VerticalAlignment,
        horizontal_alignment: HorizontalAlignment,
    ) -> Text {
        Text::new(text, vertical_alignment, horizontal_alignment, self)
    }

    pub fn item_list(
        self,
        items: Vec<String>,
        vertical_alignment: VerticalAlignment,
        horizontal_alignment: HorizontalAlignment,
    ) -> ItemList {
        ItemList::new(items, vertical_alignment, horizontal_alignment, self)
    }

    pub fn table(
        self,
        items: Vec<Vec<String>>,
        vertical_alignment: VerticalAlignment,
        horizontal_alignment: HorizontalAlignment,
    ) -> Table {
        Table::new(items, vertical_alignment, horizontal_alignment, self)
    }
}

impl Widget for Rectangle {
    fn render(&self, terminal: &mut Terminal) {
        // We iterate in this order to help with cache locality
        for y in self.y..self.y + self.height {
            for x in self.x..self.x + self.width {
                if y == self.y || y == self.y + self.height - 1 {
                    if x == self.x || x == self.x + self.width - 1 {
                        terminal.buffer[y * terminal.width + x] = b'+';
                    } else {
                        terminal.buffer[y * terminal.width + x] = b'-';
                    }
                } else if x == self.x || x == self.x + self.width - 1 {
                    terminal.buffer[y * terminal.width + x] = b'|';
                } else {
                    continue;
                }
            }
        }
    }
}

pub struct Text {
    text: String,
    vertical_alignment: VerticalAlignment,
    horizontal_alignment: HorizontalAlignment,
    area: Rectangle,
}

pub enum HorizontalAlignment {
    Left,
    Right,
    Center,
}

pub enum VerticalAlignment {
    Top,
    Bottom,
    Center,
}

impl Text {
    fn new(
        text: String,
        vertical_alignment: VerticalAlignment,
        horizontal_alignment: HorizontalAlignment,
        area: Rectangle,
    ) -> Text {
        assert!(text.len() < area.width - 2); // -2 for the border
        Text {
            text,
            vertical_alignment,
            horizontal_alignment,
            area,
        }
    }
}
impl Widget for Text {
    fn render(&self, terminal: &mut Terminal) {
        self.area.render(terminal);

        let y = match self.vertical_alignment {
            VerticalAlignment::Top => self.area.y + 1, // +1 for the border
            VerticalAlignment::Bottom => self.area.y + self.area.height - 1 - 1, // -1 for the border
            VerticalAlignment::Center => self.area.y + self.area.height / 2,
        };

        let x = match self.horizontal_alignment {
            HorizontalAlignment::Left => self.area.x + 1, // +1 for the border
            HorizontalAlignment::Right => {
                self.area.x + self.area.width - self.text.len() - 1 // -1 for the border
            }
            HorizontalAlignment::Center => self.area.x + self.area.width / 2 - self.text.len() / 2,
        };

        for (i, c) in self.text.chars().enumerate() {
            terminal.buffer[y * terminal.width + x + i] = c as u8;
        }
    }
}

pub struct ItemList {
    items: Vec<String>,
    vertical_alignment: VerticalAlignment,
    horizontal_alignment: HorizontalAlignment,
    area: Rectangle,
}

impl ItemList {
    fn new(
        items: Vec<String>,
        vertical_alignment: VerticalAlignment,
        horizontal_alignment: HorizontalAlignment,
        area: Rectangle,
    ) -> ItemList {
        assert!(items.len() < area.height - 2); // -2 for the border
        assert!(items.iter().map(|item| item.len()).max() < Some(area.width - 2)); // -2 for the border

        ItemList {
            items,
            vertical_alignment,
            horizontal_alignment,
            area,
        }
    }
}

impl Widget for ItemList {
    fn render(&self, terminal: &mut Terminal) {
        self.area.render(terminal);

        let y = match self.vertical_alignment {
            VerticalAlignment::Top => self.area.y + 1, // +1 for the border
            VerticalAlignment::Bottom => {
                // -1 for the border
                self.area.y + self.area.height - self.items.len() - 1
            }
            VerticalAlignment::Center => self.area.y + self.area.height / 2 - self.items.len() / 2,
        };

        let x = match self.horizontal_alignment {
            HorizontalAlignment::Left => self.area.x + 1, // +1 for the border
            HorizontalAlignment::Right => {
                self.area.x + self.area.width
                    - self.items.iter().map(|item| item.len()).max().unwrap()
                    - 1 // -1 for the border
            }
            HorizontalAlignment::Center => {
                self.area.x + self.area.width / 2
                    - self.items.iter().map(|item| item.len()).max().unwrap() / 2
            }
        };

        for (i, item) in self.items.iter().enumerate() {
            for (j, c) in item.chars().enumerate() {
                terminal.buffer[(y + i) * terminal.width + x + j] = c as u8;
            }
        }
    }
}

pub struct Table {
    items: Vec<Vec<String>>,
    vertical_alignment: VerticalAlignment,
    horizontal_alignment: HorizontalAlignment,
    area: Rectangle,
}

impl Table {
    fn new(
        items: Vec<Vec<String>>,
        vertical_alignment: VerticalAlignment,
        horizontal_alignment: HorizontalAlignment,
        area: Rectangle,
    ) -> Table {
        let max_item_size = items
            .iter()
            .map(|row| row.iter().map(|item| item.len()).max().unwrap())
            .max()
            .unwrap();
        let max_row_size = items.iter().map(|row| row.len()).max().unwrap();

        assert!((items.len()) < area.height - 2); // -2 for the border
        assert!((max_item_size * max_row_size) < area.width - 2); // -2 for the border

        Table {
            items,
            vertical_alignment,
            horizontal_alignment,
            area,
        }
    }
}

impl Widget for Table {
    fn render(&self, terminal: &mut Terminal) {
        self.area.render(terminal);

        let max_item_size = self
            .items
            .iter()
            .map(|row| row.iter().map(|item| item.len()).max().unwrap())
            .max()
            .unwrap();
        let max_row_size = self.items.iter().map(|row| row.len()).max().unwrap();

        let mut column_lengths = vec![0; max_row_size];
        for row in self.items.iter() {
            for (i, item) in row.iter().enumerate() {
                if item.len() > column_lengths[i] {
                    column_lengths[i] = item.len();
                }
            }
        }

        let y = match self.vertical_alignment {
            VerticalAlignment::Top => self.area.y + 1, // +1 for the border
            VerticalAlignment::Bottom => {
                // -1 for the border
                self.area.y + self.area.height - self.items.len() - 1
            }
            VerticalAlignment::Center => self.area.y + self.area.height / 2 - self.items.len() / 2,
        };

        let x = match self.horizontal_alignment {
            HorizontalAlignment::Left => self.area.x + 1, // +1 for the border
            HorizontalAlignment::Right => {
                // -1 for the border
                self.area.x + self.area.width - max_item_size * max_row_size - 1
            }
            HorizontalAlignment::Center => {
                self.area.x + self.area.width / 2 - max_item_size * max_row_size / 2
            }
        };

        for (row_index, row) in self.items.iter().enumerate() {
            for (column_index, item) in row.iter().enumerate() {
                for (k, c) in item.chars().enumerate() {
                    // Go to the correct line in the buffer
                    terminal.buffer[(y + row_index) * terminal.width
                        // Go to the start of the table
                        + x
                        // Go to the start of the table column 
                        + (column_lengths.iter().take(column_index).sum::<usize>())
                        // Add spacing between table columns
                        + column_index
                        // Go to the character position                         
                        + k] = c as u8;
                }
            }
        }
    }
}
