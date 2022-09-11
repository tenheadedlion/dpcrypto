use crate::Result;

pub fn xor<'a, T: 'a>(key: &'a [T], msg: &'a [T]) -> Result<Vec<T>>
where
    &'a T: std::ops::BitXor<Output = T>,
{
    Ok(key.iter().zip(msg).map(|(k, m)| k ^ m).collect::<Vec<T>>())
}
