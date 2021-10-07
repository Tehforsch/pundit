pub trait Named {
    fn get_name(&self) -> &str;
}

pub fn get_by_name<'a, T: Named>(items: &'a [T], name: &str) -> Option<&'a T> {
    items.iter().find(|item| item.get_name() == name)
}
