pub fn filter_line(filter: u8, bpp: usize, data: &[u8], last_line: &[u8]) -> Vec<u8> {
    let mut filtered = Vec::with_capacity(data.len());
    match filter {
        0 => {
            filtered.extend_from_slice(data);
        }
        1 => {
            filtered.extend_from_slice(&data[0..bpp]);
            filtered.extend(
                data.iter()
                    .skip(bpp)
                    .zip(data.iter())
                    .map(|(cur, last)| cur.wrapping_sub(*last)),
            );
        }
        2 => {
            if last_line.is_empty() {
                filtered.extend_from_slice(data);
            } else {
                filtered.extend(
                    data.iter()
                        .zip(last_line.iter())
                        .map(|(cur, last)| cur.wrapping_sub(*last)),
                );
            };
        }
        3 => for (i, byte) in data.iter().enumerate() {
            if last_line.is_empty() {
                filtered.push(match i.checked_sub(bpp) {
                    Some(x) => byte.wrapping_sub(data[x] >> 1),
                    None => *byte,
                });
            } else {
                filtered.push(match i.checked_sub(bpp) {
                    Some(x) => byte
                        .wrapping_sub(((u16::from(data[x]) + u16::from(last_line[i])) >> 1) as u8),
                    None => byte.wrapping_sub(last_line[i] >> 1),
                });
            };
        },
        4 => for (i, byte) in data.iter().enumerate() {
            if last_line.is_empty() {
                filtered.push(match i.checked_sub(bpp) {
                    Some(x) => byte.wrapping_sub(data[x]),
                    None => *byte,
                });
            } else {
                filtered.push(match i.checked_sub(bpp) {
                    Some(x) => {
                        byte.wrapping_sub(paeth_predictor(data[x], last_line[i], last_line[x]))
                    }
                    None => byte.wrapping_sub(last_line[i]),
                });
            };
        },
        _ => unreachable!(),
    }
    filtered
}

pub fn unfilter_line(filter: u8, bpp: usize, data: &[u8], last_line: &[u8]) -> Vec<u8> {
    let mut unfiltered = Vec::with_capacity(data.len());
    match filter {
        0 => {
            unfiltered.extend_from_slice(data);
        }
        1 => for (i, byte) in data.iter().enumerate() {
            match i.checked_sub(bpp) {
                Some(x) => {
                    let b = unfiltered[x];
                    unfiltered.push(byte.wrapping_add(b));
                }
                None => {
                    unfiltered.push(*byte);
                }
            };
        },
        2 => {
            if last_line.is_empty() {
                unfiltered.extend_from_slice(data);
            } else {
                unfiltered.extend(
                    data.iter()
                        .zip(last_line.iter())
                        .map(|(cur, last)| cur.wrapping_add(*last)),
                );
            };
        }
        3 => {
            for (i, byte) in data.iter().enumerate() {
                if last_line.is_empty() {
                    match i.checked_sub(bpp) {
                        Some(x) => {
                            let b = unfiltered[x];
                            unfiltered.push(byte.wrapping_add(b >> 1));
                        }
                        None => {
                            unfiltered.push(*byte);
                        }
                    };
                } else {
                    match i.checked_sub(bpp) {
                        Some(x) => {
                            let b = unfiltered[x];
                            unfiltered.push(byte.wrapping_add(
                                ((u16::from(b) + u16::from(last_line[i])) >> 1) as u8,
                            ));
                        }
                        None => {
                            unfiltered.push(byte.wrapping_add(last_line[i] >> 1));
                        }
                    };
                };
            }
        }
        4 => for (i, byte) in data.iter().enumerate() {
            if last_line.is_empty() {
                match i.checked_sub(bpp) {
                    Some(x) => {
                        let b = unfiltered[x];
                        unfiltered.push(byte.wrapping_add(b));
                    }
                    None => {
                        unfiltered.push(*byte);
                    }
                };
            } else {
                match i.checked_sub(bpp) {
                    Some(x) => {
                        let b = unfiltered[x];
                        unfiltered.push(byte.wrapping_add(paeth_predictor(
                            b,
                            last_line[i],
                            last_line[x],
                        )));
                    }
                    None => {
                        unfiltered.push(byte.wrapping_add(last_line[i]));
                    }
                };
            };
        },
        _ => unreachable!(),
    }
    unfiltered
}

fn paeth_predictor(a: u8, b: u8, c: u8) -> u8 {
    let p = i32::from(a) + i32::from(b) - i32::from(c);
    let pa = (p - i32::from(a)).abs();
    let pb = (p - i32::from(b)).abs();
    let pc = (p - i32::from(c)).abs();
    if pa <= pb && pa <= pc {
        a
    } else if pb <= pc {
        b
    } else {
        c
    }
}
