use crate::hex_grid::PosOddQHex;
use std::ops::Range;

pub trait HexContent {
    fn hex_content(&self, pos: &PosOddQHex, content_row: isize) -> String;
}

fn push_char(output: &mut String, c: char, count: usize) {
    for _ in 0..count {
        output.push(c);
    }
}

fn push_content_row<'a, C>(
    output: &mut String,
    length: usize,
    hex_content: &'a C,
    q: isize,
    r: isize,
    content_row: isize,
) where
    &'a C: HexContent,
{
    let s = hex_content.hex_content(&PosOddQHex { q, r }, content_row);

    let char_count = s.chars().count();
    let pad = usize::saturating_sub(length, char_count);
    let left_pad = pad / 2;
    let right_pad = pad - left_pad;
    push_char(output, ' ', left_pad);
    for c in s.chars().take(length) {
        output.push(c);
    }
    push_char(output, ' ', right_pad);
}

pub fn render_hex_grid<'a, C>(
    hex_content: &'a C,
    rows: Range<isize>,
    cols: Range<isize>,
    flat_w: usize,
    half_h: usize,
) -> String
where
    &'a C: HexContent,
{
    let mut rendered = String::new();

    for _ in cols.clone().step_by(2) {
        push_char(&mut rendered, ' ', half_h);
        push_char(&mut rendered, '_', flat_w);
        push_char(&mut rendered, ' ', half_h + flat_w);
    }
    rendered.push('\n');

    for row in rows.clone() {
        let sub_rows = 2 * half_h;
        for sub_row in 0..sub_rows {
            let is_upper_half = sub_row < half_h;
            let (l_slash, r_slash) = if is_upper_half {
                ('╱', '╲')
            } else {
                ('╲', '╱')
            };
            let left_pad = (usize::abs_diff(2 * half_h - 1, 2 * sub_row) - 1) / 2;
            push_char(&mut rendered, ' ', left_pad);
            rendered.push(l_slash);
            for col in cols.clone().step_by(2) {
                let widest_content_w = flat_w + 2 * (half_h - 1);
                let this_content_w = widest_content_w - 2 * left_pad;
                let other_content_w = flat_w + 2 * left_pad;
                let other_row = row - (is_upper_half as isize);
                let other_sub_row = (sub_row + half_h) % (2 * half_h);
                let content_index = (half_h as isize) - (sub_row as isize) - 1;
                let other_content_index = (half_h as isize) - (other_sub_row as isize) - 1;

                if sub_row == sub_rows - 1 {
                    push_char(&mut rendered, '_', flat_w);
                } else {
                    push_content_row(
                        &mut rendered,
                        this_content_w,
                        hex_content,
                        col,
                        row,
                        content_index,
                    );
                }
                rendered.push(r_slash);
                if other_sub_row == sub_rows - 1 {
                    push_char(&mut rendered, '_', flat_w);
                } else {
                    push_content_row(
                        &mut rendered,
                        other_content_w,
                        hex_content,
                        col + 1,
                        other_row,
                        other_content_index,
                    );
                }
                rendered.push(l_slash);
            }
            rendered.push('\n');
        }
    }
    rendered
}
