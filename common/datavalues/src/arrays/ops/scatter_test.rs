// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use common_exception::Result;

use crate::arrays::builders::*;
use crate::arrays::get_list_builder;
use crate::arrays::ops::scatter::ArrayScatter;
use crate::prelude::*;
use crate::DFBooleanArray;
use crate::DFUInt16Array;
use crate::DFUtf8Array;

#[test]
fn test_scatter() -> Result<()> {
    // Test DFUint16Array
    let df_uint16_array = DFUInt16Array::new_from_iter(1u16..11u16);
    // Create the indice array
    let indices = vec![1, 2, 3, 1, 3, 2, 0, 3, 1, 0];
    // The number of rows should be equal to the length of indices
    assert_eq!(df_uint16_array.len(), indices.len());

    let array_vec = unsafe { df_uint16_array.scatter_unchecked(&mut indices.into_iter(), 4)? };
    assert_eq!(&[7u16, 10], &array_vec[0].as_ref().values());
    assert_eq!(&[1u16, 4, 9], &array_vec[1].as_ref().values());
    assert_eq!(&[2u16, 6], &array_vec[2].as_ref().values());
    assert_eq!(&[3u16, 5, 8], &array_vec[3].as_ref().values());

    // Test DFUint16Array
    let df_utf8_array = DFUtf8Array::new_from_slice(&["a", "b", "c", "d"]);
    let indices = vec![1, 0, 1, 1];
    assert_eq!(df_utf8_array.len(), indices.len());

    let array_vec = unsafe { df_utf8_array.scatter_unchecked(&mut indices.into_iter(), 2)? };
    assert_eq!(
        &"b".as_bytes(),
        &array_vec[0].as_ref().value_data().as_slice()
    );
    assert_eq!(
        &"acd".as_bytes(),
        &array_vec[1].as_ref().value_data().as_slice()
    );

    // Test BooleanArray
    let df_bool_array = DFBooleanArray::new_from_slice(&[true, false, true, false]);
    let indices = vec![1, 0, 0, 1];
    assert_eq!(df_bool_array.len(), indices.len());

    let array_vec = unsafe { df_bool_array.scatter_unchecked(&mut indices.into_iter(), 2)? };
    assert_eq!(&[2], &array_vec[0].as_ref().values().as_slice());
    assert_eq!(&[1], &array_vec[1].as_ref().values().as_slice());

    // Test BinaryArray
    let mut binary_builder = BinaryArrayBuilder::new(8);
    binary_builder.append_value(&"12");
    binary_builder.append_value(&"ab");
    binary_builder.append_value(&"c");
    binary_builder.append_value(&"3");
    let df_binary_array = binary_builder.finish();
    let indices = vec![1, 0, 0, 1];
    let array_vec = unsafe { df_binary_array.scatter_unchecked(&mut indices.into_iter(), 2)? };
    assert_eq!(
        [b'a', b'b', b'c'],
        array_vec[0].as_ref().value_data().as_slice()
    );
    assert_eq!(
        [b'1', b'2', b'3'],
        array_vec[1].as_ref().value_data().as_slice()
    );

    // Test ListArray
    let mut builder = get_list_builder(&DataType::UInt16, 12, 3);
    builder.append_series(&Series::new(vec![1_u16, 2, 3]));
    builder.append_series(&Series::new(vec![7_u16, 8, 9]));
    builder.append_series(&Series::new(vec![10_u16, 11, 12]));
    builder.append_series(&Series::new(vec![4_u16, 5, 6]));
    let df_list = builder.finish();

    let indices = vec![1, 0, 0, 1];
    let array_vec = unsafe { df_list.scatter_unchecked(&mut indices.into_iter(), 2)? };

    let expected1 = "PrimitiveArray<UInt16>\n[\n  7,\n  8,\n  9,\n  10,\n  11,\n  12,\n]";
    let expected2 = "PrimitiveArray<UInt16>\n[\n  1,\n  2,\n  3,\n  4,\n  5,\n  6,\n]";
    assert_eq!(expected1, format!("{:?}", array_vec[0].as_ref().values()));
    assert_eq!(expected2, format!("{:?}", array_vec[1].as_ref().values()));

    Ok(())
}
