/*
 * Bmem: a memory game
 * Copyright (C) 2026 AshyPinguin
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

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
