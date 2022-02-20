//! UNM Resolver: Crypto: kuwo_des
//!
//! WIP - Not completed yet!

// FIXME: add tests
// FIXME: add decrypt function
// FIXME: implement well

static SECRET_KEY: &[u8; 8] = b"ylzsxkwm";
static ARRAY_MASK: &[u64; 64] = &[
    (0x0000000000000001),
    (0x0000000000000002),
    (0x0000000000000004),
    (0x0000000000000008),
    (0x0000000000000010),
    (0x0000000000000020),
    (0x0000000000000040),
    (0x0000000000000080),
    (0x0000000000000100),
    (0x0000000000000200),
    (0x0000000000000400),
    (0x0000000000000800),
    (0x0000000000001000),
    (0x0000000000002000),
    (0x0000000000004000),
    (0x0000000000008000),
    (0x0000000000010000),
    (0x0000000000020000),
    (0x0000000000040000),
    (0x0000000000080000),
    (0x0000000000100000),
    (0x0000000000200000),
    (0x0000000000400000),
    (0x0000000000800000),
    (0x0000000001000000),
    (0x0000000002000000),
    (0x0000000004000000),
    (0x0000000008000000),
    (0x0000000010000000),
    (0x0000000020000000),
    (0x0000000040000000),
    (0x0000000080000000),
    (0x0000000100000000),
    (0x0000000200000000),
    (0x0000000400000000),
    (0x0000000800000000),
    (0x0000001000000000),
    (0x0000002000000000),
    (0x0000004000000000),
    (0x0000008000000000),
    (0x0000010000000000),
    (0x0000020000000000),
    (0x0000040000000000),
    (0x0000080000000000),
    (0x0000100000000000),
    (0x0000200000000000),
    (0x0000400000000000),
    (0x0000800000000000),
    (0x0001000000000000),
    (0x0002000000000000),
    (0x0004000000000000),
    (0x0008000000000000),
    (0x0010000000000000),
    (0x0020000000000000),
    (0x0040000000000000),
    (0x0080000000000000),
    (0x0100000000000000),
    (0x0200000000000000),
    (0x0400000000000000),
    (0x0800000000000000),
    (0x1000000000000000),
    (0x2000000000000000),
    (0x4000000000000000),
    (0x8000000000000000),
];
static ARRAY_IP: [i32; 64] = [
    57, 49, 41, 33, 25, 17, 9, 1, 59, 51, 43, 35, 27, 19, 11, 3, 61, 53, 45, 37, 29, 21, 13, 5, 63,
    55, 47, 39, 31, 23, 15, 7, 56, 48, 40, 32, 24, 16, 8, 0, 58, 50, 42, 34, 26, 18, 10, 2, 60, 52,
    44, 36, 28, 20, 12, 4, 62, 54, 46, 38, 30, 22, 14, 6,
];
static ARRAY_E: [i32; 64] = [
    31, 0, 1, 2, 3, 4, -1, -1, 3, 4, 5, 6, 7, 8, -1, -1, 7, 8, 9, 10, 11, 12, -1, -1, 11, 12, 13,
    14, 15, 16, -1, -1, 15, 16, 17, 18, 19, 20, -1, -1, 19, 20, 21, 22, 23, 24, -1, -1, 23, 24, 25,
    26, 27, 28, -1, -1, 27, 28, 29, 30, 31, 30, -1, -1,
];
static MATRIX_NS_BOX: [[u64; 64]; 8] = [
    [
        14, 4, 3, 15, 2, 13, 5, 3, 13, 14, 6, 9, 11, 2, 0, 5, 4, 1, 10, 12, 15, 6, 9, 10, 1, 8, 12,
        7, 8, 11, 7, 0, 0, 15, 10, 5, 14, 4, 9, 10, 7, 8, 12, 3, 13, 1, 3, 6, 15, 12, 6, 11, 2, 9,
        5, 0, 4, 2, 11, 14, 1, 7, 8, 13,
    ],
    [
        15, 0, 9, 5, 6, 10, 12, 9, 8, 7, 2, 12, 3, 13, 5, 2, 1, 14, 7, 8, 11, 4, 0, 3, 14, 11, 13,
        6, 4, 1, 10, 15, 3, 13, 12, 11, 15, 3, 6, 0, 4, 10, 1, 7, 8, 4, 11, 14, 13, 8, 0, 6, 2, 15,
        9, 5, 7, 1, 10, 12, 14, 2, 5, 9,
    ],
    [
        10, 13, 1, 11, 6, 8, 11, 5, 9, 4, 12, 2, 15, 3, 2, 14, 0, 6, 13, 1, 3, 15, 4, 10, 14, 9, 7,
        12, 5, 0, 8, 7, 13, 1, 2, 4, 3, 6, 12, 11, 0, 13, 5, 14, 6, 8, 15, 2, 7, 10, 8, 15, 4, 9,
        11, 5, 9, 0, 14, 3, 10, 7, 1, 12,
    ],
    [
        7, 10, 1, 15, 0, 12, 11, 5, 14, 9, 8, 3, 9, 7, 4, 8, 13, 6, 2, 1, 6, 11, 12, 2, 3, 0, 5,
        14, 10, 13, 15, 4, 13, 3, 4, 9, 6, 10, 1, 12, 11, 0, 2, 5, 0, 13, 14, 2, 8, 15, 7, 4, 15,
        1, 10, 7, 5, 6, 12, 11, 3, 8, 9, 14,
    ],
    [
        2, 4, 8, 15, 7, 10, 13, 6, 4, 1, 3, 12, 11, 7, 14, 0, 12, 2, 5, 9, 10, 13, 0, 3, 1, 11, 15,
        5, 6, 8, 9, 14, 14, 11, 5, 6, 4, 1, 3, 10, 2, 12, 15, 0, 13, 2, 8, 5, 11, 8, 0, 15, 7, 14,
        9, 4, 12, 7, 10, 9, 1, 13, 6, 3,
    ],
    [
        12, 9, 0, 7, 9, 2, 14, 1, 10, 15, 3, 4, 6, 12, 5, 11, 1, 14, 13, 0, 2, 8, 7, 13, 15, 5, 4,
        10, 8, 3, 11, 6, 10, 4, 6, 11, 7, 9, 0, 6, 4, 2, 13, 1, 9, 15, 3, 8, 15, 3, 1, 14, 12, 5,
        11, 0, 2, 12, 14, 7, 5, 10, 8, 13,
    ],
    [
        4, 1, 3, 10, 15, 12, 5, 0, 2, 11, 9, 6, 8, 7, 6, 9, 11, 4, 12, 15, 0, 3, 10, 5, 14, 13, 7,
        8, 13, 14, 1, 2, 13, 6, 14, 9, 4, 1, 2, 14, 11, 13, 5, 0, 1, 10, 8, 3, 0, 11, 3, 5, 9, 4,
        15, 2, 7, 8, 12, 15, 10, 7, 6, 12,
    ],
    [
        13, 7, 10, 0, 6, 9, 5, 15, 8, 4, 3, 10, 11, 14, 12, 5, 2, 11, 9, 6, 15, 12, 0, 3, 4, 1, 14,
        13, 1, 2, 7, 8, 1, 2, 12, 15, 10, 4, 0, 3, 13, 14, 6, 9, 7, 8, 9, 6, 15, 1, 5, 12, 3, 10,
        14, 5, 8, 7, 11, 0, 4, 13, 2, 11,
    ],
];
static ARRAY_P: [i32; 32] = [
    15, 6, 19, 20, 28, 11, 27, 16, 0, 14, 22, 25, 4, 17, 30, 9, 1, 7, 23, 13, 31, 26, 2, 8, 18, 12,
    29, 5, 21, 10, 3, 24,
];
static ARRAY_IP1: [i32; 64] = [
    39, 7, 47, 15, 55, 23, 63, 31, 38, 6, 46, 14, 54, 22, 62, 30, 37, 5, 45, 13, 53, 21, 61, 29,
    36, 4, 44, 12, 52, 20, 60, 28, 35, 3, 43, 11, 51, 19, 59, 27, 34, 2, 42, 10, 50, 18, 58, 26,
    33, 1, 41, 9, 49, 17, 57, 25, 32, 0, 40, 8, 48, 16, 56, 24,
];
static ARRAY_PC1: [i32; 56] = [
    56, 48, 40, 32, 24, 16, 8, 0, 57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26, 18, 10, 2, 59,
    51, 43, 35, 62, 54, 46, 38, 30, 22, 14, 6, 61, 53, 45, 37, 29, 21, 13, 5, 60, 52, 44, 36, 28,
    20, 12, 4, 27, 19, 11, 3,
];
static ARRAY_PC2: [i32; 64] = [
    13, 16, 10, 23, 0, 4, -1, -1, 2, 27, 14, 5, 20, 9, -1, -1, 22, 18, 11, 3, 25, 7, -1, -1, 15, 6,
    26, 19, 12, 1, -1, -1, 40, 51, 30, 36, 46, 54, -1, -1, 29, 39, 50, 44, 32, 47, -1, -1, 43, 48,
    38, 55, 33, 52, -1, -1, 45, 41, 49, 35, 28, 31, -1, -1,
];
static ARRAY_LS: [i32; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];
static ARRAY_LS_MASK: [u64; 3] = [0x0000000000000000, 0x0000000000100001, 0x0000000000300003];

/// Transform the bit with the specified array mask.
pub fn bit_transform(array: &'static [i32], source: u64) -> u64 {
    let len = array.len();
    let mut dest = 0u64;

    for bti in 0..len {
        if array[bti] >= 0 && (source & ARRAY_MASK[array[bti] as usize]) != 0 {
            dest |= ARRAY_MASK[bti];
        }
    }

    dest
}

fn des_sub_keys(key: u64, k: &mut [u64; 16], decrypt: bool) {
    /* PC-1变换 */
    let mut temp: u64 = bit_transform(&ARRAY_PC1, key);
    for j in 0..16 {
        {
            /* 循环左移 */
            let source = temp;
            let offset = ARRAY_LS[j] as usize;
            temp = ((source & ARRAY_LS_MASK[offset]) << (28 - offset))
                | ((source & !ARRAY_LS_MASK[offset]) >> offset);
        }
        /* PC-2变换 */
        // 要初始化k的元素为0
        k[j] = bit_transform(&ARRAY_PC2, temp);
    }

    /* 如果解密则反转子密钥顺序 */
    if decrypt {
        let mut t: u64;
        for j in 0..8 {
            t = k[j];
            k[j] = k[15 - j];
            k[15 - j] = t;
        }
    }
}

static HIG32_MASK: u64 = 0xffffffff00000000;
static LOW32_MASK: u64 = 0x00000000ffffffff;

fn des64(sub_keys: &[u64], data: u64) -> u64 {
    let mut out = bit_transform(&ARRAY_IP, data);
    let mut src2: [u64; 2] = [out & LOW32_MASK, (out & HIG32_MASK) >> 32];
    let mut l: u64;
    let mut r: u64;
    let mut r8: [u64; 8] = [0u64; 8];
    /* 主迭代 */
    for sub_key in sub_keys.iter().take(16) {
        /* F变换开始 */
        r = src2[1];
        /* E变换 */
        r = bit_transform(&ARRAY_E, r);
        /* 与子密钥异或 */
        r ^= sub_key;
        /* S盒变换 */
        {
            for (k, item) in r8.iter_mut().enumerate() {
                *item = 0xff & (r >> (k * 8));
            }

            let mut s_out: u64 = 0;
            for sbi in (0..=7).rev() {
                s_out <<= 4;

                s_out |= MATRIX_NS_BOX[sbi][r8[sbi] as usize];
            }
            r = s_out;
        }
        /* P变换 */
        r = bit_transform(&ARRAY_P, r);
        /* f变换完成 */
        l = src2[0];
        src2[0] = src2[1];
        src2[1] = (l ^ r) & LOW32_MASK;
    }
    /* 交换高低32位 */
    src2.swap(0, 1);
    /* IP-1变换 */
    out = (src2[1] << 32 & HIG32_MASK) | (LOW32_MASK & src2[0]);
    out = bit_transform(&ARRAY_IP1, out);

    out
}

fn encrypt_private(src: &[u8], key: &[u8; 8]) -> Vec<u8> {
    let src_len = src.len();
    let mut key_l: u64 = 0;
    for (i, key_item) in key.iter().enumerate().take(8) {
        key_l |= (key_item << (i * 8)) as u64;
    }
    let num = src_len >> 3;
    // 子密钥（临时数据）
    let mut sub_key = [0u64; 16];

    des_sub_keys(key_l, &mut sub_key, true);

    // 加密
    let mut p_src = vec![0u64; num];
    for i in 0..num {
        for j in 0..8 {
            p_src[i] |= (src[i * 8 + j] as u64) << (j * 8)
        }
    }
    let p_encyrpt_length = ((num + 1) * 8 + 1) / 8;
    //存放密文
    let mut p_encrypt = vec![0u64; p_encyrpt_length];
    // 计算前部的数据块(除了最后一部分)
    for i in 0..num {
        p_encrypt[i] = des64(&sub_key, p_src[i]);
    }

    // 保存多出来的字节
    let sz_tail = src[num * 8..].to_vec();

    let mut tail64 = 0u64;
    // 处理结尾处不够8个字节的部分
    let tail_num = src_len % 8;
    for (i, sz_tail_item) in sz_tail.iter().enumerate().take(tail_num) {
        tail64 |= (*sz_tail_item as u64) << (i * 8)
    }
    // 计算多出的那一位(最后一位)
    p_encrypt[num] = des64(&sub_key, tail64);
    let mut result = vec![0u8; p_encrypt.len() * 8];
    let mut temp = 0;
    let ff: u64 = 255;
    let mut enc: u64;
    // 将密文转为字节型
    for p_encrypt_item in &p_encrypt {
        for j in 0..8 {
            enc = ff & (*p_encrypt_item << (j * 8));
            result[temp] = enc as u8;
            temp += 1;
        }
    }
    result
}

/// Encrypt `src` with KuwoDES.
pub fn encrypt(src: &[u8]) -> Vec<u8> {
    encrypt_private(src, SECRET_KEY)
}
