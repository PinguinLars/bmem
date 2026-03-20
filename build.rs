use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(QmlModule::new("me.ashypinguin.bmem").qml_file("qml/main.qml"))
        .qt_module("Network")
        .qt_module("Qml")
        .qt_module("Gui")
        .files(["src/cxxqt_object.rs"])
        .build();

    // non rust project files
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=qml/main.qml");
}
