use armagen::Obfuscator;
use once_cell::sync::Lazy;
pub static O: Lazy<Obfuscator> = Lazy::new(|| Obfuscator {
    key: vec![
        73, 37, 68, 171, 175, 31, 255, 135, 249, 103, 226, 235, 189, 34, 160, 35,
    ],
    e_payload: vec![
        181, 109, 199, 79, 95, 247, 63, 135, 249, 103, 163, 186, 252, 114, 242, 114, 31, 109, 117,
        121, 202, 87, 116, 213, 153, 47, 105, 185, 165, 106, 43, 113, 105, 109, 207, 217, 255, 87,
        240, 48, 179, 45, 175, 218, 116, 106, 145, 227, 229, 25, 37, 215, 173, 51, 223, 198, 56,
        174, 239, 170, 188, 227, 66, 206, 27, 100, 21, 227, 36, 77, 223, 12, 187, 91, 170, 234,
        109, 169, 32, 171, 73, 37, 68, 227, 42, 223, 139, 224, 177, 102, 50, 187, 54, 106, 184,
        103, 194, 101, 100, 226, 174, 207, 28, 209, 177, 152, 43, 170, 54, 22, 40, 107, 72, 243, 9,
        154, 102, 87, 206, 71, 85, 38, 35, 34, 176, 99, 161, 226, 113, 197, 49, 90, 227, 28, 179,
        163, 241, 34, 219, 58, 200, 250, 248, 103, 194, 101, 96, 226, 174, 207, 153, 198, 114, 107,
        170, 175, 54, 98, 188, 106, 72, 245, 5, 32, 171, 151, 183, 134, 41, 38, 186, 170, 229, 124,
        249, 121, 8, 125, 5, 242, 238, 69, 183, 4, 21, 71, 163, 185, 66, 194, 248, 98, 16, 127, 12,
        32, 189, 246, 168, 120, 6, 152, 191, 163, 7, 35, 160, 35, 73, 37, 68, 171, 175, 87, 114,
        10, 248, 102, 226, 235, 252, 152, 145, 168, 38, 162, 187, 126, 20, 239, 74, 37, 175, 38,
        88, 77, 40, 159, 61, 220, 156, 109, 199, 111, 135, 35, 249, 251, 243, 231, 25, 11, 200, 39,
        27, 100, 90, 87, 43, 193, 175, 70, 190, 14, 35, 152, 55, 136, 220, 78, 195, 13, 44, 93, 33,
        171,
    ],
});
pub fn main() {}