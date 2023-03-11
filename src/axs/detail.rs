use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Detail {
    title: String,
    body: Vec<u8>,
}


pub trait AsDetail {
    type Body;
    fn title(&self) -> &str;
    fn body(&self) -> &Self::Body;
}

pub trait ToDetail {
    fn to_detail(&self) -> Detail;
}

impl<T: AsDetail> ToDetail for T
where
    T::Body: AsRef<[u8]>,
{
    fn to_detail(&self) -> Detail {
        Detail {
            title: self.title().to_string(),
            body: self.body().as_ref().to_vec(),
        }
    }
}

impl Detail {
    pub fn from<T: ToDetail>(&self, item: T) -> Self {
        item.to_detail()
    }
    
    pub fn body_i8(title: String, body: i8) -> Self {
        Self {
            title,
            body: body.to_le_bytes().to_vec(),
        }
    }

    pub fn body_i16(title: String, body: i16) -> Self {
        Self {
            title,
            body: body.to_le_bytes().to_vec(),
        }
    }
    pub fn body_i32(title: String, body: i32) -> Self {
        Self {
            title,
            body: body.to_le_bytes().to_vec(),
        }
    }

    pub fn body_i64(title: String, body: i64) -> Self {
        Self {
            title,
            body: body.to_le_bytes().to_vec(),
        }
    }

    pub fn body_i128(title: String, body: i128) -> Self {
        Self {
            title,
            body: body.to_le_bytes().to_vec(),
        }
    }
    pub fn body_isize(title: String, body: isize) -> Self {
        Self {
            title,
            body: body.to_le_bytes().to_vec(),
        }
    }
    pub fn body_u8(title: String, body: u8) -> Self {
        Self {
            title,
            body: body.to_le_bytes().to_vec(),
        }
    }
    pub fn body_u16(title: String, body: u16) -> Self {
        Self {
            title,
            body: body.to_le_bytes().to_vec(),
        }
    }
    pub fn body_u32(title: String, body: u32) -> Self {
        Self {
            title,
            body: body.to_le_bytes().to_vec(),
        }
    }

    pub fn body_u64(title: String, body: u64) -> Self {
        Self {
            title,
            body: body.to_le_bytes().to_vec(),
        }
    }

    pub fn body_u128(title: String, body: u128) -> Self {
        Self {
            title,
            body: body.to_le_bytes().to_vec(),
        }
    }

    pub fn body_usize(title: String, body: usize) -> Self {
        Self {
            title,
            body: body.to_le_bytes().to_vec(),
        }
    }

    pub fn body_string(title: String, body: String) -> Self {
        Self {
            title,
            body: body.into_bytes(),
        }
    }

    pub fn body_bool(title: String, body: bool) -> Self {
        Self {
            title,
            body: match body {
                true => vec![u8::MAX],
                false => vec![u8::MIN],
            },
        }
    }
}
