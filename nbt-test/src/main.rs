fn main() {
    println!("Hello, nbt!");
    // sleep 1s
    println!("============ small test ============");
    big_read_test();
    println!("============ cli test ============");
    cli_read_test();
}

fn big_read_test() {
    let data: [u8; 0x608] = [
        0x0A, 0x00, 0x05, 0x4C, 0x65, 0x76, 0x65, 0x6C, 0x04, 0x00, 0x08, 0x6C, 0x6F, 0x6E, 0x67,
        0x54, 0x65, 0x73, 0x74, 0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x02, 0x00, 0x09,
        0x73, 0x68, 0x6F, 0x72, 0x74, 0x54, 0x65, 0x73, 0x74, 0x7F, 0xFF, 0x08, 0x00, 0x0A, 0x73,
        0x74, 0x72, 0x69, 0x6E, 0x67, 0x54, 0x65, 0x73, 0x74, 0x00, 0x29, 0x48, 0x45, 0x4C, 0x4C,
        0x4F, 0x20, 0x57, 0x4F, 0x52, 0x4C, 0x44, 0x20, 0x54, 0x48, 0x49, 0x53, 0x20, 0x49, 0x53,
        0x20, 0x41, 0x20, 0x54, 0x45, 0x53, 0x54, 0x20, 0x53, 0x54, 0x52, 0x49, 0x4E, 0x47, 0x20,
        0xC3, 0x85, 0xC3, 0x84, 0xC3, 0x96, 0x21, 0x05, 0x00, 0x09, 0x66, 0x6C, 0x6F, 0x61, 0x74,
        0x54, 0x65, 0x73, 0x74, 0x3E, 0xFF, 0x18, 0x32, 0x03, 0x00, 0x07, 0x69, 0x6E, 0x74, 0x54,
        0x65, 0x73, 0x74, 0x7F, 0xFF, 0xFF, 0xFF, 0x0A, 0x00, 0x14, 0x6E, 0x65, 0x73, 0x74, 0x65,
        0x64, 0x20, 0x63, 0x6F, 0x6D, 0x70, 0x6F, 0x75, 0x6E, 0x64, 0x20, 0x74, 0x65, 0x73, 0x74,
        0x0A, 0x00, 0x03, 0x68, 0x61, 0x6D, 0x08, 0x00, 0x04, 0x6E, 0x61, 0x6D, 0x65, 0x00, 0x06,
        0x48, 0x61, 0x6D, 0x70, 0x75, 0x73, 0x05, 0x00, 0x05, 0x76, 0x61, 0x6C, 0x75, 0x65, 0x3F,
        0x40, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x03, 0x65, 0x67, 0x67, 0x08, 0x00, 0x04, 0x6E, 0x61,
        0x6D, 0x65, 0x00, 0x07, 0x45, 0x67, 0x67, 0x62, 0x65, 0x72, 0x74, 0x05, 0x00, 0x05, 0x76,
        0x61, 0x6C, 0x75, 0x65, 0x3F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x0F, 0x6C, 0x69,
        0x73, 0x74, 0x54, 0x65, 0x73, 0x74, 0x20, 0x28, 0x6C, 0x6F, 0x6E, 0x67, 0x29, 0x04, 0x00,
        0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0B, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0D, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0F, 0x09, 0x00,
        0x13, 0x6C, 0x69, 0x73, 0x74, 0x54, 0x65, 0x73, 0x74, 0x20, 0x28, 0x63, 0x6F, 0x6D, 0x70,
        0x6F, 0x75, 0x6E, 0x64, 0x29, 0x0A, 0x00, 0x00, 0x00, 0x02, 0x08, 0x00, 0x04, 0x6E, 0x61,
        0x6D, 0x65, 0x00, 0x0F, 0x43, 0x6F, 0x6D, 0x70, 0x6F, 0x75, 0x6E, 0x64, 0x20, 0x74, 0x61,
        0x67, 0x20, 0x23, 0x30, 0x04, 0x00, 0x0A, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x2D,
        0x6F, 0x6E, 0x00, 0x00, 0x01, 0x26, 0x52, 0x37, 0xD5, 0x8D, 0x00, 0x08, 0x00, 0x04, 0x6E,
        0x61, 0x6D, 0x65, 0x00, 0x0F, 0x43, 0x6F, 0x6D, 0x70, 0x6F, 0x75, 0x6E, 0x64, 0x20, 0x74,
        0x61, 0x67, 0x20, 0x23, 0x31, 0x04, 0x00, 0x0A, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64,
        0x2D, 0x6F, 0x6E, 0x00, 0x00, 0x01, 0x26, 0x52, 0x37, 0xD5, 0x8D, 0x00, 0x01, 0x00, 0x08,
        0x62, 0x79, 0x74, 0x65, 0x54, 0x65, 0x73, 0x74, 0x7F, 0x07, 0x00, 0x65, 0x62, 0x79, 0x74,
        0x65, 0x41, 0x72, 0x72, 0x61, 0x79, 0x54, 0x65, 0x73, 0x74, 0x20, 0x28, 0x74, 0x68, 0x65,
        0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x31, 0x30, 0x30, 0x30, 0x20, 0x76, 0x61, 0x6C,
        0x75, 0x65, 0x73, 0x20, 0x6F, 0x66, 0x20, 0x28, 0x6E, 0x2A, 0x6E, 0x2A, 0x32, 0x35, 0x35,
        0x2B, 0x6E, 0x2A, 0x37, 0x29, 0x25, 0x31, 0x30, 0x30, 0x2C, 0x20, 0x73, 0x74, 0x61, 0x72,
        0x74, 0x69, 0x6E, 0x67, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6E, 0x3D, 0x30, 0x20, 0x28,
        0x30, 0x2C, 0x20, 0x36, 0x32, 0x2C, 0x20, 0x33, 0x34, 0x2C, 0x20, 0x31, 0x36, 0x2C, 0x20,
        0x38, 0x2C, 0x20, 0x2E, 0x2E, 0x2E, 0x29, 0x29, 0x00, 0x00, 0x03, 0xE8, 0x00, 0x3E, 0x22,
        0x10, 0x08, 0x0A, 0x16, 0x2C, 0x4C, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4E, 0x50, 0x5C, 0x0E,
        0x2E, 0x58, 0x28, 0x02, 0x4A, 0x38, 0x30, 0x32, 0x3E, 0x54, 0x10, 0x3A, 0x0A, 0x48, 0x2C,
        0x1A, 0x12, 0x14, 0x20, 0x36, 0x56, 0x1C, 0x50, 0x2A, 0x0E, 0x60, 0x58, 0x5A, 0x02, 0x18,
        0x38, 0x62, 0x32, 0x0C, 0x54, 0x42, 0x3A, 0x3C, 0x48, 0x5E, 0x1A, 0x44, 0x14, 0x52, 0x36,
        0x24, 0x1C, 0x1E, 0x2A, 0x40, 0x60, 0x26, 0x5A, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0C, 0x22,
        0x42, 0x08, 0x3C, 0x16, 0x5E, 0x4C, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4E, 0x1E, 0x5C, 0x40,
        0x2E, 0x26, 0x28, 0x34, 0x4A, 0x06, 0x30, 0x00, 0x3E, 0x22, 0x10, 0x08, 0x0A, 0x16, 0x2C,
        0x4C, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4E, 0x50, 0x5C, 0x0E, 0x2E, 0x58, 0x28, 0x02, 0x4A,
        0x38, 0x30, 0x32, 0x3E, 0x54, 0x10, 0x3A, 0x0A, 0x48, 0x2C, 0x1A, 0x12, 0x14, 0x20, 0x36,
        0x56, 0x1C, 0x50, 0x2A, 0x0E, 0x60, 0x58, 0x5A, 0x02, 0x18, 0x38, 0x62, 0x32, 0x0C, 0x54,
        0x42, 0x3A, 0x3C, 0x48, 0x5E, 0x1A, 0x44, 0x14, 0x52, 0x36, 0x24, 0x1C, 0x1E, 0x2A, 0x40,
        0x60, 0x26, 0x5A, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0C, 0x22, 0x42, 0x08, 0x3C, 0x16, 0x5E,
        0x4C, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4E, 0x1E, 0x5C, 0x40, 0x2E, 0x26, 0x28, 0x34, 0x4A,
        0x06, 0x30, 0x00, 0x3E, 0x22, 0x10, 0x08, 0x0A, 0x16, 0x2C, 0x4C, 0x12, 0x46, 0x20, 0x04,
        0x56, 0x4E, 0x50, 0x5C, 0x0E, 0x2E, 0x58, 0x28, 0x02, 0x4A, 0x38, 0x30, 0x32, 0x3E, 0x54,
        0x10, 0x3A, 0x0A, 0x48, 0x2C, 0x1A, 0x12, 0x14, 0x20, 0x36, 0x56, 0x1C, 0x50, 0x2A, 0x0E,
        0x60, 0x58, 0x5A, 0x02, 0x18, 0x38, 0x62, 0x32, 0x0C, 0x54, 0x42, 0x3A, 0x3C, 0x48, 0x5E,
        0x1A, 0x44, 0x14, 0x52, 0x36, 0x24, 0x1C, 0x1E, 0x2A, 0x40, 0x60, 0x26, 0x5A, 0x34, 0x18,
        0x06, 0x62, 0x00, 0x0C, 0x22, 0x42, 0x08, 0x3C, 0x16, 0x5E, 0x4C, 0x44, 0x46, 0x52, 0x04,
        0x24, 0x4E, 0x1E, 0x5C, 0x40, 0x2E, 0x26, 0x28, 0x34, 0x4A, 0x06, 0x30, 0x00, 0x3E, 0x22,
        0x10, 0x08, 0x0A, 0x16, 0x2C, 0x4C, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4E, 0x50, 0x5C, 0x0E,
        0x2E, 0x58, 0x28, 0x02, 0x4A, 0x38, 0x30, 0x32, 0x3E, 0x54, 0x10, 0x3A, 0x0A, 0x48, 0x2C,
        0x1A, 0x12, 0x14, 0x20, 0x36, 0x56, 0x1C, 0x50, 0x2A, 0x0E, 0x60, 0x58, 0x5A, 0x02, 0x18,
        0x38, 0x62, 0x32, 0x0C, 0x54, 0x42, 0x3A, 0x3C, 0x48, 0x5E, 0x1A, 0x44, 0x14, 0x52, 0x36,
        0x24, 0x1C, 0x1E, 0x2A, 0x40, 0x60, 0x26, 0x5A, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0C, 0x22,
        0x42, 0x08, 0x3C, 0x16, 0x5E, 0x4C, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4E, 0x1E, 0x5C, 0x40,
        0x2E, 0x26, 0x28, 0x34, 0x4A, 0x06, 0x30, 0x00, 0x3E, 0x22, 0x10, 0x08, 0x0A, 0x16, 0x2C,
        0x4C, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4E, 0x50, 0x5C, 0x0E, 0x2E, 0x58, 0x28, 0x02, 0x4A,
        0x38, 0x30, 0x32, 0x3E, 0x54, 0x10, 0x3A, 0x0A, 0x48, 0x2C, 0x1A, 0x12, 0x14, 0x20, 0x36,
        0x56, 0x1C, 0x50, 0x2A, 0x0E, 0x60, 0x58, 0x5A, 0x02, 0x18, 0x38, 0x62, 0x32, 0x0C, 0x54,
        0x42, 0x3A, 0x3C, 0x48, 0x5E, 0x1A, 0x44, 0x14, 0x52, 0x36, 0x24, 0x1C, 0x1E, 0x2A, 0x40,
        0x60, 0x26, 0x5A, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0C, 0x22, 0x42, 0x08, 0x3C, 0x16, 0x5E,
        0x4C, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4E, 0x1E, 0x5C, 0x40, 0x2E, 0x26, 0x28, 0x34, 0x4A,
        0x06, 0x30, 0x00, 0x3E, 0x22, 0x10, 0x08, 0x0A, 0x16, 0x2C, 0x4C, 0x12, 0x46, 0x20, 0x04,
        0x56, 0x4E, 0x50, 0x5C, 0x0E, 0x2E, 0x58, 0x28, 0x02, 0x4A, 0x38, 0x30, 0x32, 0x3E, 0x54,
        0x10, 0x3A, 0x0A, 0x48, 0x2C, 0x1A, 0x12, 0x14, 0x20, 0x36, 0x56, 0x1C, 0x50, 0x2A, 0x0E,
        0x60, 0x58, 0x5A, 0x02, 0x18, 0x38, 0x62, 0x32, 0x0C, 0x54, 0x42, 0x3A, 0x3C, 0x48, 0x5E,
        0x1A, 0x44, 0x14, 0x52, 0x36, 0x24, 0x1C, 0x1E, 0x2A, 0x40, 0x60, 0x26, 0x5A, 0x34, 0x18,
        0x06, 0x62, 0x00, 0x0C, 0x22, 0x42, 0x08, 0x3C, 0x16, 0x5E, 0x4C, 0x44, 0x46, 0x52, 0x04,
        0x24, 0x4E, 0x1E, 0x5C, 0x40, 0x2E, 0x26, 0x28, 0x34, 0x4A, 0x06, 0x30, 0x00, 0x3E, 0x22,
        0x10, 0x08, 0x0A, 0x16, 0x2C, 0x4C, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4E, 0x50, 0x5C, 0x0E,
        0x2E, 0x58, 0x28, 0x02, 0x4A, 0x38, 0x30, 0x32, 0x3E, 0x54, 0x10, 0x3A, 0x0A, 0x48, 0x2C,
        0x1A, 0x12, 0x14, 0x20, 0x36, 0x56, 0x1C, 0x50, 0x2A, 0x0E, 0x60, 0x58, 0x5A, 0x02, 0x18,
        0x38, 0x62, 0x32, 0x0C, 0x54, 0x42, 0x3A, 0x3C, 0x48, 0x5E, 0x1A, 0x44, 0x14, 0x52, 0x36,
        0x24, 0x1C, 0x1E, 0x2A, 0x40, 0x60, 0x26, 0x5A, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0C, 0x22,
        0x42, 0x08, 0x3C, 0x16, 0x5E, 0x4C, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4E, 0x1E, 0x5C, 0x40,
        0x2E, 0x26, 0x28, 0x34, 0x4A, 0x06, 0x30, 0x00, 0x3E, 0x22, 0x10, 0x08, 0x0A, 0x16, 0x2C,
        0x4C, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4E, 0x50, 0x5C, 0x0E, 0x2E, 0x58, 0x28, 0x02, 0x4A,
        0x38, 0x30, 0x32, 0x3E, 0x54, 0x10, 0x3A, 0x0A, 0x48, 0x2C, 0x1A, 0x12, 0x14, 0x20, 0x36,
        0x56, 0x1C, 0x50, 0x2A, 0x0E, 0x60, 0x58, 0x5A, 0x02, 0x18, 0x38, 0x62, 0x32, 0x0C, 0x54,
        0x42, 0x3A, 0x3C, 0x48, 0x5E, 0x1A, 0x44, 0x14, 0x52, 0x36, 0x24, 0x1C, 0x1E, 0x2A, 0x40,
        0x60, 0x26, 0x5A, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0C, 0x22, 0x42, 0x08, 0x3C, 0x16, 0x5E,
        0x4C, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4E, 0x1E, 0x5C, 0x40, 0x2E, 0x26, 0x28, 0x34, 0x4A,
        0x06, 0x30, 0x00, 0x3E, 0x22, 0x10, 0x08, 0x0A, 0x16, 0x2C, 0x4C, 0x12, 0x46, 0x20, 0x04,
        0x56, 0x4E, 0x50, 0x5C, 0x0E, 0x2E, 0x58, 0x28, 0x02, 0x4A, 0x38, 0x30, 0x32, 0x3E, 0x54,
        0x10, 0x3A, 0x0A, 0x48, 0x2C, 0x1A, 0x12, 0x14, 0x20, 0x36, 0x56, 0x1C, 0x50, 0x2A, 0x0E,
        0x60, 0x58, 0x5A, 0x02, 0x18, 0x38, 0x62, 0x32, 0x0C, 0x54, 0x42, 0x3A, 0x3C, 0x48, 0x5E,
        0x1A, 0x44, 0x14, 0x52, 0x36, 0x24, 0x1C, 0x1E, 0x2A, 0x40, 0x60, 0x26, 0x5A, 0x34, 0x18,
        0x06, 0x62, 0x00, 0x0C, 0x22, 0x42, 0x08, 0x3C, 0x16, 0x5E, 0x4C, 0x44, 0x46, 0x52, 0x04,
        0x24, 0x4E, 0x1E, 0x5C, 0x40, 0x2E, 0x26, 0x28, 0x34, 0x4A, 0x06, 0x30, 0x00, 0x3E, 0x22,
        0x10, 0x08, 0x0A, 0x16, 0x2C, 0x4C, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4E, 0x50, 0x5C, 0x0E,
        0x2E, 0x58, 0x28, 0x02, 0x4A, 0x38, 0x30, 0x32, 0x3E, 0x54, 0x10, 0x3A, 0x0A, 0x48, 0x2C,
        0x1A, 0x12, 0x14, 0x20, 0x36, 0x56, 0x1C, 0x50, 0x2A, 0x0E, 0x60, 0x58, 0x5A, 0x02, 0x18,
        0x38, 0x62, 0x32, 0x0C, 0x54, 0x42, 0x3A, 0x3C, 0x48, 0x5E, 0x1A, 0x44, 0x14, 0x52, 0x36,
        0x24, 0x1C, 0x1E, 0x2A, 0x40, 0x60, 0x26, 0x5A, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0C, 0x22,
        0x42, 0x08, 0x3C, 0x16, 0x5E, 0x4C, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4E, 0x1E, 0x5C, 0x40,
        0x2E, 0x26, 0x28, 0x34, 0x4A, 0x06, 0x30, 0x06, 0x00, 0x0A, 0x64, 0x6F, 0x75, 0x62, 0x6C,
        0x65, 0x54, 0x65, 0x73, 0x74, 0x3F, 0xDF, 0x8F, 0x6B, 0xBB, 0xFF, 0x6A, 0x5E, 0x00,
    ];
    read_test(data.to_vec());
}

pub fn print_speed(speed: f64) {
    println!("speed: {} (bytes/s)", speed);
    let speeds = vec![
        speed / 1024.0,
        speed / 1024.0 / 1024.0,
        speed / 1024.0 / 1024.0 / 1024.0,
        speed / 1024.0 / 1024.0 / 1024.0 / 1024.0,
    ];
    // 只输出 > 1024 的
    for (i, speed) in speeds.iter().enumerate() {
        if *speed > 1.0 {
            println!("{:?} ({}/s)", speed, ["KB", "MB", "GB", "TB"][i]);
        }
    }
}

macro_rules! test_lib {
    ($func: block, $name: expr, $len: expr) => {
        std::thread::sleep(std::time::Duration::from_micros(100));
        let start_time = std::time::Instant::now();
        $func
        let end_time = std::time::Instant::now();
        println!("=== {} ===", $name);
        println!("time: {:?}", end_time - start_time);
        let raw_speed = $len as f64 / (end_time - start_time).as_secs_f64();
        print_speed(raw_speed);
        #[cfg(feature = "core_debug")]
        println!("nbt_data: {:#?}", nbt_data);
    };
}

fn test_v1(data: Vec<u8>) {
    let len = data.len();
    std::thread::sleep(std::time::Duration::from_secs(1));
    test_lib!(
        {
            let cursor: shen_nbt1::data::Reader = std::io::Cursor::new(data.as_slice());
            let _nbt_data = shen_nbt1::data::NbtItem::try_from(cursor).unwrap();
        },
        "nbt v1",
        len
    );
}
fn test_v2(data: Vec<u8>) {
    let len = data.len();
    std::thread::sleep(std::time::Duration::from_secs(1));
    test_lib!(
        {
            let _nbt_data = shen_nbt2::Value::from_vec(data);
        },
        "nbt v2",
        len
    );
}

fn test_v3(data: Vec<u8>) {
    let len = data.len();
    std::thread::sleep(std::time::Duration::from_secs(1));
    test_lib!(
        {
            let _nbt_data = shen_nbt3::Value::from_vec(data);
        },
        "nbt v3",
        len
    );
}

fn test_v4(data: Vec<u8>) {
    let len = data.len();
    std::thread::sleep(std::time::Duration::from_secs(1));
    test_lib!(
        {
            let _nbt_data = shen_nbt4::Value::from_vec(data);
        },
        "nbt v4",
        len
    );
}

fn test_v5(mut data: Vec<u8>) {
    let len = data.len();
    std::thread::sleep(std::time::Duration::from_secs(1));
    test_lib!(
        {
            let _nbt_data = shen_nbt5::NbtValue::from_binary(data.as_mut_slice());
        },
        "nbt v5",
        len
    );
}

fn test_fastnbt(data: Vec<u8>) {
    let len = data.len();
    std::thread::sleep(std::time::Duration::from_secs(1));
    test_lib!(
        {
            let _nbt_data: fastnbt::Value = fastnbt::from_bytes(data.as_slice()).unwrap();
        },
        "fastnbt",
        len
    );
}

fn read_test(in_data: Vec<u8>) {
    #[cfg(feature = "debug")]
    println!("data: {:?}", data);

    std::thread::sleep(std::time::Duration::from_secs(1));

    let data = in_data.clone();
    test_v1(data);

    let data = in_data.clone();
    test_v2(data);

    let data = in_data.clone();
    test_v3(data);

    let data = in_data.clone();
    test_v4(data);

    let data = in_data.clone();
    test_v5(data);

    let data = in_data.clone();
    test_fastnbt(data);
}

fn cli_read_test() {
    let mut args = std::env::args();
    // 如果有, 取出
    if let Some(arg) = args.nth(1) {
        let read_data = std::fs::read(&arg).unwrap();
        // read_test(data);
        let len = read_data.len();
        let mut data = read_data;

        std::thread::sleep(std::time::Duration::from_secs(1));
        let start_time = std::time::Instant::now();
        // let nbt_data = shen_nbt4::Value::from_vec(data);
        let nbt_data = shen_nbt5::NbtValue::from_binary(data.as_mut_slice());
        let end_time = std::time::Instant::now();
        println!("=== shen nbt 5 ===");
        print!("time: {:?}", end_time - start_time);
        let raw_speed = len as f64 / (end_time - start_time).as_secs_f64();
        print_speed(raw_speed);
        #[cfg(feature = "core_debug")]
        println!("nbt_data: {:#?}", nbt_data);
        // println!("nbt len {}", nbt_data.as_byte().is_some());
        // test_lib!(
        //     {
        //         let cursor: shen_nbt1::data::Reader = std::io::Cursor::new(data.as_slice());
        //         let nbt_data = shen_nbt1::data::NbtItem::try_from(cursor).unwrap();

        //     }, "nbt v1", len
        // );

        // let data2 = std::fs::read(&arg).unwrap();
        // test_lib!(
        //     {
        //         let nbt_data = shen_nbt2::Value::from_vec(data2);
        //     }, "nbt v2", len
        // );

        // let data3 = std::fs::read(&arg).unwrap();
        // test_lib!(
        //     {
        //         let nbt_data = shen_nbt3::Value::from_vec(data3);
        //     }, "nbt v3", len
        // );

        // let data4 = std::fs::read(&arg).unwrap();
        // test_lib!(
        //     {
        //         let nbt_data = shen_nbt4::Value::from_vec(data);
        //     }, "nbt v4", len
        // );

        // let data5 = std::fs::read(&arg).unwrap();
        // test_lib!(
        //     {
        //         let nbt_data: fastnbt::Value = fastnbt::from_bytes(data5.as_slice()).unwrap();
        //     }, "fastnbt", len
        // );
    } else {
        println!("Usage: cargo run --release -- <file>");
    }
}
