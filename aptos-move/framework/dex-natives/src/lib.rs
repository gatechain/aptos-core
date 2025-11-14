use aptos_native_interface::{
    RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeResult,
};
use better_any::{Tid, TidAble};
use bytes::Bytes;
use move_core_types::{
    account_address::AccountAddress,
    effects::Op,
    identifier::Identifier,
    language_storage::{StructTag, TypeTag},
    value::{MoveStructLayout, MoveTypeLayout},
};
use move_vm_runtime::{native_extensions::SessionListener, native_functions::NativeFunctionTable};
use move_vm_types::{
    loaded_data::runtime_types::Type,
    resolver::ResourceResolver,
    value_serde::ValueSerDeContext,
    values::{Copyable, GlobalValue, Struct, Value},
};
use smallvec::{smallvec, SmallVec};
use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
    vec,
};
use triomphe::Arc as TriompheArc;

const FEE_RATE: u64 = 997;

pub struct DexChangeSet {
    pub changes:
        BTreeMap<(AccountAddress, StructTag), Op<(Bytes, Option<TriompheArc<MoveTypeLayout>>)>>,
}

#[derive(Tid)]
pub struct NativeDexContext<'a> {
    resolver: &'a dyn ResourceResolver,
    pools: RefCell<BTreeMap<(AccountAddress, StructTag), Pool>>,
}

pub struct Pool {
    layout: TriompheArc<MoveTypeLayout>,
    value: GlobalValue,
}

fn pool_layout() -> MoveTypeLayout {
    MoveTypeLayout::Struct(MoveStructLayout::new(vec![
        MoveTypeLayout::U128,
        MoveTypeLayout::U128,
        MoveTypeLayout::U128,
        MoveTypeLayout::U64,
    ]))
}

impl<'a> NativeDexContext<'a> {
    pub fn new(resolver: &'a dyn ResourceResolver) -> Self {
        Self {
            resolver,
            pools: RefCell::new(BTreeMap::new()),
        }
    }

    fn get_or_create_pool(&self, addr: AccountAddress, tag: &StructTag) -> GlobalValue {
        let key = (addr, tag.clone());
        let mut pools = self.pools.borrow_mut();

        if let Some(pool) = pools.get(&key) {
            return pool.value.deep_copy().unwrap();
        }

        let layout = pool_layout();
        let layout_arc = TriompheArc::new(layout.clone());

        match self.resolver.get_resource_bytes_with_metadata_and_layout(
            &addr,
            tag,
            &[],
            Some(&layout),
        ) {
            Ok((Some(bytes), _)) => {
                let value = deserialize_value(&bytes, &layout);
                let gv = GlobalValue::cached(value).unwrap();

                pools.insert(key, Pool {
                    layout: layout_arc,
                    value: gv.deep_copy().unwrap(),
                });
                gv
            },
            _ => {
                let default_value = Value::struct_(Struct::pack(vec![
                    Value::u128(1),
                    Value::u128(1),
                    Value::u128(1),
                    Value::u64(FEE_RATE),
                ]));

                let mut gv = GlobalValue::none();
                gv.move_to(default_value).expect("move_to should succeed");

                pools.insert(key, Pool {
                    layout: layout_arc,
                    value: gv.deep_copy().unwrap(),
                });
                gv
            },
        }
    }

    pub fn into_change_set(self) -> DexChangeSet {
        let mut changes = BTreeMap::new();

        for (key, pool) in self.pools.into_inner() {
            let layout = pool.layout.as_ref();
            let gv = pool.value;

            if let Some(op) = gv.into_effect() {
                match op {
                    Op::Delete => {
                        changes.insert(key, Op::Delete);
                    },
                    Op::New(value) | Op::Modify(value) => {
                        let (bytes, layout_arc) = serialize_value(&value, &layout);
                        changes.insert(key, Op::New((bytes, layout_arc)));
                    },
                }
            }
        }

        DexChangeSet { changes }
    }
}

impl<'a> SessionListener for NativeDexContext<'a> {
    fn start(&mut self, _: &[u8; 32], _: &[u8], _: u8) {}
    fn finish(&mut self) {}
    fn abort(&mut self) {}
}

pub fn serialize_value(
    val: &Value,
    layout: &MoveTypeLayout,
) -> (Bytes, Option<TriompheArc<MoveTypeLayout>>) {
    let bytes = ValueSerDeContext::new(None)
        .with_delayed_fields_serde()
        .serialize(val, layout)
        .unwrap()
        .unwrap();

    (Bytes::from(bytes), Some(TriompheArc::new(layout.clone())))
}

pub fn deserialize_value(bytes: &[u8], layout: &MoveTypeLayout) -> Value {
    ValueSerDeContext::new(None)
        .with_delayed_fields_serde()
        .deserialize(bytes, layout)
        .unwrap()
}

fn build_pool_tag(ty_tag_a: TypeTag, ty_tag_b: TypeTag) -> StructTag {
    StructTag {
        address: AccountAddress::ONE,
        module: Identifier::new("dex").unwrap(),
        name: Identifier::new("Pool").unwrap(),
        type_args: vec![ty_tag_a, ty_tag_b],
    }
}

pub fn native_create_pool(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    _args: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let type_a = &ty_args[0];
    let type_b = &ty_args[1];
    let tag_a = context.type_to_type_tag(type_a)?;
    let tag_b = context.type_to_type_tag(type_b)?;
    let pool_tag = build_pool_tag(tag_a, tag_b);

    let pool_addr = AccountAddress::ONE;
    let extensions = context.extensions();
    let dex_ctx = extensions.get::<NativeDexContext>();

    let mut gv = dex_ctx.get_or_create_pool(pool_addr, &pool_tag);

    let _v = gv.move_from().unwrap();
    Ok(smallvec![Value::u64(1)])
}

pub fn native_get_pool(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    _args: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let type_a = &ty_args[0];
    let type_b = &ty_args[1];
    let tag_a = context.type_to_type_tag(type_a)?;
    let tag_b = context.type_to_type_tag(type_b)?;
    let pool_tag = build_pool_tag(tag_a, tag_b);

    let pool_addr = AccountAddress::ONE;
    let extensions = context.extensions();
    let dex_ctx = extensions.get::<NativeDexContext>();

    let _gv = dex_ctx.get_or_create_pool(pool_addr, &pool_tag);

    // Note: How to parse/extract the value of gv?
    Ok(smallvec![Value::u64(1)])
}

pub fn dex_natives(
    dex_addr: AccountAddress,
    builder: &mut SafeNativeBuilder,
) -> NativeFunctionTable {
    builder.with_incremental_gas_charging(false, |builder| {
        builder
            .make_named_natives([
                ("create_pool", native_create_pool as RawSafeNative),
                ("get_pool", native_get_pool as RawSafeNative),
            ])
            .map(|(func_name, func)| {
                (
                    dex_addr,
                    Identifier::new("dex").unwrap(),
                    Identifier::new(func_name).unwrap(),
                    func,
                )
            })
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let layout = pool_layout();

        let original = Value::struct_(Struct::pack(vec![
            Value::u128(1),
            Value::u128(1),
            Value::u128(1),
            Value::u64(997),
        ]));

        let (bytes, _) = serialize_value(&original, &layout);

        let recovered = deserialize_value(&bytes, &layout);

        let (bytes2, _) = serialize_value(&recovered, &layout);

        assert_eq!(
            bytes, bytes2,
            "serialize -> deserialize -> serialize not equal"
        );
    }
}
