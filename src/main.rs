use cstr::cstr;
use qmetaobject::prelude::*;

mod implementation;

qrc!(app_resources,
    "smore" {
        "qml/main.qml",
    },
);

fn main() {
    app_resources();
    qml_register_type::<implementation::SmoreState>(cstr!("SmoreRs"), 1, 0, cstr!("SmoreState"));
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/smore/qml/main.qml".into());
    engine.exec();
}
