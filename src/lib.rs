use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, log};
use std::collections::HashMap;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    followups: HashMap<u16, DonationFollowup> ,
    donation: HashMap<u16, Donation>,
    batches: u16,
}


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Donation { 

    // SETUP CONTRACT STATE
    donor_name: String,
    donor_phone_no: usize,
    donor_email: String,
    distributor_name: String,
    distributor_phone_no: usize,
    distributor_email: String,
    scholar_name: String,
    institution_name: String,
    institution_phone_no: usize,
    institution_email: String,
    date_released: String,
    date_accounted: String,
    is_accounted: bool,

}

#[near_bindgen]
#[derive(Default,BorshDeserialize, BorshSerialize)]
pub struct DonationFollowup{
    //set up contract method
    donation_batch:u16,
    current_possessor:String,
    has_been_accounted:bool,
}

impl Default for Contract {
    fn default() -> Self {
        Contract {
            followups: HashMap::new(),
            donation: HashMap::new(),
            batches:1,
        
        }
    }
}
#[near_bindgen]
impl Contract
  {
    // ADD CONTRACT METHODS HERE
    #[private]
     pub fn new_donation(&mut self, donor_name: String, donor_phone_no: usize, donor_email: String, institution_name: String, institution_phone_no: usize, institution_email: String, scholar_name: String,
        distributor_name: String, distributor_phone_no: usize, distributor_email: String, date: String, )
 {
    let new_donation = Donation 
     {
        donor_name,
        donor_phone_no: donor_phone_no,
        donor_email: donor_email,
        institution_name: institution_name,
        institution_phone_no: institution_phone_no,
        institution_email: institution_email,
        scholar_name: scholar_name,
        distributor_name: distributor_name,
        distributor_phone_no: distributor_phone_no,
        distributor_email:distributor_email,
        date_released: date,
        date_accounted: "".to_string(),
        is_accounted: false,

     };
     
    log!("batch is{}", &self.batches );
    self.donation.insert(self.batches, new_donation);
    self.batches += 1;
  }
    #[private]
    pub fn release(&mut self, batch:u16, possessor: String,){
        let new_followup = DonationFollowup {
            donation_batch: batch,
            current_possessor: possessor,
            has_been_accounted:false,
        };

        self.followups.insert(batch, new_followup);
    }

    pub fn follow_donation(&self, batch: u16, phone: usize, name:String, ) ->String {
        if self.donation[&batch].donor_phone_no != phone{
            log!("only donation donor can followup donation !");
        }
        if self.donation[&batch].scholar_name != name {
            log!("only scholar can followup donation !");
        }
        self.followups[&batch].current_possessor.clone()
    }
}
/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
   // use near_sdk::test_utils::{get_logs, accounts};
    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
   
    // TESTS HERE
    # [test]
    fn test_new_donation(){
      // let mut conext = get_context(accounts(1));
      let mut contract = Contract::default();
      contract.new_donation("joe" .to_string(), 12345, "abc@gmail.com" .to_string(), "wema fundation" .to_string(), 54321, "info@wema.co.ke" .to_string(),
     "ann" .to_string(), "JKUAT" .to_string(), 67890, "info@JKUAT.ac.ke" .to_string(), (10-2-2020) .to_string() );
    assert_eq!(1, contract.donation.len())
    
    }

    #[test]
    fn test_release(){
      let mut contract = Contract::default();
        contract.new_donation("joe" .to_string(), 12345, "abc@gmail.com" .to_string(), "wema fundation" .to_string(), 54321, "info@wema.co.ke" .to_string(),
        "ann" .to_string(), "JKUAT" .to_string(), 67890, "info@JKUAT.ac.ke" .to_string(), (10-2-2020) .to_string() );
        contract.release(1, "ann".to_string());

        assert_eq!(1, contract.followups.len())

    }
}