use crate::{nbt_version, NbtReader, NbtTypeConversion, NbtValue};

/// 生成测试数据
pub fn gen_datas(len: usize) -> Vec<u8> {
    let mut datas = Vec::with_capacity(len);
    for i in 0..len {
        datas.push(i as u8);
    }
    datas
}

mod safe_test {
    use super::*;

    #[test]
    fn basic_init() {
        let mut data = vec![0x01, 0x02, 0x03, 0x04];
        let reader = NbtReader::new(&mut data);
        assert_eq!(reader.cursor, 0);
        let same_data = vec![0x01, 0x02, 0x03, 0x04];
        assert_eq!(reader.data, &same_data);
    }

    #[test]
    fn read_x8() {
        let mut data: Vec<u8> = vec![0x01, 0x02, i8::MIN as u8, u8::MAX];
        let mut reader = NbtReader::new(data.as_mut_slice());
        assert_eq!(reader.read_i8(), 0x01);
        assert_eq!(reader.cursor, 1);
        assert_eq!(reader.read_u8(), 0x02);
        assert_eq!(reader.cursor, 2);
        assert_eq!(reader.read_i8(), i8::MIN);
        assert_eq!(reader.cursor, 3);
        assert_eq!(reader.read_u8(), u8::MAX);
    }

    #[test]
    fn read_x16() {
        let mut data = vec![0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04];
        data.extend(i16::MIN.to_be_bytes());
        data.extend(i16::MAX.to_be_bytes());
        let mut reader = NbtReader::new(&mut data);
        assert_eq!(reader.read_be_i16(), 0x0102);
        assert_eq!(reader.cursor, 2);
        assert_eq!(reader.read_be_u16(), 0x0304);
        assert_eq!(reader.cursor, 4);
        assert_eq!(reader.read_le_i16(), 0x0201);
        assert_eq!(reader.cursor, 6);
        assert_eq!(reader.read_le_u16(), 0x0403);
        assert_eq!(reader.cursor, 8);
        assert_eq!(reader.read_be_i16(), i16::MIN);
        assert_eq!(reader.cursor, 10);
        assert_eq!(reader.read_be_i16(), i16::MAX);
        assert_eq!(reader.cursor, 12);
    }

    #[test]
    fn read_x32() {
        let mut data = vec![
            0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x02,
            0x03, 0x04,
        ];
        let mut reader = NbtReader::new(&mut data);
        assert_eq!(reader.read_be_i32(), 0x01020304);
        assert_eq!(reader.cursor, 4);
        assert_eq!(reader.read_be_u32(), 0x01020304);
        assert_eq!(reader.cursor, 8);
        assert_eq!(reader.read_le_i32(), 0x04030201);
        assert_eq!(reader.cursor, 12);
        assert_eq!(reader.read_le_u32(), 0x04030201);
        assert_eq!(reader.cursor, 16);
    }

    #[test]
    fn read_x64() {
        let mut data = vec![
            0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x02,
            0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04,
            0x01, 0x02, 0x03, 0x04,
        ];
        let mut reader = NbtReader::new(&mut data);
        assert_eq!(reader.read_be_i64(), 0x0102030401020304);
        assert_eq!(reader.cursor, 8);
        assert_eq!(reader.read_be_u64(), 0x0102030401020304);
        assert_eq!(reader.cursor, 16);
        assert_eq!(reader.read_le_i64(), 0x0403020104030201);
        assert_eq!(reader.cursor, 24);
        assert_eq!(reader.read_le_u64(), 0x0403020104030201);
        assert_eq!(reader.cursor, 32);
    }

    #[test]
    fn read_fxx() {
        let mut data = Vec::with_capacity(12);
        data.extend_from_slice(&std::f32::consts::PI.to_be_bytes());
        data.extend_from_slice(&std::f64::consts::PI.to_be_bytes());
        data.extend_from_slice(&std::f32::consts::PI.to_le_bytes());
        data.extend_from_slice(&std::f64::consts::PI.to_le_bytes());
        println!("{:?}", data);
        let mut reader = NbtReader::new(&mut data);
        assert_eq!(reader.read_be_f32(), std::f32::consts::PI);
        assert_eq!(reader.cursor, 4);
        assert_eq!(reader.read_be_f64(), std::f64::consts::PI);
        assert_eq!(reader.cursor, 12);
        assert_eq!(reader.read_le_f32(), std::f32::consts::PI);
        assert_eq!(reader.cursor, 16);
        assert_eq!(reader.read_le_f64(), std::f64::consts::PI);
        assert_eq!(reader.cursor, 24);
    }

    #[test]
    fn read_string() {
        let mut data = Vec::with_capacity(20);
        data.extend("Hello world!啊？".as_bytes());
        let len = data.len();
        println!("{:?}", data);
        let mut reader = NbtReader::new(&mut data);
        assert_eq!(reader.read_string(len), Ok("Hello world!啊？".to_string()));
        assert_eq!(reader.cursor, 18);
    }
}

#[test]
fn just_format() {
    assert_eq!(15_u8.as_nbt_type_name(), "未知类型(15)");
}

/// unsafe 测试
///
/// 实际内容与 safe_test 一致
///
/// 测试方法就是 safe 读一遍，然后 unsafe 读一遍，然后比较结果
///
/// 反正只要 safe 测试过了，unsafe 直接参考 safe 的测试结果就行
mod unsafe_test {
    use super::*;

    #[test]
    fn read_x16() {
        let mut data = vec![0x01, 0x02, 0x03, 0x04];
        let mut reader = NbtReader::new(&mut data);
        unsafe {
            let value = reader.read_be_i16_unsafe();
            reader.roll_back(2);
            let safe_value = reader.read_be_i16();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 2);
            let value = reader.read_be_u16_unsafe();
            reader.roll_back(2);
            let safe_value = reader.read_be_u16();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 4);
        }
    }

    #[test]
    fn read_x32() {
        let mut data = vec![0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04];
        let mut reader = NbtReader::new(&mut data);
        unsafe {
            let value = reader.read_be_i32_unsafe();
            reader.roll_back(4);
            let safe_value = reader.read_be_i32();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 4);
            let value = reader.read_be_u32_unsafe();
            reader.roll_back(4);
            let safe_value = reader.read_be_u32();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 8);
        }
    }

    #[test]
    fn read_x64() {
        let mut data = vec![
            0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x02,
            0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04,
            0x01, 0x02, 0x03, 0x04,
        ];
        let mut reader = NbtReader::new(&mut data);
        unsafe {
            let value = reader.read_be_i64_unsafe();
            reader.roll_back(8);
            let safe_value = reader.read_be_i64();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 8);

            let value = reader.read_be_u64_unsafe();
            reader.roll_back(8);
            let safe_value = reader.read_be_u64();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 16);

            let value = reader.read_le_i64_unsafe();
            reader.roll_back(8);
            let safe_value = reader.read_le_i64();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 24);

            let value = reader.read_le_u64_unsafe();
            reader.roll_back(8);
            let safe_value = reader.read_le_u64();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 32);
        }
    }

    #[test]
    fn read_fxx() {
        let mut data = Vec::with_capacity(12);
        data.extend_from_slice(&std::f32::consts::PI.to_be_bytes());
        data.extend_from_slice(&std::f64::consts::PI.to_be_bytes());
        data.extend_from_slice(&std::f32::consts::PI.to_le_bytes());
        data.extend_from_slice(&std::f64::consts::PI.to_le_bytes());
        println!("{:?}", data);
        let mut reader = NbtReader::new(&mut data);
        unsafe {
            let value = reader.read_be_f32_unsafe();
            reader.roll_back(4);
            let safe_value = reader.read_be_f32();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 4);

            let value = reader.read_be_f64_unsafe();
            reader.roll_back(8);
            let safe_value = reader.read_be_f64();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 12);

            let value = reader.read_le_f32_unsafe();
            reader.roll_back(4);
            let safe_value = reader.read_le_f32();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 16);

            let value = reader.read_le_f64_unsafe();
            reader.roll_back(8);
            let safe_value = reader.read_le_f64();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 24);
        }
    }

    #[test]
    fn read_array() {
        let mut data = gen_datas(100);
        let mut reader = NbtReader::new(&mut data);
        unsafe {
            let value = reader.read_i8_array_unsafe(100);
            reader.roll_back(100);
            let safe_value = reader.read_i8_array(100);
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 100);
        }
    }

    #[test]
    fn read_i32_array() {
        let mut value = gen_datas(4 * 100);
        let mut reader = NbtReader::new(&mut value);
        unsafe {
            let value = reader.read_i32_array_unsafe(100);
            reader.roll_back(100 * 4);
            let safe_value = reader.read_i32_array(100);
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 100 * 4);
        }
    }

    #[test]
    fn read_i64_array() {
        let mut value = gen_datas(8 * 100);
        let mut reader = NbtReader::new(&mut value);
        unsafe {
            let value = reader.read_i64_array_unsafe(100);
            reader.roll_back(100 * 8);
            let safe_value = reader.read_i64_array(100);
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 100 * 8);
        }
    }
}

mod nbt {
    use super::*;

    #[test]
    fn hello_world() {
        let mut data: [u8; 0x21] = [
            0x0A, 0x00, 0x0B, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64,
            0x08, 0x00, 0x04, 0x6E, 0x61, 0x6D, 0x65, 0x00, 0x09, 0x42, 0x61, 0x6E, 0x61, 0x6E,
            0x72, 0x61, 0x6D, 0x61, 0x00,
        ];
        let data = NbtValue::from_binary::<nbt_version::Java>(&mut data);
        println!("{:?}", data);
        let correct_data = NbtValue::Compound(
            Some("hello world".to_string()),
            vec![("name".to_string(), NbtValue::String("Bananrama".to_string()))],
        );
        assert_eq!(data, Ok(correct_data))
    }

    #[test]
    fn hello_world_java_net() {
        let mut data: [u8; 20] = [
            0x0A, 0x08, 0x00, 0x04, 0x6E, 0x61, 0x6D, 0x65, 0x00, 0x09, 0x42, 0x61, 0x6E, 0x61,
            0x6E, 0x72, 0x61, 0x6D, 0x61, 0x00,
        ];
        let data = NbtValue::from_binary::<nbt_version::JavaNetAfter1_20_2>(&mut data);
        println!("{:?}", data);
        let correct_data = NbtValue::Compound(
            None,
            vec![("name".to_string(), NbtValue::String("Bananrama".to_string()))],
        );
        assert_eq!(data, Ok(correct_data))
    }

    #[test]
    fn big_test() {
        let mut data: [u8; 0x608] = [
            0x0A, 0x00, 0x05, 0x4C, 0x65, 0x76, 0x65, 0x6C, 0x04, 0x00, 0x08, 0x6C, 0x6F, 0x6E,
            0x67, 0x54, 0x65, 0x73, 0x74, 0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x02,
            0x00, 0x09, 0x73, 0x68, 0x6F, 0x72, 0x74, 0x54, 0x65, 0x73, 0x74, 0x7F, 0xFF, 0x08,
            0x00, 0x0A, 0x73, 0x74, 0x72, 0x69, 0x6E, 0x67, 0x54, 0x65, 0x73, 0x74, 0x00, 0x29,
            0x48, 0x45, 0x4C, 0x4C, 0x4F, 0x20, 0x57, 0x4F, 0x52, 0x4C, 0x44, 0x20, 0x54, 0x48,
            0x49, 0x53, 0x20, 0x49, 0x53, 0x20, 0x41, 0x20, 0x54, 0x45, 0x53, 0x54, 0x20, 0x53,
            0x54, 0x52, 0x49, 0x4E, 0x47, 0x20, 0xC3, 0x85, 0xC3, 0x84, 0xC3, 0x96, 0x21, 0x05,
            0x00, 0x09, 0x66, 0x6C, 0x6F, 0x61, 0x74, 0x54, 0x65, 0x73, 0x74, 0x3E, 0xFF, 0x18,
            0x32, 0x03, 0x00, 0x07, 0x69, 0x6E, 0x74, 0x54, 0x65, 0x73, 0x74, 0x7F, 0xFF, 0xFF,
            0xFF, 0x0A, 0x00, 0x14, 0x6E, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x63, 0x6F, 0x6D,
            0x70, 0x6F, 0x75, 0x6E, 0x64, 0x20, 0x74, 0x65, 0x73, 0x74, 0x0A, 0x00, 0x03, 0x68,
            0x61, 0x6D, 0x08, 0x00, 0x04, 0x6E, 0x61, 0x6D, 0x65, 0x00, 0x06, 0x48, 0x61, 0x6D,
            0x70, 0x75, 0x73, 0x05, 0x00, 0x05, 0x76, 0x61, 0x6C, 0x75, 0x65, 0x3F, 0x40, 0x00,
            0x00, 0x00, 0x0A, 0x00, 0x03, 0x65, 0x67, 0x67, 0x08, 0x00, 0x04, 0x6E, 0x61, 0x6D,
            0x65, 0x00, 0x07, 0x45, 0x67, 0x67, 0x62, 0x65, 0x72, 0x74, 0x05, 0x00, 0x05, 0x76,
            0x61, 0x6C, 0x75, 0x65, 0x3F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x0F, 0x6C,
            0x69, 0x73, 0x74, 0x54, 0x65, 0x73, 0x74, 0x20, 0x28, 0x6C, 0x6F, 0x6E, 0x67, 0x29,
            0x04, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0B, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x0D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x0F, 0x09, 0x00, 0x13, 0x6C, 0x69, 0x73, 0x74, 0x54, 0x65, 0x73, 0x74,
            0x20, 0x28, 0x63, 0x6F, 0x6D, 0x70, 0x6F, 0x75, 0x6E, 0x64, 0x29, 0x0A, 0x00, 0x00,
            0x00, 0x02, 0x08, 0x00, 0x04, 0x6E, 0x61, 0x6D, 0x65, 0x00, 0x0F, 0x43, 0x6F, 0x6D,
            0x70, 0x6F, 0x75, 0x6E, 0x64, 0x20, 0x74, 0x61, 0x67, 0x20, 0x23, 0x30, 0x04, 0x00,
            0x0A, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x2D, 0x6F, 0x6E, 0x00, 0x00, 0x01,
            0x26, 0x52, 0x37, 0xD5, 0x8D, 0x00, 0x08, 0x00, 0x04, 0x6E, 0x61, 0x6D, 0x65, 0x00,
            0x0F, 0x43, 0x6F, 0x6D, 0x70, 0x6F, 0x75, 0x6E, 0x64, 0x20, 0x74, 0x61, 0x67, 0x20,
            0x23, 0x31, 0x04, 0x00, 0x0A, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x2D, 0x6F,
            0x6E, 0x00, 0x00, 0x01, 0x26, 0x52, 0x37, 0xD5, 0x8D, 0x00, 0x01, 0x00, 0x08, 0x62,
            0x79, 0x74, 0x65, 0x54, 0x65, 0x73, 0x74, 0x7F, 0x07, 0x00, 0x65, 0x62, 0x79, 0x74,
            0x65, 0x41, 0x72, 0x72, 0x61, 0x79, 0x54, 0x65, 0x73, 0x74, 0x20, 0x28, 0x74, 0x68,
            0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x31, 0x30, 0x30, 0x30, 0x20, 0x76,
            0x61, 0x6C, 0x75, 0x65, 0x73, 0x20, 0x6F, 0x66, 0x20, 0x28, 0x6E, 0x2A, 0x6E, 0x2A,
            0x32, 0x35, 0x35, 0x2B, 0x6E, 0x2A, 0x37, 0x29, 0x25, 0x31, 0x30, 0x30, 0x2C, 0x20,
            0x73, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6E, 0x67, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20,
            0x6E, 0x3D, 0x30, 0x20, 0x28, 0x30, 0x2C, 0x20, 0x36, 0x32, 0x2C, 0x20, 0x33, 0x34,
            0x2C, 0x20, 0x31, 0x36, 0x2C, 0x20, 0x38, 0x2C, 0x20, 0x2E, 0x2E, 0x2E, 0x29, 0x29,
            0x00, 0x00, 0x03, 0xE8, 0x00, 0x3E, 0x22, 0x10, 0x08, 0x0A, 0x16, 0x2C, 0x4C, 0x12,
            0x46, 0x20, 0x04, 0x56, 0x4E, 0x50, 0x5C, 0x0E, 0x2E, 0x58, 0x28, 0x02, 0x4A, 0x38,
            0x30, 0x32, 0x3E, 0x54, 0x10, 0x3A, 0x0A, 0x48, 0x2C, 0x1A, 0x12, 0x14, 0x20, 0x36,
            0x56, 0x1C, 0x50, 0x2A, 0x0E, 0x60, 0x58, 0x5A, 0x02, 0x18, 0x38, 0x62, 0x32, 0x0C,
            0x54, 0x42, 0x3A, 0x3C, 0x48, 0x5E, 0x1A, 0x44, 0x14, 0x52, 0x36, 0x24, 0x1C, 0x1E,
            0x2A, 0x40, 0x60, 0x26, 0x5A, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0C, 0x22, 0x42, 0x08,
            0x3C, 0x16, 0x5E, 0x4C, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4E, 0x1E, 0x5C, 0x40, 0x2E,
            0x26, 0x28, 0x34, 0x4A, 0x06, 0x30, 0x00, 0x3E, 0x22, 0x10, 0x08, 0x0A, 0x16, 0x2C,
            0x4C, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4E, 0x50, 0x5C, 0x0E, 0x2E, 0x58, 0x28, 0x02,
            0x4A, 0x38, 0x30, 0x32, 0x3E, 0x54, 0x10, 0x3A, 0x0A, 0x48, 0x2C, 0x1A, 0x12, 0x14,
            0x20, 0x36, 0x56, 0x1C, 0x50, 0x2A, 0x0E, 0x60, 0x58, 0x5A, 0x02, 0x18, 0x38, 0x62,
            0x32, 0x0C, 0x54, 0x42, 0x3A, 0x3C, 0x48, 0x5E, 0x1A, 0x44, 0x14, 0x52, 0x36, 0x24,
            0x1C, 0x1E, 0x2A, 0x40, 0x60, 0x26, 0x5A, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0C, 0x22,
            0x42, 0x08, 0x3C, 0x16, 0x5E, 0x4C, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4E, 0x1E, 0x5C,
            0x40, 0x2E, 0x26, 0x28, 0x34, 0x4A, 0x06, 0x30, 0x00, 0x3E, 0x22, 0x10, 0x08, 0x0A,
            0x16, 0x2C, 0x4C, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4E, 0x50, 0x5C, 0x0E, 0x2E, 0x58,
            0x28, 0x02, 0x4A, 0x38, 0x30, 0x32, 0x3E, 0x54, 0x10, 0x3A, 0x0A, 0x48, 0x2C, 0x1A,
            0x12, 0x14, 0x20, 0x36, 0x56, 0x1C, 0x50, 0x2A, 0x0E, 0x60, 0x58, 0x5A, 0x02, 0x18,
            0x38, 0x62, 0x32, 0x0C, 0x54, 0x42, 0x3A, 0x3C, 0x48, 0x5E, 0x1A, 0x44, 0x14, 0x52,
            0x36, 0x24, 0x1C, 0x1E, 0x2A, 0x40, 0x60, 0x26, 0x5A, 0x34, 0x18, 0x06, 0x62, 0x00,
            0x0C, 0x22, 0x42, 0x08, 0x3C, 0x16, 0x5E, 0x4C, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4E,
            0x1E, 0x5C, 0x40, 0x2E, 0x26, 0x28, 0x34, 0x4A, 0x06, 0x30, 0x00, 0x3E, 0x22, 0x10,
            0x08, 0x0A, 0x16, 0x2C, 0x4C, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4E, 0x50, 0x5C, 0x0E,
            0x2E, 0x58, 0x28, 0x02, 0x4A, 0x38, 0x30, 0x32, 0x3E, 0x54, 0x10, 0x3A, 0x0A, 0x48,
            0x2C, 0x1A, 0x12, 0x14, 0x20, 0x36, 0x56, 0x1C, 0x50, 0x2A, 0x0E, 0x60, 0x58, 0x5A,
            0x02, 0x18, 0x38, 0x62, 0x32, 0x0C, 0x54, 0x42, 0x3A, 0x3C, 0x48, 0x5E, 0x1A, 0x44,
            0x14, 0x52, 0x36, 0x24, 0x1C, 0x1E, 0x2A, 0x40, 0x60, 0x26, 0x5A, 0x34, 0x18, 0x06,
            0x62, 0x00, 0x0C, 0x22, 0x42, 0x08, 0x3C, 0x16, 0x5E, 0x4C, 0x44, 0x46, 0x52, 0x04,
            0x24, 0x4E, 0x1E, 0x5C, 0x40, 0x2E, 0x26, 0x28, 0x34, 0x4A, 0x06, 0x30, 0x00, 0x3E,
            0x22, 0x10, 0x08, 0x0A, 0x16, 0x2C, 0x4C, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4E, 0x50,
            0x5C, 0x0E, 0x2E, 0x58, 0x28, 0x02, 0x4A, 0x38, 0x30, 0x32, 0x3E, 0x54, 0x10, 0x3A,
            0x0A, 0x48, 0x2C, 0x1A, 0x12, 0x14, 0x20, 0x36, 0x56, 0x1C, 0x50, 0x2A, 0x0E, 0x60,
            0x58, 0x5A, 0x02, 0x18, 0x38, 0x62, 0x32, 0x0C, 0x54, 0x42, 0x3A, 0x3C, 0x48, 0x5E,
            0x1A, 0x44, 0x14, 0x52, 0x36, 0x24, 0x1C, 0x1E, 0x2A, 0x40, 0x60, 0x26, 0x5A, 0x34,
            0x18, 0x06, 0x62, 0x00, 0x0C, 0x22, 0x42, 0x08, 0x3C, 0x16, 0x5E, 0x4C, 0x44, 0x46,
            0x52, 0x04, 0x24, 0x4E, 0x1E, 0x5C, 0x40, 0x2E, 0x26, 0x28, 0x34, 0x4A, 0x06, 0x30,
            0x00, 0x3E, 0x22, 0x10, 0x08, 0x0A, 0x16, 0x2C, 0x4C, 0x12, 0x46, 0x20, 0x04, 0x56,
            0x4E, 0x50, 0x5C, 0x0E, 0x2E, 0x58, 0x28, 0x02, 0x4A, 0x38, 0x30, 0x32, 0x3E, 0x54,
            0x10, 0x3A, 0x0A, 0x48, 0x2C, 0x1A, 0x12, 0x14, 0x20, 0x36, 0x56, 0x1C, 0x50, 0x2A,
            0x0E, 0x60, 0x58, 0x5A, 0x02, 0x18, 0x38, 0x62, 0x32, 0x0C, 0x54, 0x42, 0x3A, 0x3C,
            0x48, 0x5E, 0x1A, 0x44, 0x14, 0x52, 0x36, 0x24, 0x1C, 0x1E, 0x2A, 0x40, 0x60, 0x26,
            0x5A, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0C, 0x22, 0x42, 0x08, 0x3C, 0x16, 0x5E, 0x4C,
            0x44, 0x46, 0x52, 0x04, 0x24, 0x4E, 0x1E, 0x5C, 0x40, 0x2E, 0x26, 0x28, 0x34, 0x4A,
            0x06, 0x30, 0x00, 0x3E, 0x22, 0x10, 0x08, 0x0A, 0x16, 0x2C, 0x4C, 0x12, 0x46, 0x20,
            0x04, 0x56, 0x4E, 0x50, 0x5C, 0x0E, 0x2E, 0x58, 0x28, 0x02, 0x4A, 0x38, 0x30, 0x32,
            0x3E, 0x54, 0x10, 0x3A, 0x0A, 0x48, 0x2C, 0x1A, 0x12, 0x14, 0x20, 0x36, 0x56, 0x1C,
            0x50, 0x2A, 0x0E, 0x60, 0x58, 0x5A, 0x02, 0x18, 0x38, 0x62, 0x32, 0x0C, 0x54, 0x42,
            0x3A, 0x3C, 0x48, 0x5E, 0x1A, 0x44, 0x14, 0x52, 0x36, 0x24, 0x1C, 0x1E, 0x2A, 0x40,
            0x60, 0x26, 0x5A, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0C, 0x22, 0x42, 0x08, 0x3C, 0x16,
            0x5E, 0x4C, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4E, 0x1E, 0x5C, 0x40, 0x2E, 0x26, 0x28,
            0x34, 0x4A, 0x06, 0x30, 0x00, 0x3E, 0x22, 0x10, 0x08, 0x0A, 0x16, 0x2C, 0x4C, 0x12,
            0x46, 0x20, 0x04, 0x56, 0x4E, 0x50, 0x5C, 0x0E, 0x2E, 0x58, 0x28, 0x02, 0x4A, 0x38,
            0x30, 0x32, 0x3E, 0x54, 0x10, 0x3A, 0x0A, 0x48, 0x2C, 0x1A, 0x12, 0x14, 0x20, 0x36,
            0x56, 0x1C, 0x50, 0x2A, 0x0E, 0x60, 0x58, 0x5A, 0x02, 0x18, 0x38, 0x62, 0x32, 0x0C,
            0x54, 0x42, 0x3A, 0x3C, 0x48, 0x5E, 0x1A, 0x44, 0x14, 0x52, 0x36, 0x24, 0x1C, 0x1E,
            0x2A, 0x40, 0x60, 0x26, 0x5A, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0C, 0x22, 0x42, 0x08,
            0x3C, 0x16, 0x5E, 0x4C, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4E, 0x1E, 0x5C, 0x40, 0x2E,
            0x26, 0x28, 0x34, 0x4A, 0x06, 0x30, 0x00, 0x3E, 0x22, 0x10, 0x08, 0x0A, 0x16, 0x2C,
            0x4C, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4E, 0x50, 0x5C, 0x0E, 0x2E, 0x58, 0x28, 0x02,
            0x4A, 0x38, 0x30, 0x32, 0x3E, 0x54, 0x10, 0x3A, 0x0A, 0x48, 0x2C, 0x1A, 0x12, 0x14,
            0x20, 0x36, 0x56, 0x1C, 0x50, 0x2A, 0x0E, 0x60, 0x58, 0x5A, 0x02, 0x18, 0x38, 0x62,
            0x32, 0x0C, 0x54, 0x42, 0x3A, 0x3C, 0x48, 0x5E, 0x1A, 0x44, 0x14, 0x52, 0x36, 0x24,
            0x1C, 0x1E, 0x2A, 0x40, 0x60, 0x26, 0x5A, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0C, 0x22,
            0x42, 0x08, 0x3C, 0x16, 0x5E, 0x4C, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4E, 0x1E, 0x5C,
            0x40, 0x2E, 0x26, 0x28, 0x34, 0x4A, 0x06, 0x30, 0x00, 0x3E, 0x22, 0x10, 0x08, 0x0A,
            0x16, 0x2C, 0x4C, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4E, 0x50, 0x5C, 0x0E, 0x2E, 0x58,
            0x28, 0x02, 0x4A, 0x38, 0x30, 0x32, 0x3E, 0x54, 0x10, 0x3A, 0x0A, 0x48, 0x2C, 0x1A,
            0x12, 0x14, 0x20, 0x36, 0x56, 0x1C, 0x50, 0x2A, 0x0E, 0x60, 0x58, 0x5A, 0x02, 0x18,
            0x38, 0x62, 0x32, 0x0C, 0x54, 0x42, 0x3A, 0x3C, 0x48, 0x5E, 0x1A, 0x44, 0x14, 0x52,
            0x36, 0x24, 0x1C, 0x1E, 0x2A, 0x40, 0x60, 0x26, 0x5A, 0x34, 0x18, 0x06, 0x62, 0x00,
            0x0C, 0x22, 0x42, 0x08, 0x3C, 0x16, 0x5E, 0x4C, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4E,
            0x1E, 0x5C, 0x40, 0x2E, 0x26, 0x28, 0x34, 0x4A, 0x06, 0x30, 0x06, 0x00, 0x0A, 0x64,
            0x6F, 0x75, 0x62, 0x6C, 0x65, 0x54, 0x65, 0x73, 0x74, 0x3F, 0xDF, 0x8F, 0x6B, 0xBB,
            0xFF, 0x6A, 0x5E, 0x00,
        ];
        let value = NbtValue::from_binary::<nbt_version::Java>(&mut data);
        println!("{:?}", value);
        assert!(value.is_ok());
        // 其他版本
    }
}
