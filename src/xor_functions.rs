use std::fs;

pub fn xor_block(data_block: &[u8], key_bytes: &[u8]) -> Vec<u8> {
    let mut t: Vec<u8> = Vec::with_capacity(data_block.len());
    for i in 0..data_block.len() {
        let ik = i as usize % key_bytes.len();
        t.push(data_block[i] ^ key_bytes[ik]);
    }
    t
}

pub fn xor_string(s: &String, key: &String) -> Vec<u8> {
    let string_data: Vec<u8> = s.as_bytes().to_vec();
    let num_blocks = (string_data.len() / key.len()) + if string_data.len() % key.len() != 0 { 1 } else { 0 };

    let mut encrypted_data: Vec<u8> = Vec::with_capacity(string_data.len());
    for i in 0..num_blocks {
        let start = i * key.len();
        let block = if start+key.len() <=  string_data.len() {
            &string_data[start..start+key.len()]
        } else {
            &string_data[start..]
        };
        encrypted_data.append(
            & mut xor_block(block, key.as_bytes())
        );
    }
    encrypted_data
}

pub fn xor_encrypt_file(fname: &String, oname: &String, key: &String) -> Result<i32,i32> {
    let file_string: String = match fs::read_to_string(fname) {
        Ok(s) => s,
        Err(e) => {eprintln!("{}",e); return Err(-2)}
    };

    let encrypted_data = xor_string(&file_string, &key);
    
    match fs::write(oname, &encrypted_data[..]) {
        Ok(_) => Ok(0),
        Err(e) => {println!("{}", e); Err(-1) }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn simple_0() {
        let text = String::from("Super Secret String!");
        let key = String::from("key");
        let expected_bytes = [ 0x38,
            0x10,
            0x9,
            0xe,
            0x17,
            0x59,
            0x38,
            0x0,
            0x1a,
            0x19,
            0x0,
            0xd,
            0x4b,
            0x36,
            0xd,
            0x19,
            0xc,
            0x17,
            0xc,
            0x44
        ];
        let output = xor_string(&text, &key);
        let diff = (expected_bytes.len() as i128 - output.len() as i128).abs();
        assert_eq!(diff, 0i128);
        for (exp,out) in expected_bytes.iter().zip(&output){
            assert_eq!(exp, out, "{}, {}", exp, out);
        }
    }
}