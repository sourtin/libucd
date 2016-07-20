pub mod general;

pub trait UCDSearch {
    type T;
    fn search(&self, cp: char) -> Option<Self::T>;
}

impl<S> UCDSearch for [((u8,u8,u8), (u8,u8,u8), S)]
    where S: Clone
{
    type T = S;

    fn search(&self, cp: char) -> Option<S> {
        use core::cmp::Ordering::{Equal, Less, Greater};

        let cp = cp as u32;
        match self.binary_search_by(|&((rb1,rb2,rb3), (re1,re2,re3), _)| {
            let rb: u32 = (rb1 as u32)*65536 + (rb2 as u32)*256 + (rb3 as u32);
            let re: u32 = (re1 as u32)*65536 + (re2 as u32)*256 + (re3 as u32);
            if rb <= cp && cp <= re { Equal }
            else if re < cp { Less }
            else { Greater }
        }) {
            Ok(idx) => {
                let (_, _, ref v) = self[idx];
                Some(v.clone())
            },
            _ => None
        }
    }
}
