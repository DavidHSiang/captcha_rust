use ab_glyph::PxScale;

//定义验证码字符.去除了0、O、I、L等容易混淆的字母
pub const BASIC_CHAR: [char; 54] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'M',
    'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

//定义字符串随机的颜色
pub const BASIC_COLOR: [[u8; 3]; 5] = [
    [139, 90, 0],
    [139, 62, 47],
    [0, 0, 139],
    [139, 0, 0],
    [28, 28, 28],
];

//定义背景颜色
pub const WHITE: [u8; 3] = [248, 248, 255];

//定义字体大小
pub const SCALE: PxScale = PxScale { x: 38.0, y: 35.0 };
