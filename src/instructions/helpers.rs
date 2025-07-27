#[repr(C,packed)]
pub struct LoanData {
    pub protocol_token_account: [u8; 32],
    pub balance: u64,
}
 
pub fn get_token_amount(data: &[u8]) -> u64 {
    unsafe { *(data.as_ptr().add(64) as *const u64) }
}


// This file is straightforward. It contains a LoanData struct,
// which we'll use to temporarily store loan data in an account before the loan is repaid. 
// It also provides a get_token_amount() helper function to read the token amount from an account.

