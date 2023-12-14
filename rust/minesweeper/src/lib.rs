const MINE: char = '*';
const MINE_B: u8 = b'*';

#[derive(Copy, Clone)]
enum Cell {
    Mine,
    Safe(u8),
}

impl Cell {
    fn increment(&mut self) {
        *self = match self {
            Cell::Mine => Cell::Mine,
            Cell::Safe(val) => Cell::Safe(*val + 1),
        };
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    };

    let rows_number = minefield.len();
    let cols_number = minefield[0].len();

    let mut field = vec![vec![Cell::Safe(0u8); cols_number]; rows_number];

    for row_index in 0..rows_number {
        let current_row = minefield[row_index].as_bytes();
        for col_index in 0..cols_number {
            if current_row[col_index] == MINE_B {
                field[row_index][col_index] = Cell::Mine;

                let mut inc = |row_shift: isize, col_shift: isize| {
                    if let Some(row_index) = row_index.checked_add_signed(row_shift) {
                        if row_index < rows_number {
                            if let Some(col_index) = col_index.checked_add_signed(col_shift) {
                                if col_index < cols_number {
                                    field[row_index][col_index].increment();
                                }
                            }
                        }
                    }
                };

                inc(-1, -1);
                inc(-1, 0);
                inc(-1, 1);
                inc(0, 1);
                inc(0, -1);
                inc(1, -1);
                inc(1, 0);
                inc(1, 1);
            }
        }
    }

    map_result(field)
}

fn map_result(field: Vec<Vec<Cell>>) -> Vec<String> {
    let result: Vec<String> = field
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| match cell {
                    Cell::Mine => MINE,
                    Cell::Safe(val) => match val {
                        0 => ' ',
                        _ => std::char::from_digit(*val as u32, 10).unwrap(),
                    },
                })
                .collect()
        })
        .collect();
    result
}
