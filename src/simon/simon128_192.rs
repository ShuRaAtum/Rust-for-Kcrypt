// Simon 128/192

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
    let mut z: u64 = 0xfc2ce51207a635db;

    let mut a: u64 = mk[0];
    let mut b: u64 = mk[1];
    let mut x: u64 = mk[2];

    for i in 0..21 {
        rk[3*i] = a;
        a = a ^ c ^ ((z&1) as u64) ^ rotr64(x, 3) ^ rotr64(x, 4);
        z = z.rotate_right(1);

        rk[3*i+1] = b;
        b = b ^ c ^ ((z&1) as u64) ^ rotr64(a, 3) ^ rotr64(a, 4);
        z = z.rotate_right(1);

        rk[3*i+2] = x;
        x = x ^ c ^ ((z&1) as u64) ^ rotr64(b, 3) ^ rotr64(b, 4);
        z = z.rotate_right(1);
    }

    rk[63] = a; 
    a = a ^ c ^ 1 ^ rotr64(x, 3) ^ rotr64(x, 4);
    rk[64] = b;
    b = b ^ c ^ 0 ^ rotr64(a, 3) ^ rotr64(a, 4);
    rk[65] = x;
    x = x ^ c ^ 1 ^ rotr64(b, 3) ^ rotr64(b, 4);
    rk[66] = a;
    rk[67] = b;
    rk[68] = x;
}

fn encrypt(pt:&Vec<u64>, rk:&mut Vec<u64>) -> Vec<u64> {
    let mut data: Vec<u64> = pt.to_vec();

    let tmp: u64;

    for i in 0..34 {
        data[0] = data[0] ^ f64(data[1]);
        data[0] = data[0] ^ rk[2*i];
        data[1] = data[1] ^ f64(data[0]);
        data[1] = data[1] ^ rk[2*i+1];
    }

    tmp = data[1];
    data[1] = data[0] ^ f64(data[1]) ^ rk[68];
    data[0] = tmp;

    data
}

fn decrypt(ct:&Vec<u64>, rk:&mut Vec<u64>) -> Vec<u64> {
    let mut data: Vec<u64> = ct.to_vec();

    let tmp: u64;

    tmp = data[0];
    data[0] = data[1] ^ f64(data[0]) ^ rk[68];
    data[1] = tmp;

    for i in 0..34 {
        data[1] = data[1] ^ f64(data[0]);
        data[1] = data[1] ^ rk[67-2*i];
        data[0] = data[0] ^ f64(data[1]);
        data[0] = data[0] ^ rk[67-(2*i+1)];
    }

    data
}

fn main() {

    let pt: Vec<u64> = vec![0x6568772065626972, 0x206572656874206e];
    let mk: Vec<u64> = vec![0x0706050403020100, 0x0f0e0d0c0b0a0908, 0x1716151413121110];
    let mut rk: Vec<u64> = vec![0; 69];
    // ct = 0xc4ac61effcdc0d4f, 0x6c9c8d6e2597b85b

    key_schedule(&mk, &mut rk);

    println!("SIMON128_192");

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
