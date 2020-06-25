use crate::usubtraction;
// This File handles Screen Buffer calles.

fn string_to_vec(w: usize, h: usize, string: &str) -> Vec<char> {
    let mut string: String = string.chars().map(|c| c).collect();
    let len = string.len();
    let space: String = (0..usubtraction(w, len)).map(|_| ' ').collect();
    string.push_str(&space);
    let mut s: String = string
        .lines()
        .flat_map(|row_str| {
            let len = row_str.len();
            let new_row = &row_str[..std::cmp::min(w as usize, len)];
            new_row.chars().chain((len..w as usize).map(|_| ' '))
        })
        .collect();
    let len = s.len();
    let space: String = (0..w * h - len).map(|_| ' ').collect();
    s.push_str(&space);
    s.chars().collect()
}

fn replace_from(idx: usize, w: usize, dst: &mut Vec<char>, src: &[char], queued: &mut Vec<usize>) {
    let dst_end = std::cmp::min(dst.len(), src.len() + idx);
    let src_end = std::cmp::min(dst_end - idx, src.len());
    dst[idx..dst_end].copy_from_slice(&src[..src_end]);
    if queued.is_empty() {
        queued.push(idx / w)
    }
    queued.extend((idx..dst_end).filter(|i| i % w == 0));
}

fn _insert_at(idx: usize, w: usize, line: &str, dst: &mut Vec<char>, queued: &mut Vec<usize>) {
    // Depracated.
    let start = (idx / w) * w;
    let max_width = std::cmp::min(w - 1, line.len());
    let mut src: Vec<char> = line[..max_width].chars().collect();
    let spaces: Vec<char> = (0..usubtraction(w, src.len())).map(|_| ' ').collect();
    src.extend(spaces);
    replace_from(start, w, dst, &src, queued);
}

pub fn screen_update(w: usize, h: usize, text: &str, dst: &mut Vec<char>, queued: &mut Vec<usize>) {
    let src = string_to_vec(w, h, text);
    replace_from(0, w, dst, &src[..], queued);
}

pub fn screen_update_line(
    line_num: usize,         // Screen y value
    w: usize,                // Width of Screen
    text: &str,              // Line form Text file
    dst: &mut Vec<char>,     // Screen Array
    queued: &mut Vec<usize>, // Index line numbers queued for updating on Screen
) {
    // Takes data to update screen.
    let mut src: Vec<char> = text.chars().collect();
    src.extend((0..usubtraction(w, src.len())).map(|_| ' '));
    dst[line_num * w..line_num * w + w].copy_from_slice(&src[..w]);
    queued.push(line_num * w);
}

pub fn _screen_update_lines(
    w: usize,                  // Width of Screen
    lines: Vec<(usize, &str)>, // Lines contain line loc and text form file
    dst: &mut Vec<char>,       // Screen Array
    queued: &mut Vec<usize>,   // Index line numbers queued for updating on Screen
) {
    for (line_num, line) in &lines {
        screen_update_line(*line_num, w, &line, dst, queued);
    }
}

pub fn screen_update_line_down(
    line_num: usize,         // Cursor y value
    w: usize,                // Width of Screen
    text: &str,              // Lines form Text file
    dst: &mut Vec<char>,     // Screen Array
    queued: &mut Vec<usize>, // Index line numbers queued for updating on Screen
) {
    for (idx, line) in text.lines().enumerate() {
        screen_update_line(idx + line_num, w, &line, dst, queued);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_vec_over_filled() {
        let width = 5;
        let height = 5;
        let string = string_to_vec(width, height, "123\n123\n1\n");
        let true_output = vec![
            '1', '2', '3', ' ', ' ', '1', '2', '3', ' ', ' ', '1', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ];
        assert_eq!(string, true_output);
    }

    #[test]
    fn test_string_to_vec_empty() {
        let width = 5;
        let height = 5;
        let string = string_to_vec(width, height, "");
        let true_output = vec![
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ];
        assert_eq!(string, true_output);
    }

    #[test]
    fn test_string_to_vec_line_one() {
        let width = 5;
        let height = 5;
        let string = string_to_vec(width, height, "123");
        let true_output = vec![
            '1', '2', '3', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ];
        assert_eq!(string, true_output);
    }
}
