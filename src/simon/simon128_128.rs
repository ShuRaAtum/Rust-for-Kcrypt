// Simon 128/128

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
    let mut z: u64 = 0x7369f885192c0ef5;

    let mut a: u64 = mk[0];
    let mut b: u64 = mk[1];

    for i in 0..32 {
        rk[2*i] = a;
        a = a ^ c ^ ((z&1) as u64) ^ rotr64(b, 3) ^ rotr64(b, 4);
        z = z.rotate_right(1);

        rk[2*i+1] = b;
        b = b ^ c ^ ((z&1) as u64) ^ rotr64(a, 3) ^ rotr64(a, 4);
        z = z.rotate_right(1);
    }

    rk[64] = a; 
    a = a ^ c ^ 1 ^ rotr64(b, 3) ^ rotr64(b, 4);
    rk[65] = b;
    b = b ^ c ^ 0 ^ rotr64(a, 3) ^ rotr64(a, 4);
    rk[66] = a;
    rk[67] = b;
}

fn encrypt(pt:&Vec<u64>, rk:&mut Vec<u64>) -> Vec<u64> {
    let mut data: Vec<u64> = pt.to_vec();

    for i in 0..34 {
        data[0] = data[0] ^ f64(data[1]);
        data[0] = data[0] ^ rk[2*i];
        data[1] = data[1] ^ f64(data[0]);
        data[1] = data[1] ^ rk[2*i+1];
    }

    data
}

fn decrypt(ct:&Vec<u64>, rk:&mut Vec<u64>) -> Vec<u64> {
    let mut data: Vec<u64> = ct.to_vec();

    for i in 0..34 {
        data[1] = data[1] ^ f64(data[0]);
        data[1] = data[1] ^ rk[67-2*i];
        data[0] = data[0] ^ f64(data[1]);
        data[0] = data[0] ^ rk[67-(2*i+1)];
    }

    data
}

fn main() {

    let pt: Vec<u64> = vec![0x6c6c657661727420, 0x6373656420737265];
    let mk: Vec<u64> = vec![0x0706050403020100, 0x0f0e0d0c0b0a0908];
    let mut rk: Vec<u64> = vec![0; 68];
    // ct = 0x65aa832af84e0bbc, 0x49681b1e1e54fe3f

    key_schedule(&mk, &mut rk);

    println!("SIMON128_128");

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
