use std::ops::Index;


const NS256: u32 =26;

static SC256: [[u32; 8]; 26] = [
	[ 0x917caf90, 0x6c1b10a2, 0x6f352943, 0xcf778243, 0x2ceb7472, 0x29e96ff2, 0x8a9ba428, 0x2eeb2642 ],
	[ 0x0e2c4021, 0x872bb30e, 0xa45e6cb2, 0x46f9c612, 0x185fe69e, 0x1359621b, 0x263fccb2, 0x1a116870 ],
	[ 0x3a6c612f, 0xb2dec195, 0x02cb1f56, 0x40bfd858, 0x784684b6, 0x6cbb7d2e, 0x660c7ed8, 0x2b79d88a ],
	[ 0xa6cd9069, 0x91a05747, 0xcdea7558, 0x00983098, 0xbecb3b2e, 0x2838ab9a, 0x728b573e, 0xa55262b5 ],
	[ 0x745dfa0f, 0x31f79ed8, 0xb85fce25, 0x98c8c898, 0x8a0669ec, 0x60e445c2, 0xfde295b0, 0xf7b5185a ],
	[ 0xd2580983, 0x29967709, 0x182df3dd, 0x61916130, 0x90705676, 0x452a0822, 0xe07846ad, 0xaccd7351 ],
	[ 0x2a618d55, 0xc00d8032, 0x4621d0f5, 0xf2f29191, 0x00c6cd06, 0x6f322a67, 0x58bef48d, 0x7a40c4fd ],
	[ 0x8beee27f, 0xcd8db2f2, 0x67f2c63b, 0xe5842383, 0xc793d306, 0xa15c91d6, 0x17b381e5, 0xbb05c277 ],
	[ 0x7ad1620a, 0x5b40a5bf, 0x5ab901a2, 0x69a7a768, 0x5b66d9cd, 0xfdee6877, 0xcb3566fc, 0xc0c83a32 ],
	[ 0x4c336c84, 0x9be6651a, 0x13baa3fc, 0x114f0fd1, 0xc240a728, 0xec56e074, 0x009c63c7, 0x89026cf2 ],
	[ 0x7f9ff0d0, 0x824b7fb5, 0xce5ea00f, 0x605ee0e2, 0x02e7cfea, 0x43375560, 0x9d002ac7, 0x8b6f5f7b ],
	[ 0x1f90c14f, 0xcdcb3537, 0x2cfeafdd, 0xbf3fc342, 0xeab7b9ec, 0x7a8cb5a3, 0x9d2af264, 0xfacedb06 ],
	[ 0xb052106e, 0x99006d04, 0x2bae8d09, 0xff030601, 0xa271a6d6, 0x0742591d, 0xc81d5701, 0xc9a9e200 ],
	[ 0x02627f1e, 0x996d719d, 0xda3b9634, 0x02090800, 0x14187d78, 0x499b7624, 0xe57458c9, 0x738be2c9 ],
	[ 0x64e19d20, 0x06df0f36, 0x15d1cb0e, 0x0b110802, 0x2c95f58c, 0xe5119a6d, 0x59cd22ae, 0xff6eac3c ],
	[ 0x467ebd84, 0xe5ee453c, 0xe79cd923, 0x1c190a0d, 0xc28b81b8, 0xf6ac0852, 0x26efd107, 0x6e1ae93b ],
	[ 0xc53c41ca, 0xd4338221, 0x8475fd0a, 0x35231729, 0x4e0d3a7a, 0xa2b45b48, 0x16c0d82d, 0x890424a9 ],
	[ 0x017e0c8f, 0x07b5a3f5, 0xfa73078e, 0x583a405e, 0x5b47b4c8, 0x570fa3ea, 0xd7990543, 0x8d28ce32 ],
	[ 0x7f8a9b90, 0xbd5998fc, 0x6d7a9688, 0x927a9eb6, 0xa2fc7d23, 0x66b38e41, 0x709e491a, 0xb5f700bf ],
	[ 0x0a262c0f, 0x16f295b9, 0xe8111ef5, 0x0d195548, 0x9f79a0c5, 0x1a41cfa7, 0x0ee7638a, 0xacf7c074 ],
	[ 0x30523b19, 0x09884ecf, 0xf93014dd, 0x266e9d55, 0x191a6664, 0x5c1176c1, 0xf64aed98, 0xa4b83520 ],
	[ 0x828d5449, 0x91d71dd8, 0x2944f2d6, 0x950bf27b, 0x3380ca7d, 0x6d88381d, 0x4138868e, 0x5ced55c4 ],
	[ 0x0fe19dcb, 0x68f4f669, 0x6e37c8ff, 0xa0fe6e10, 0xb44b47b0, 0xf5c0558a, 0x79bf14cf, 0x4a431a20 ],
	[ 0xf17f68da, 0x5deb5fd1, 0xa600c86d, 0x9f6c7eb0, 0xff92f864, 0xb615e07f, 0x38d3e448, 0x8d5d3a6a ],
	[ 0x70e843cb, 0x494b312e, 0xa6c93613, 0x0beb2f4f, 0x928b5d63, 0xcbf66035, 0x0cb82c80, 0xea97a4f7 ],
	[ 0x592c0f3b, 0x947c5f77, 0x6fff49b9, 0xf71a7e5a, 0x1de8c0f5, 0xc2569600, 0xc4e4ac8c, 0x823c9ce1 ]
];

static IV256: [u32; 16] = [
    0x46a10f1f, 0xfddce486, 0xb41443a8, 0x198e6b9d, 0x3304388d, 0xb0f5a3c7, 0xb36061c4, 0x7adbd553,
	0x105d5378, 0x2f74de54, 0x5c2f2d95, 0xf2553fbe, 0x8051357a, 0x138668c8, 0x47aa4484, 0xe01afb41
];

static GAMMA256: [u8; 8] = [0, 8, 16, 24, 24, 16, 8, 0 ];

struct hashState256{
    hashbitlen: u64,
    cv256: [u32; 16],
    Last256: [u8; 128]
}

fn print_array<T: std::fmt::UpperHex>(state: &[T], str: &str, ui: &str) {
    print!("\n{str} : ");
    if ui == "u8"{
        let mut a = 0;
        for i in state {
            if a%4==0 { println!(); }
            print!("{:#04X} ", i);
            a += 1;
        }
    } else {
        let mut a = 0;
        for i in state {
            if a%4==0 { println!(); }
            print!("{:#010X} ", i);
            a += 1;
        }
    }
    println!();
}

pub fn LSH_TEST(){
    println!("LSH 256-256 TEST");

    let databitlen256: [u32; 16] = [ 0, 1, 2, 7, 8, 15, 16, 1023, 1024, 1025, 1024 * 2 - 1, 1024 * 2, 1024 * 2 + 1, 1024 * 3 - 1, 1024 * 3, 1024 * 3 + 1];
    // let databitlen256: [u32; 5] = [ 0, 1024, 1025, 1024 * 2 - 1, 1024 * 3 + 1 ];
    for i in databitlen256{
        // Create Test Vector
        let mut data: Vec<u8> = Vec::new();
        
        let datalen: u32 = (i / 8) + if i & 0x7 != 0 { 1 } else { 0 };

        println!("Input Message Length in Bits : {}", i);
        println!("Input Message :");
        for l in 0..datalen{
           data.push(l as u8);
           print!("{:#04x} ", data[l as usize]);
        }
        println!();

        // Start Hash Function
        let mut hash: [u8; 32] = [0; 32];
        Hash256(256, data, i as u64, &mut hash);

        println!("Hash Value: ");
        for i in hash{
            print!("{:#02x} ", i);
        } println!("\n----------------------------------------------------------------------");  println!();
    }
    

}

fn Hash256(hashbitlen: u64, data: Vec<u8>, databitlen: u64, hashval: &mut [u8]){
    let mut state = hashState256 {
        hashbitlen: 0,
        cv256: [0; 16],
        Last256: [0; 128]
    };

    // @TODO Error check

    //ret = Init256(&state, hashbitlen);
    let ret = Init256(&mut state, 256);

    //Update256(&state, data, databitlen);	
    Update256(&mut state, data, databitlen);

	//Final256(&state, hashval);
    Final256(&mut state, hashval);
}

fn Init256(state: &mut hashState256, hashbitlen: u64) -> u32{
    if hashbitlen>256{
        return 0
    }
    else if hashbitlen<256{
        state.cv256[0] = 32;
        state.cv256[1] = hashbitlen as u32;
        
        let mut T: [u32; 16] = [0; 16];

        for j in 0..NS256/2 {
            let k = 2*j;
            for l in 0..8 {
                let mut vl = state.cv256[l];
                let mut vr = state.cv256[l+8];
                vl = (vl.wrapping_add(vr)).rotate_left(29) ^ SC256[k as usize][l];
                vr = (vr.wrapping_add(vl)).rotate_left(1);
                T[l] = vr.wrapping_add(vl);
                T[l+8] = vr.rotate_left(GAMMA256[l] as u32);
            }
            
            state.cv256[0] = T[ 6]; state.cv256[ 8] = T[ 2];
            state.cv256[1] = T[ 4]; state.cv256[ 9] = T[ 0];
            state.cv256[2] = T[ 5]; state.cv256[10] = T[ 1];
            state.cv256[3] = T[ 7]; state.cv256[11] = T[ 3];
            state.cv256[4] = T[12]; state.cv256[12] = T[ 8];
            state.cv256[5] = T[15]; state.cv256[13] = T[11];
            state.cv256[6] = T[14]; state.cv256[14] = T[10];
            state.cv256[7] = T[13]; state.cv256[15] = T[ 9];


            let k = 2*j+1;
            for l in 0..8 {
                let mut vl = state.cv256[l];
                let mut vr = state.cv256[l+8];
                vl = (vl.wrapping_add(vr)).rotate_left(5) ^ SC256[k as usize][l];
                vr = (vr.wrapping_add(vl)).rotate_left(17);
                T[l] = vr.wrapping_add(vl);
                T[l+8] = vr.rotate_left(GAMMA256[l] as u32);
            }
            
            state.cv256[0] = T[ 6]; state.cv256[ 8] = T[ 2];
            state.cv256[1] = T[ 4]; state.cv256[ 9] = T[ 0];
            state.cv256[2] = T[ 5]; state.cv256[10] = T[ 1];
            state.cv256[3] = T[ 7]; state.cv256[11] = T[ 3];
            state.cv256[4] = T[12]; state.cv256[12] = T[ 8];
            state.cv256[5] = T[15]; state.cv256[13] = T[11];
            state.cv256[6] = T[14]; state.cv256[14] = T[10];
            state.cv256[7] = T[13]; state.cv256[15] = T[ 9];
        } 
    }
    else {
        state.cv256.clone_from_slice(&IV256);
    }

    state.hashbitlen = hashbitlen;

    1
}

fn u8tou32(output: &mut [u32], input: &[u8]) {
    for i in 0..32 {
        output[i] = (input[i*4] as u32) | 
            ((input[i*4 + 1] as u32) << 8) | 
            ((input[i*4 + 2] as u32) << 16) | 
            ((input[i*4 + 3] as u32) << 24);
    }
}

fn compress256(state: &mut hashState256, datablock: &[u8]){
    let mut m: [u32; (16*(NS256+1))as usize] = [0; (16*(NS256+1)) as usize];

    u8tou32(&mut m, &datablock);

    for j in 2..(NS256+1){
        let k = 16*j;
        m[(k +  0) as usize] = m[(k - 16) as usize].wrapping_add(m[(k - 29) as usize]);
		m[(k +  1) as usize] = m[(k - 15) as usize].wrapping_add(m[(k - 30) as usize]);
		m[(k +  2) as usize] = m[(k - 14) as usize].wrapping_add(m[(k - 32) as usize]);
		m[(k +  3) as usize] = m[(k - 13) as usize].wrapping_add(m[(k - 31) as usize]);
		m[(k +  4) as usize] = m[(k - 12) as usize].wrapping_add(m[(k - 25) as usize]);
		m[(k +  5) as usize] = m[(k - 11) as usize].wrapping_add(m[(k - 28) as usize]);
		m[(k +  6) as usize] = m[(k - 10) as usize].wrapping_add(m[(k - 27) as usize]);
		m[(k +  7) as usize] = m[(k -  9) as usize].wrapping_add(m[(k - 26) as usize]);
		m[(k +  8) as usize] = m[(k -  8) as usize].wrapping_add(m[(k - 21) as usize]);
		m[(k +  9) as usize] = m[(k -  7) as usize].wrapping_add(m[(k - 22) as usize]);
		m[(k + 10) as usize] = m[(k -  6) as usize].wrapping_add(m[(k - 24) as usize]);
		m[(k + 11) as usize] = m[(k -  5) as usize].wrapping_add(m[(k - 23) as usize]);
		m[(k + 12) as usize] = m[(k -  4) as usize].wrapping_add(m[(k - 17) as usize]);
		m[(k + 13) as usize] = m[(k -  3) as usize].wrapping_add(m[(k - 20) as usize]);
		m[(k + 14) as usize] = m[(k -  2) as usize].wrapping_add(m[(k - 19) as usize]);
		m[(k + 15) as usize] = m[(k -  1) as usize].wrapping_add(m[(k - 18) as usize]);
    }

    
    for j in 0..NS256/2 {
        {
            let mut T: [u32; 16] = [0; 16];
            let k = 2*j;
        
            for l in 0..8{
                let mut vl = state.cv256[l] ^ m[(16*k)as usize + l];
                let mut vr = state.cv256[l+8] ^ m[(16*k)as usize + (l+8)];
                vl = vl.wrapping_add(vr).rotate_left(29) ^ SC256[k as usize][l];
                vr = vr.wrapping_add(vl).rotate_left(1);
                T[l] = vr.wrapping_add(vl);
                T[l+8] = vr.rotate_left(GAMMA256[l] as u32);
            }

            state.cv256[0] = T[ 6];state.cv256[ 8] = T[ 2];
            state.cv256[1] = T[ 4];state.cv256[ 9] = T[ 0];
            state.cv256[2] = T[ 5];state.cv256[10] = T[ 1];
            state.cv256[3] = T[ 7];state.cv256[11] = T[ 3];
            state.cv256[4] = T[12];state.cv256[12] = T[ 8];
            state.cv256[5] = T[15];state.cv256[13] = T[11];
            state.cv256[6] = T[14];state.cv256[14] = T[10];
            state.cv256[7] = T[13];state.cv256[15] = T[ 9];
        }
        {
            let mut T: [u32; 16] = [0; 16];
            let k = 2*j+1;

            for l in 0..8{
                let mut vl = state.cv256[l] ^ m[(16*k)as usize + l];
                let mut vr = state.cv256[l+8] ^ m[(16*k)as usize + (l+8)];
                vl = vl.wrapping_add(vr).rotate_left(5) ^ SC256[k as usize][l];
                vr = vr.wrapping_add(vl).rotate_left(17);
                T[l] = vr.wrapping_add(vl);
                T[l+8] = vr.rotate_left(GAMMA256[l] as u32);
            }

            state.cv256[0] = T[ 6];state.cv256[ 8] = T[ 2];
            state.cv256[1] = T[ 4];state.cv256[ 9] = T[ 0];
            state.cv256[2] = T[ 5];state.cv256[10] = T[ 1];
            state.cv256[3] = T[ 7];state.cv256[11] = T[ 3];
            state.cv256[4] = T[12];state.cv256[12] = T[ 8];
            state.cv256[5] = T[15];state.cv256[13] = T[11];
            state.cv256[6] = T[14];state.cv256[14] = T[10];
            state.cv256[7] = T[13];state.cv256[15] = T[ 9];
        }
    }

    for l in 0..16{
        state.cv256[l] ^= m[(16*NS256)as usize + l];
    }
}

fn memset(output: &mut [u8]){
    output.fill(0);
}

fn Update256(state: &mut hashState256, data: Vec<u8>, databitlen: u64) {
    let numBlocks: u64 = (databitlen>>10) + 1;
    let mut index: usize = 0;
    for i in 0..numBlocks-1{
        compress256(state, &data[(i*128)as usize..(i*128+128)as usize]);
        index+=128;
    }

    if (databitlen & 0x3ff) != 0 {
        let temp: u64 = (numBlocks-1)<<7;
        let pos1: u32 = ((databitlen>>3).wrapping_sub(temp)) as u32;
        let pos2: u32 = (databitlen & 0x7) as u32;

        let temp_data = &data[index..];

        if pos2 != 0 {
            if temp_data.len() != 0
            { 
                state.Last256[0..pos1 as usize].clone_from_slice(&temp_data[0..pos1 as usize]);
                state.Last256[pos1 as usize] = (temp_data[pos1 as usize]&(0xff << (8-pos2))) ^ (1<<(7-pos2)); 
            }
            if pos1 != 127 {
                memset(&mut state.Last256[(pos1+1)as usize ..]);
            }
        } else {
            if temp_data.len() != 0{
                state.Last256[0..pos1 as usize].clone_from_slice(&temp_data[0..pos1 as usize]);
            }
            state.Last256[pos1 as usize] = 0x80;
            if pos1 != 127 {
                memset(&mut state.Last256[(pos1+1)as usize ..]);
            }
        }
    } else {
        state.Last256[0] = 0x80;
        memset(&mut state.Last256[1..]);
    }
}


fn Final256(state: &mut hashState256, hashval: &mut [u8]){
    let mut H: [u32; 8] = [0; 8];
    compress256(state, &state.Last256.clone());

    for l in 0..8 {
        H[l] = state.cv256[l] ^ state.cv256[l+8];
    }

    for l in 0..(state.hashbitlen)>>3 {
        hashval[l as usize] = (H[(l>>2) as usize] >> ((l<<3)&0x1f)) as u8;
    }
}
