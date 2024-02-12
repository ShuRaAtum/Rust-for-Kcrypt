// Simon 128/256

fn rotl64(x: u64, n: u8) -> u64 {
    x.rotate_left(u32::from(n))
}

fn rotr64(x: u64, n: u8) -> u64 {
    x.rotate_right(u32::from(n))
}

fn f64(x:u64) -> u64 {
    (rotl64(x, 1) & rotl64(x, 8)) ^ rotl64(x, 2)
}

fn key_schedule(mk:&Vec<u64>, rk:&mut Vec<u64>) {
    let c: u64 = 0xfffffffffffffffc;
    let mut z: u64 = 0xfdc94c3a046d678b;

    let mut a: u64 = mk[0];
    let mut b: u64 = mk[1];
    let mut x: u64 = mk[2];
    let mut y: u64 = mk[3];

    for i in 0..16 {
        rk[4*i] = a;
        a = a ^ c ^ ((z&1) as u64) ^ rotr64(y, 3) ^ rotr64(y, 4) ^ b ^ rotr64(b, 1);
        z = z.rotate_right(1);

        rk[4*i+1] = b;
        b = b ^ c ^ ((z&1) as u64) ^ rotr64(a, 3) ^ rotr64(a, 4) ^ x ^ rotr64(x, 1);
        z = z.rotate_right(1);

        rk[4*i+2] = x;
        x = x ^ c ^ ((z&1) as u64) ^ rotr64(b, 3) ^ rotr64(b, 4) ^ y ^ rotr64(y, 1);
        z = z.rotate_right(1);

        rk[4*i+3] = y;
        y = y ^ c ^ ((z&1) as u64) ^ rotr64(x, 3) ^ rotr64(x, 4) ^ a ^ rotr64(a, 1);
        z = z.rotate_right(1);
    }

    rk[64] = a; 
    a = a ^ c ^ 0 ^ rotr64(y, 3) ^ rotr64(y, 4) ^ b ^ rotr64(b, 1);
    rk[65] = b;
    b = b ^ c ^ 1 ^ rotr64(a, 3) ^ rotr64(a, 4) ^ x ^ rotr64(x, 1);
    rk[66] = x;
    x = x ^ c ^ 0 ^ rotr64(b, 3) ^ rotr64(b, 4) ^ y ^ rotr64(y, 1);
    rk[67] = y;
    y = y ^ c ^ 0 ^ rotr64(x, 3) ^ rotr64(x, 4) ^ a ^ rotr64(a, 1);
    rk[68] = a;
    rk[69] = b;
    rk[70] = x;
    rk[71] = y;
}

fn encrypt(pt:&Vec<u64>, rk:&mut Vec<u64>) -> Vec<u64> {
    let mut data: Vec<u64> = pt.to_vec();

    for i in 0..36 {
        data[0] = data[0] ^ f64(data[1]);
        data[0] = data[0] ^ rk[2*i];
        data[1] = data[1] ^ f64(data[0]);
        data[1] = data[1] ^ rk[2*i+1];
    }

    data
}

fn decrypt(ct:&Vec<u64>, rk:&mut Vec<u64>) -> Vec<u64> {
    let mut data: Vec<u64> = ct.to_vec();

    for i in 0..36 {
        data[1] = data[1] ^ f64(data[0]);
        data[1] = data[1] ^ rk[71-2*i];
        data[0] = data[0] ^ f64(data[1]);
        data[0] = data[0] ^ rk[71-(2*i+1)];
    }

    data
}

fn main() {

    let pt: Vec<u64> = vec![0x6d69732061207369, 0x74206e69206d6f6f];
    let mk: Vec<u64> = vec![0x0706050403020100, 0x0f0e0d0c0b0a0908, 0x1716151413121110, 0x1f1e1d1c1b1a1918];
    let mut rk: Vec<u64> = vec![0; 72];
    // ct = 0x8d2b5579afc8a3a0, 0x3bf72a87efe7b868

    key_schedule(&mk, &mut rk);

    println!("SIMON128_256");

    println!("");
    println!("ENC...");

    let data: Vec<u64> = encrypt(&pt, &mut rk);

    println!("");
    println!("CT: 0x{:16x} 0x{:16x}", data[0], data[1]);


    println!("");


    println!("");
    println!("DEC...");

    let data: Vec<u64> = decrypt(&data, &mut rk);

    println!("");
    println!("PT: 0x{:16x} 0x{:16x}", data[0], data[1]);


    println!("");
}
