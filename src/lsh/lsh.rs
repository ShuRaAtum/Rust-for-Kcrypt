use crate::lsh::lsh256;
use crate::lsh::lsh512;

fn LSH_256_TEST(hashbitlen: u64){
    let databitlen: [u32; 16] = [ 0, 1, 2, 7, 8, 15, 16, 1023, 1024, 1025, 1024 * 2 - 1, 1024 * 2, 1024 * 2 + 1, 1024 * 3 - 1, 1024 * 3, 1024 * 3 + 1];
    // let databitlen: [u32; 4] = [ 0, 1, 8, 1024 ];

    for i in databitlen{
        // Create Test Vector
        let mut data: Vec<u8> = Vec::new();
        
        let datalen: u32 = (i / 8) + if i & 0x7 != 0 { 1 } else { 0 };
        println!("LSH 256-{} TEST", hashbitlen);
        println!("Input Message Length in Bits : {}", i);
        println!("Input Message :");
        for l in 0..datalen{
           data.push(l as u8);
        //    print!("{:#04x} ", data[l as usize]);
        }
        println!();

        // Start Hash Function
        let mut hash = vec![0; (hashbitlen/8) as usize];
        lsh256::Hash256(hashbitlen, data, i as u64, &mut hash);

        println!("Hash Value: ");
        for i in hash{
            print!("{:#02x} ", i);
        } println!("\n----------------------------------------------------------------------");  println!();
    }
}


fn LSH_512_TEST(hashbitlen: u64){
    let databitlen: [u32; 16] = [ 0, 1, 2, 7, 8, 15, 16, 2047, 2048, 2049, 2048 * 2 - 1, 2048 * 2, 2048 * 2 + 1, 2048 * 3 - 1, 2048 * 3, 2048 * 3 + 1 ];
    // let databitlen: [u32; 4] = [ 0, 1, 8, 1024 ];

    for i in databitlen{
        // Create Test Vector
        let mut data: Vec<u8> = Vec::new();
        
        let datalen: u32 = (i / 8) + if i & 0x7 != 0 { 1 } else { 0 };
        println!("LSH 512-{} TEST", hashbitlen);
        println!("Input Message Length in Bits : {}", i);
        println!("Input Message :");
        for l in 0..datalen{
           data.push(l as u8);
        //    print!("{:#04x} ", data[l as usize]);
        }
        println!();

        // Start Hash Function
        let mut hash = vec![0; (hashbitlen/8) as usize];
        lsh512::Hash512(hashbitlen, data, i as u64, &mut hash);

        println!("Hash Value: ");
        for i in hash{
            print!("{:#02x} ", i);
        } println!("\n----------------------------------------------------------------------");  println!();
    }
}

pub fn LSH_TEST(){
    // LSH_256_TEST(Hash_length)
    // LSH_256_TEST(224);
    LSH_256_TEST(256);

    // LSH_512_TEST(Hash_length)
    // LSH_512_TEST(224);
    // LSH_512_TEST(256);
    // LSH_512_TEST(384);
    LSH_512_TEST(512);
}