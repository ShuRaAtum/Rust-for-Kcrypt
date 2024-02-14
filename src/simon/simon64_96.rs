// Simon 64/96

fn rotl32(x: u32, n: u8) -> u32 {
    x.rotate_left(u32::from(n))
}

fn rotr32(x: u32, n: u8) -> u32 {
    x.rotate_right(u32::from(n))
}

fn f32(x:u32) -> u32 {
    (rotl32(x, 1) & rotl32(x, 8)) ^ rotl32(x, 2)
}

fn key_schedule(mk:&Vec<u32>, rk:&mut Vec<u32>) {
    let c: u32 = 0xfffffffc;
    let mut z: u64 = 0x7369f885192c0ef5;

    rk[0] = mk[0];
    rk[1] = mk[1];
    rk[2] = mk[2];

    for i in 3..42 {
        rk[i] = c ^ ((z&1) as u32) ^ rk[i-3] ^ rotr32(rk[i-1], 3) ^ rotr32(rk[i-1], 4);
        z = z.rotate_right(1);
    }
}

fn encrypt(pt:&Vec<u32>, rk:&mut Vec<u32>) -> Vec<u32> {
    let mut data: Vec<u32> = pt.to_vec();

    for i in 0..21 {
        data[0] = data[0] ^ f32(data[1]);
        data[0] = data[0] ^ rk[2*i];
        data[1] = data[1] ^ f32(data[0]);
        data[1] = data[1] ^ rk[2*i+1];
    }

    data
}

fn decrypt(ct:&Vec<u32>, rk:&mut Vec<u32>) -> Vec<u32> {
    let mut data: Vec<u32> = ct.to_vec();

    for i in 0..21 {
        data[1] = data[1] ^ f32(data[0]);
        data[1] = data[1] ^ rk[41-2*i];
        data[0] = data[0] ^ f32(data[1]);
        data[0] = data[0] ^ rk[41-(2*i+1)];
    }

    data
}

fn main() {

    let pt: Vec<u32> = vec![0x6e696c63, 0x6f722067];
    let mk: Vec<u32> = vec![0x03020100, 0x0b0a0908, 0x13121110];
    let mut rk: Vec<u32> = vec![0; 42];

    key_schedule(&mk, &mut rk);

    println!("SIMON64_96");

    println!("");
    println!("ENC...");

    let data: Vec<u32> = encrypt(&pt, &mut rk);

    println!("");
    println!("CT: 0x{:08x} 0x{:08x}", data[0], data[1]);


    println!("");


    println!("");
    println!("DEC...");

    let data: Vec<u32> = decrypt(&data, &mut rk);

    println!("");
    println!("PT: 0x{:08x} 0x{:08x}", data[0], data[1]);


    println!("");
}
