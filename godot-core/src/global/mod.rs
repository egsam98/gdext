/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Godot global enums, constants and utility functions.
//!
//! See also [Godot docs for `@GlobalScope`](https://docs.godotengine.org/en/stable/classes/class_@globalscope.html#methods).
//!
//! # Builtin-related enums
//!
//! The library ships several additional enums in places where GDScript would use magic numbers. These are co-located with
//! builtin types, in the [`godot::builtin`][crate::builtin] module. The enums are:
//!
//! - Color: [`ColorChannelOrder`][crate::builtin::ColorChannelOrder]
//! - Projection: [`ProjectionEye`][crate::builtin::ProjectionEye], [`ProjectionPlane`][crate::builtin::ProjectionPlane]
//! - Rectangle: [`Side`], [`Corner`] <sub>(godot-generated)</sub>
//! - Rotation: [`EulerOrder`] <sub>(godot-generated)</sub>
//! - Variant: [`VariantType`][crate::builtin::VariantType], [`VariantOperator`][crate::builtin::VariantOperator]
//! - Vector: [`Vector2Axis`][crate::builtin::Vector2Axis], [`Vector3Axis`][crate::builtin::Vector3Axis], [`Vector4Axis`][crate::builtin::Vector4Axis]
//!

mod print;

pub use crate::{godot_error, godot_print, godot_print_rich, godot_script_error, godot_warn};

// Some enums are directly re-exported from crate::builtin.
pub use crate::gen::central::global_enums::*;
pub use crate::gen::utilities::*;

// This is needed for generated classes to find symbols, even those that have been moved to crate::builtin.
use crate::builtin::Variant;
#[allow(unused_imports)] // micromanaging imports for generated code is not fun
pub(crate) use crate::builtin::{Corner, EulerOrder, Side};
use crate::obj::Gd;

// ----------------------------------------------------------------------------------------------------------------------------------------------
// Deprecations

// Reminder: remove #![allow(deprecated)] in utilities.test along with the below functions.

#[deprecated = "Instance utilities in `godot::global` will be removed. Use methods on `Gd` and `InstanceId` instead.\n\
    For detailed reasons, see https://github.com/godot-rust/gdext/pull/892."]
pub fn instance_from_id(instance_id: i64) -> Option<Gd<crate::classes::Object>> {
    crate::gen::utilities::instance_from_id(instance_id)
}

#[deprecated = "Instance utilities in `godot::global` will be removed. Use methods on `Gd` and `InstanceId` instead.\n\
    For detailed reasons, see https://github.com/godot-rust/gdext/pull/892."]
pub fn is_instance_valid(instance: Variant) -> bool {
    crate::gen::utilities::is_instance_valid(&instance)
}

#[deprecated = "Instance utilities in `godot::global` will be removed. Use methods on `Gd` and `InstanceId` instead.\n\
    For detailed reasons, see https://github.com/godot-rust/gdext/pull/892."]
pub fn is_instance_id_valid(instance_id: i64) -> bool {
    crate::gen::utilities::is_instance_id_valid(instance_id)
}
