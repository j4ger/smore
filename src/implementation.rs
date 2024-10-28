use qmetaobject::*;

#[derive(QObject, Default)]
pub struct SmoreState {
    base: qt_base_class!(trait QObject),
    username: qt_property!(QString; NOTIFY username_changed),
    username_changed: qt_signal!(),
}

impl SmoreState {}
