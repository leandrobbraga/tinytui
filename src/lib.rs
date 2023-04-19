pub trait Widget {
    fn render(&self, terminal: &mut Terminal);
}

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
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.buffer[y * self.width + x] as char);
            }
            println!();
        }
    }

    pub fn area(&self) -> Rectangle {
        Rectangle::new(0, 0, self.width as u32, self.height as u32)
    }
}

#[derive(Debug)]
pub struct Rectangle {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(x: u32, y: u32, width: u32, height: u32) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }

    pub fn split_horizontally(self) -> (Rectangle, Rectangle) {
        let width = self.width / 2;
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
            width: width - gap,
            height: self.height,
        };
        let right = Rectangle {
            x: self.x + width + gap,
            y: self.y,
            width: (self.width - width - gap),
            height: self.height,
        };

        (left, right)
    }

    pub fn split_vertically(self) -> (Rectangle, Rectangle) {
        let height = self.height / 2;

        let top = Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height,
        };
        let bottom = Rectangle {
            x: self.x,
            y: self.y + height,
            width: self.width,
            height: (self.height - height),
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
        for y in self.y..self.y + self.height {
            for x in self.x..self.x + self.width {
                if y == self.y || y == self.y + self.height - 1 {
                    if x == self.x || x == self.x + self.width - 1 {
                        terminal.buffer[(y * terminal.width as u32 + x) as usize] = b'+';
                    } else {
                        terminal.buffer[(y * terminal.width as u32 + x) as usize] = b'-';
                    }
                } else if x == self.x || x == self.x + self.width - 1 {
                    terminal.buffer[(y * terminal.width as u32 + x) as usize] = b'|';
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
        assert!((text.len() as u32) < area.width - 2); // -2 for the border
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

        let x;
        let y;

        match self.vertical_alignment {
            VerticalAlignment::Top => y = self.area.y + 1, // +1 for the border
            VerticalAlignment::Bottom => y = self.area.y + self.area.height - 1 - 1, // -1 for the border
            VerticalAlignment::Center => y = self.area.y + self.area.height / 2,
        }

        match self.horizontal_alignment {
            HorizontalAlignment::Left => x = self.area.x + 1, // +1 for the border
            HorizontalAlignment::Right => {
                x = self.area.x + self.area.width - self.text.len() as u32 - 1 // -1 for the border
            }
            HorizontalAlignment::Center => {
                x = self.area.x + self.area.width / 2 - self.text.len() as u32 / 2
            }
        }

        for (i, c) in self.text.chars().enumerate() {
            terminal.buffer[(y * terminal.width as u32 + x + i as u32) as usize] = c as u8;
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
        assert!((items.len() as u32) < area.height - 2); // -2 for the border
        assert!(items.iter().map(|item| item.len()).max() < Some(area.width as usize - 2)); // -2 for the border

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

        let x;
        let y;

        match self.vertical_alignment {
            VerticalAlignment::Top => y = self.area.y + 1, // +1 for the border
            VerticalAlignment::Bottom => {
                // -1 for the border
                y = self.area.y + self.area.height - self.items.len() as u32 - 1
            }
            VerticalAlignment::Center => {
                y = self.area.y + self.area.height / 2 - self.items.len() as u32 / 2
            }
        }

        match self.horizontal_alignment {
            HorizontalAlignment::Left => x = self.area.x + 1, // +1 for the border
            HorizontalAlignment::Right => {
                x = self.area.x + self.area.width
                    - self.items.iter().map(|item| item.len()).max().unwrap() as u32
                    - 1 // -1 for the border
            }
            HorizontalAlignment::Center => {
                x = self.area.x + self.area.width / 2
                    - self.items.iter().map(|item| item.len()).max().unwrap() as u32 / 2
            }
        }

        for (i, item) in self.items.iter().enumerate() {
            for (j, c) in item.chars().enumerate() {
                terminal.buffer[((y + i as u32) * terminal.width as u32 + x + j as u32) as usize] =
                    c as u8;
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

        assert!((items.len() as u32) < area.height - 2); // -2 for the border
        assert!((max_item_size * max_row_size) < area.width as usize - 2); // -2 for the border

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

        let x;
        let y;

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

        match self.vertical_alignment {
            VerticalAlignment::Top => y = self.area.y + 1, // +1 for the border
            VerticalAlignment::Bottom => {
                // -1 for the border
                y = self.area.y + self.area.height - self.items.len() as u32 - 1
            }
            VerticalAlignment::Center => {
                y = self.area.y + self.area.height / 2 - self.items.len() as u32 / 2
            }
        }

        match self.horizontal_alignment {
            HorizontalAlignment::Left => x = self.area.x + 1, // +1 for the border
            HorizontalAlignment::Right => {
                x = self.area.x + self.area.width - (max_item_size * max_row_size) as u32 - 1
                // -1 for the border
            }
            HorizontalAlignment::Center => {
                x = self.area.x + self.area.width / 2 - (max_item_size * max_row_size) as u32 / 2
            }
        }


        for (row_index, row) in self.items.iter().enumerate() {
            for (column_index, item) in row.iter().enumerate() {
                for (k, c) in item.chars().enumerate() {
                    // Go to the correct line in the buffer
                    terminal.buffer[((y + row_index as u32) * terminal.width as u32 
                        // Go to the start of the table
                        + x 
                        // Go to the start of the table column 
                        + (column_lengths.iter().take(column_index).map(|length| *length as u32).sum::<u32>())
                        // Add spacing between table columns
                        + column_index as u32 
                        // Go to the character position                         
                        + k as u32) as usize] = c as u8; 
                }
            }
        }
    }
}
