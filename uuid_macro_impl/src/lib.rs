#![recursion_limit="128"]

#[macro_use]
extern crate proc_macro_hack;

#[macro_use]
extern crate quote;


proc_macro_expr_impl! {
    pub fn uuid_parts_impl(input: &str) -> String {

        let mut i = 0usize;
        let mut bytes = [0u8; 16];

        let hex = |m: char| {
            match m {
                '0' => 0u8,
                '1' => 1u8,
                '2' => 2u8,
                '3' => 3u8,
                '4' => 4u8,
                '5' => 5u8,
                '6' => 6u8,
                '7' => 7u8,
                '8' => 8u8,
                '9' => 9u8,
                'A' | 'a' => 10u8,
                'B' | 'b' => 11u8,
                'C' | 'c' => 12u8,
                'D' | 'd' => 13u8,
                'E' | 'e' => 14u8,
                'F' | 'f' => 15u8,
                _ => panic!("unexpected char.")
            }
        };
        for c in input.chars() {
            if c == '-' || c == '"' {
                continue;
            }
            let index = i / 2;
            if index >= 16 {
                panic!("out ouf index.")
            }
            bytes[index] = if i % 2 == 0 {hex(c)} else { bytes[index] * 16 + hex(c) };
            i += 1;
        }



        let d0: u8  = bytes[0];
        let d1: u8  = bytes[1];
        let d2: u8  = bytes[2];
        let d3: u8  = bytes[3];

        // data1: u32

        let d4: u8  = bytes[4];
        let d5: u8  =  bytes[5];

        // data2: u16

        let d6: u8  = bytes[6];
        let d7: u8  =  bytes[7];
        // data3: u16

        let data4_0: u8 = bytes[8];
        let data4_1: u8 = bytes[9];
        let data4_2: u8 = bytes[10];
        let data4_3: u8 = bytes[11];
        let data4_4: u8 = bytes[12];
        let data4_5: u8 = bytes[13];
        let data4_6: u8 = bytes[14];
        let data4_7: u8 = bytes[15];
        // data4: u64

        (quote! {
            [
                #d0 as u8,
                #d1 as u8,
                #d2 as u8,
                #d3 as u8,
                #d4 as u8,
                #d5 as u8,
                #d6 as u8,
                #d7 as u8,

               #data4_0 as u8,
               #data4_1 as u8,
               #data4_2 as u8,
               #data4_3 as u8,
               #data4_4 as u8,
               #data4_5 as u8,
               #data4_6 as u8,
               #data4_7 as u8
               ]
        }).to_string()

    }
}
