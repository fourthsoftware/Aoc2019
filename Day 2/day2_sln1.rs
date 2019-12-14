use std::fs;

fn main() {

    let mut ins_vec: Vec<i64> = fs::read_to_string("day2_sln1_input.txt").unwrap().split(",").map(|x| x.parse().unwrap()).collect();

    let mut ins_ptr_idx = 0;

    while ins_ptr_idx < ins_vec.len() {
        if ins_vec[ins_ptr_idx] == 1 {
            let add_1 = ins_vec[ins_ptr_idx + 1];
            let add_2 = ins_vec[ins_ptr_idx + 2];
            let pos_to_write = ins_vec[ins_ptr_idx + 3];
            
            ins_vec[pos_to_write as usize] = add_1 + add_2;
            ins_ptr_idx += 4; // move on to next instruction
        } else if ins_vec[ins_ptr_idx] == 2 {
            let mul_1 = ins_vec[ins_ptr_idx + 1];
            let mul_2 = ins_vec[ins_ptr_idx + 2];
            let pos_to_write = ins_vec[ins_ptr_idx + 3];
            
            ins_vec[pos_to_write as usize] = mul_1 * mul_2;
            ins_ptr_idx += 4; // move on to next instruction
        } else if ins_vec[ins_ptr_idx] == 99 {
            println!("Finished.");
            break;
        } else {
            println!("Can't process opcode at position: {}", ins_ptr_idx);
            break;
        }

    }

    println!("{:?}", ins_vec);
}