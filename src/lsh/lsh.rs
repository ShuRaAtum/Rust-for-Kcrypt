

fn LSH_256_256_TEST(){
    println!("LSH 256-256 TEST");

    let databitlen256: [u32; 16] = [ 0, 1, 2, 7, 8, 15, 16, 1023, 1024, 1025, 1024 * 2 - 1, 1024 * 2, 1024 * 2 + 1, 1024 * 3 - 1, 1024 * 3, 1024 * 3 + 1];
    // let databitlen256: [u32; 4] = [ 0, 1, 8, 1024 ];
    for i in databitlen256{
        // Create Test Vector
        let mut data: Vec<u8> = Vec::new();
        
        let datalen: u32 = (i / 8) + if i & 0x7 != 0 { 1 } else { 0 };
        println!("LSH 256-256 TEST");
        println!("Input Message Length in Bits : {}", i);
        println!("Input Message :");
        for l in 0..datalen{
           data.push(l as u8);
        //    print!("{:#04x} ", data[l as usize]);
        }
        println!();

        // Start Hash Function
        // let 
        // let size: usize = hasibitlen / 8;
        let mut hash: [u8; 32] = [0; 32];
        Hash256(224, data, i as u64, &mut hash);

        println!("Hash Value: ");
        for i in hash{
            print!("{:#02x} ", i);
        } println!("\n----------------------------------------------------------------------");  println!();
    }
    

}


pub fn LSH_TEST(){
    LSH_256_256_TEST();
}