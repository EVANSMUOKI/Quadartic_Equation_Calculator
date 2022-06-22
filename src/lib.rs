use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, log};
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Coefficient {
    //USER STATES
    val1: f32,
    val2: f32,
    val3: f32,
}
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Quadratic {
    // setting up the states
      coefficients : Vec<Coefficient >,
      val1: f32,
      val2: f32,
      val3: f32,
    
}


#[near_bindgen]
impl Quadratic {

    //CONTRACT METHODS

    //the function to obtain coefficients input from user
      
    pub fn obtain_coefficients(&mut self, a: f32,b: f32,c: f32,){
    
        //accept user input and store the values for a, b, and c.
          let _new_coefficients =Coefficient {
           val1: a, 
            val2: b, 
            val3:c,
           
        };


      }

      //function to find roots of the equation
     pub fn find_roots(&mut self) {
         //all find_roots methods here


        let discriminant: f32 = (self.val2*self.val2)- (4f32 * self.val1 * self.val3);    
        
        if discriminant > 0f32 {
            //checking if roots are real and distinct
      
            let x_0 = (-self.val2 + discriminant.sqrt()) /(self.val1*2f32);
            let x_1 = (-self.val2 - discriminant.sqrt()) /(self.val1 *2f32);
            // (x_0, x_1)
            log!("x1= {} x2= {}", x_0, x_1);
            } 
            else if discriminant == 0.0 {
                //checking if the roots are real and reapeted

                let x_0= (-self.val2 )/(self.val1*2f32);
                log!("x= {}", x_0);
                // (x_0, x_1)

            }
            else if discriminant < 0.0 {
                //checking if roots are not real(complex)


                let real_part=(-self.val2 )/(self.val1*2f32);
                let img_part= discriminant.sqrt()/(self.val1*2f32);
                log!("x1= ({}+{}i) x2= ({}{}i)", real_part,img_part,real_part,-img_part);

            }
     }
                    
    
 


pub fn reset(&mut self) {
    //this function resets the values of coefficients to zero
    self.val1=0f32;
    self.val2 =0f32;
    self.val3 =0f32;

    log!("Reset coefficients to zero"); 
}
}   


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

//use the attribute below for unit tests

#[cfg(test)]
mod tests{
    use super::*;
    use near_sdk::{testing_env, VMContext};

fn get_context(_input: Vec<f32>, is_view: bool) -> VMContext {
   
 VMContext {
    current_account_id: "evansmuoki.testnet".to_string(),
    signer_account_id: "robert.testnet".to_string(),
    signer_account_pk: vec![0, 1, 2],
    predecessor_account_id: "jane.testnet".to_string(),
    input: vec![],
    block_index: 0,
    block_timestamp: 0,
    account_balance: 0,
    account_locked_balance: 0,
    storage_usage: 0,
    attached_deposit: 0,
    prepaid_gas: 10u64.pow(18),
    random_seed: vec![0, 1, 2],
    is_view,
    output_data_receivers: vec![],
    epoch_height: 19, 
    }
}
//TESTS HERE
    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
 #[test]
 //testing the obtained coefficients

 fn obtain_coefficients_test (){
    let context = get_context(vec![], false);
    testing_env!(context);
    
    let mut contract = Quadratic { coefficients: Vec::new(),val1: 0.0, val2: 0.0, val3: 0.0};
        contract.obtain_coefficients (1.0,-5.0,6.0);
    
    println!("Coefficients after inserting should be : {} {} {}", contract.val1, contract.val2, contract.val3);
    assert_eq!(0, contract.coefficients.len());

 }
   #[test]
   //testing the roots output
    fn find_roots_test(){

        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Quadratic { coefficients: Vec::new(),val1: 0.0,val2: 0.0,val3:0.0};
        contract.find_roots();
        //assert_ne!(0.0,0.0, "{} {}", contract.val1, contract.val2,);
        assert_eq!(0.0, contract.val1);
        println!("The roots are: {:?}", contract.find_roots());

    }

    #[test]
    //testing the reset function for coefficientd
    fn reset_coefficients_test (){
            let context = get_context(vec![], false);
            testing_env!(context);
            let mut contract = Quadratic{ coefficients: Vec::new(),val1: 0.0,val2: 0.0,val3:0.0};
            contract.find_roots();
            //assert_ne!(0,  contract.coefficients.len());
            contract.reset();
            println!("Value after coeffients reset: {}", contract.coefficients.len());
            assert_eq!(0, contract.coefficients.len());
           
    }
}

 