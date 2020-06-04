use crate::{core::Widget, result::*};

pub struct Button {

}

impl IntoResult<Widget> for Button {
    fn into(self) -> MorphResult<Widget> {
        Widget::new()
    }  
}