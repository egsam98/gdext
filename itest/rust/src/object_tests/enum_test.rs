/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::framework::itest;
use godot::builtin::varray;
use godot::classes::input::CursorShape;
use godot::classes::mesh::PrimitiveType;
use godot::classes::{time, ArrayMesh};
use godot::global::{Orientation, Key};
use std::collections::HashSet;

#[itest]
fn enum_ords() {
    use godot::obj::EngineEnum;
    assert_eq!(CursorShape::CURSOR_ARROW.ord(), 0);
    assert_eq!(CursorShape::CURSOR_IBEAM.ord(), 1);
    assert_eq!(CursorShape::CURSOR_POINTING_HAND.ord(), 2);
    assert_eq!(CursorShape::CURSOR_CROSS.ord(), 3);
    assert_eq!(CursorShape::CURSOR_WAIT.ord(), 4);
    assert_eq!(CursorShape::CURSOR_BUSY.ord(), 5);
    assert_eq!(CursorShape::CURSOR_DRAG.ord(), 6);
    assert_eq!(CursorShape::CURSOR_CAN_DROP.ord(), 7);
    assert_eq!(CursorShape::CURSOR_FORBIDDEN.ord(), 8);
    assert_eq!(CursorShape::CURSOR_VSIZE.ord(), 9);
    assert_eq!(CursorShape::CURSOR_HSIZE.ord(), 10);
    assert_eq!(CursorShape::CURSOR_BDIAGSIZE.ord(), 11);
    assert_eq!(CursorShape::CURSOR_FDIAGSIZE.ord(), 12);
    assert_eq!(CursorShape::CURSOR_MOVE.ord(), 13);
    assert_eq!(CursorShape::CURSOR_VSPLIT.ord(), 14);
    assert_eq!(CursorShape::CURSOR_HSPLIT.ord(), 15);
    assert_eq!(CursorShape::CURSOR_HELP.ord(), 16);
}

#[itest]
fn enum_equality() {
    // TODO: find 2 overlapping ords in same enum

    // assert_eq!(
    //     file_access::CompressionMode::COMPRESSION_DEFLATE,
    //     file_access::CompressionMode::COMPRESSION_DEFLATE
    // );
}

#[itest]
fn enum_hash() {
    let mut months = HashSet::new();
    months.insert(time::Month::MONTH_JANUARY);
    months.insert(time::Month::MONTH_FEBRUARY);
    months.insert(time::Month::MONTH_MARCH);
    months.insert(time::Month::MONTH_APRIL);
    months.insert(time::Month::MONTH_MAY);
    months.insert(time::Month::MONTH_JUNE);
    months.insert(time::Month::MONTH_JULY);
    months.insert(time::Month::MONTH_AUGUST);
    months.insert(time::Month::MONTH_SEPTEMBER);
    months.insert(time::Month::MONTH_OCTOBER);
    months.insert(time::Month::MONTH_NOVEMBER);
    months.insert(time::Month::MONTH_DECEMBER);

    assert_eq!(months.len(), 12);
}

// Testing https://github.com/godot-rust/gdext/issues/335
// This fails upon calling the function, we don't actually need to make a good call.
#[itest]
fn add_surface_from_arrays() {
    let mut mesh = ArrayMesh::new();
    mesh.add_surface_from_arrays(PrimitiveType::PRIMITIVE_TRIANGLES, varray![]);
}

#[itest]
fn enum_as_str() {
    use godot::obj::EngineEnum;
    assert_eq!(Orientation::Vertical.as_str(), "VERTICAL");
    assert_eq!(Orientation::Horizontal.as_str(), "HORIZONTAL");

    assert_eq!(Key::NONE.as_str(), "NONE");
    assert_eq!(Key::SPECIAL.as_str(), "SPECIAL");
    assert_eq!(Key::ESCAPE.as_str(), "ESCAPE");
    assert_eq!(Key::TAB.as_str(), "TAB");
    assert_eq!(Key::A.as_str(), "A");
}

#[itest]
fn enum_godot_name() {
    use godot::obj::EngineEnum;
    assert_eq!(Orientation::Vertical.godot_name(), Orientation::Vertical.as_str());
    assert_eq!(Orientation::Horizontal.godot_name(), Orientation::Vertical.as_str());

    assert_eq!(Key::NONE.godot_name(), "KEY_NONE");
    assert_eq!(Key::SPECIAL.godot_name(), "KEY_SPECIAL");
    assert_eq!(Key::ESCAPE.godot_name(), "KEY_ESCAPE");
    assert_eq!(Key::TAB.godot_name(), "KEY_TAB");
    assert_eq!(Key::A.godot_name(), "KEY_A");
}
