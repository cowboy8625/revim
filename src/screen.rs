pub fn string_to_vec(w: u16, string: &str) -> Vec<char> {
    string
        .lines()
        .flat_map(|row_str| {
            let len = row_str.len();
            row_str
                .chars()
                .chain((len..w as usize).into_iter().map(|_| ' '))
        })
        .collect()
}

pub fn replace_from(idx: usize, w: usize, dst: &mut Vec<char>, src: &Vec<char>, queued: &mut Vec<usize>) {
    let dst_end = std::cmp::min(dst.len(), src.len() + idx);
    let src_end = std::cmp::min(dst_end - idx, src.len());
    dst[idx..dst_end].copy_from_slice(&src[..src_end]);
    if queued.is_empty() {
        queued.push(idx / w)
    }
    queued.extend((idx..dst_end).filter(|i| i % w == 0));
}

pub fn insert_at(idx: usize, w: usize, dst: &mut Vec<char>, queued: &mut Vec<usize>, chr: char) {
    let start = (idx / w) * w;
    let end   = idx + w;
    let i     = idx - start;
    let mut src: Vec<char> = dst[start..end].iter().map(|c| *c).collect();
    src.insert(i, chr);
    src.pop();
    replace_from(start, w, dst, &src, queued);
}


