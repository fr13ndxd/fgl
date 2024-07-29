pub mod button;
pub mod icon;
pub mod label;
pub mod scale;
use gtk4::prelude::WidgetExt;

pub trait WidgetOptions {
    fn toggle_classname(&self, class_name: &str, enable: bool);
}

impl<T: WidgetExt> WidgetOptions for T {
    fn toggle_classname(&self, class_name: &str, enable: bool) {
        if enable {
            self.add_css_class(class_name);
        } else {
            self.remove_css_class(class_name);
        }
    }
}
