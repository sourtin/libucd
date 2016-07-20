use core::cmp::Ordering::{Equal, Less, Greater};
pub mod general;

pub fn search_range<S>(table: &[((u8,u8,u8),(u8,u8,u8),S)], cp: char) -> Option<S>
    where S: Clone
{
    let cp = cp as u32;
    match table.binary_search_by(|&((rb1,rb2,rb3), (re1,re2,re3), _)| {
        let rb: u32 = (rb1 as u32)*65536 + (rb2 as u32)*256 + (rb3 as u32);
        let re: u32 = (re1 as u32)*65536 + (re2 as u32)*256 + (re3 as u32);
        if rb <= cp && cp <= re { Equal }
        else if re < cp { Less }
        else { Greater }
    }) {
        Ok(idx) => {
            let (_, _, ref v) = table[idx];
            Some(v.clone())
        },
        _ => None
    }
}

pub fn search<S>(table: &[((u8,u8,u8),S)], cp: char) -> Option<S>
    where S: Clone
{
    let ca = cp as u32;
    match table.binary_search_by(|&((cb1,cb2,cb3), _)| {
        let cb: u32 = (cb1 as u32)*65536 + (cb2 as u32)*256 + (cb3 as u32);
        if cb == ca { Equal }
        else if cb < ca { Less }
        else { Greater }
    }) {
        Ok(idx) => {
            let (_, ref v) = table[idx];
            Some(v.clone())
        },
        _ => None
    }
}
