use crate::error::NoModificationAllowedError;

pub struct SVGNumber
{
    value : usize,
    readOnly : bool
}

impl SVGNumber
{
    pub fn setValue(&mut self, newValue : usize) -> Result<(),NoModificationAllowedError>
    {
        match self.readOnly 
        {
            true => { self.value = newValue; Ok(()) },
            false => return Err(NoModificationAllowedError)
        }
    }

    pub fn getValue(&self) -> &usize
    {
        &self.value
    }

    pub fn isReadOnly(&self) -> &bool
    {
        &self.readOnly
    }
}