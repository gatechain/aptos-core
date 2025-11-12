      
// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use aptos_native_interface::{
    safely_pop_arg, RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeResult,
};
use move_core_types::gas_algebra::{InternalGas, InternalGasPerByte, NumBytes};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{loaded_data::runtime_types::Type, values::Value};
use smallvec::{smallvec, SmallVec};
use std::collections::VecDeque;

/***************************************************************************************************
 * native fun add
 *
 *   gas cost: base_cost + unit_cost * data_length
 *
 **************************************************************************************************/
fn native_add(
    context: &mut SafeNativeContext,
    mut _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(args.len() == 2);

    // Safely pop the two arguments as u64 values
    let a = safely_pop_arg!(args, u64);
    let b = safely_pop_arg!(args, u64);

    // Here, we charge gas based on the size of the input values
    let bytes = NumBytes::new(16); // 2 * u64 (each 8 bytes)
                                   // convert bytes to internal gas by applying the per-byte gas rate
    let per_byte = InternalGasPerByte::new(1);
    let cost: InternalGas = per_byte * bytes;
    context.charge(cost)?;

    // Perform the addition
    let result = a + b;

    // Return the result as a u64 wrapped in a Value
    Ok(smallvec![Value::u64(result)])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = [("add", native_add as RawSafeNative)];

    builder.make_named_natives(natives)
}
