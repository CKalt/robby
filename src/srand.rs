use rand::prelude::*;
    
#[cfg(opt_rng="ChaCha8Rng")]
use rand_chacha::ChaCha8Rng;
#[cfg(opt_rng="ChaCha20Rng")]
use rand_chacha::ChaCha20Rng;

#[cfg(opt_rng="ChaCha8Rng")]
pub type SRng = ChaCha8Rng;
#[cfg(opt_rng="ChaCha20Rng")]
pub type SRng = ChaCha20Rng;

pub struct RngFactory;
impl RngFactory {
    #[cfg(opt_rng="ChaCha8Rng")]
    pub fn new(seed: Option<u64>) -> rand_chacha::ChaCha8Rng {
        match seed {
            Some(num) => ChaCha8Rng::seed_from_u64(num),
            None => ChaCha8Rng::from_entropy(),
        }
    }
    #[cfg(opt_rng="ChaCha20Rng")]
    pub fn new(seed: Option<u64>) -> rand_chacha::ChaCha20Rng {
        match seed {
            Some(num) => ChaCha20Rng::seed_from_u64(num),
            None => ChaCha20Rng::from_entropy(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::srand::RngFactory;
    use rand::prelude::*;

    #[test]
    #[cfg(opt_rng="ChaCha8Rng")]
    fn rng_factory() {
        let seed = Some(737221);
        let mut srng = RngFactory::new(seed);
        let n_u8: u8 = srng.gen();
        let n_u16: u16 = srng.gen();
        let n_u32 =  srng.gen::<u32>();
        let n_i32 = srng.gen::<i32>();
        let n_f64 = srng.gen::<f64>();
        let n_bool1 = srng.gen::<bool>();
        let n_bool2 = srng.gen::<bool>();
        let n_bool3 = srng.gen::<bool>();
        let n_bool4 = srng.gen::<bool>();

        assert_eq!(n_u8, 23);
        assert_eq!(n_u16, 11108);
        assert_eq!(n_u32, 2960706524);
        assert_eq!(n_i32, -484573038);
        assert_eq!(n_f64, 0.3272060383192589);
        assert_eq!(n_bool1, false);
        assert_eq!(n_bool2, false);
        assert_eq!(n_bool3, false);
        assert_eq!(n_bool4, true);

        println!("ChaCha8Rng Seeded Random Test Passed u8: {}", n_u8);
        println!("ChaCha8Rng Seeded Random Test Passed u16: {}", n_u16);
        println!("ChaCha8Rng Seeded Random Test Passed u32: {}", n_u32); 
        println!("ChaCha8Rng Seeded Random Test Passed i32: {}", n_i32);
        println!("ChaCha8Rng Seeded Random Test Passed float: {}", n_f64);
        println!("ChaCha8Rng Seeded Random Test Passed float: {}", n_bool1);
        println!("ChaCha8Rng Seeded Random Test Passed float: {}", n_bool2);
        println!("ChaCha8Rng Seeded Random Test Passed float: {}", n_bool3);
        println!("ChaCha8Rng Seeded Random Test Passed float: {}\n\n", n_bool4);

        /*
            Expected output:
                ChaCha8Rng Seeded Random Test Passed u8: 23
                ChaCha8Rng Seeded Random Test Passed u16: 11108
                ChaCha8Rng Seeded Random Test Passed u32: 2960706524
                ChaCha8Rng Seeded Random Test Passed i32: -484573038
                ChaCha8Rng Seeded Random Test Passed float: 0.3272060383192589
        */
    }

    #[test]
    #[cfg(opt_rng="ChaCha20Rng")]
    fn rng_factory() {
        let seed = Some(38337221);
        let mut srng = RngFactory::new(seed);
        let n_u8: u8 = srng.gen();
        let n_u16: u16 = srng.gen();
        let n_u32 =  srng.gen::<u32>();
        let n_i32 = srng.gen::<i32>();
        let n_f64 = srng.gen::<f64>();
        let n_bool1 = srng.gen::<bool>();
        let n_bool2 = srng.gen::<bool>();
        let n_bool3 = srng.gen::<bool>();
        let n_bool4 = srng.gen::<bool>();

        assert_eq!(n_u8, 244);
        assert_eq!(n_u16, 1503);
        assert_eq!(n_u32, 2528013810);
        assert_eq!(n_i32, 285179100);
        assert_eq!(n_f64, 0.8508277408098059);
        assert_eq!(n_bool1, true);
        assert_eq!(n_bool2, false);
        assert_eq!(n_bool3, false);
        assert_eq!(n_bool4, true);

        println!("ChaCha20Rng Seeded Random Test Passed u8: {}", n_u8);
        println!("ChaCha20Rng Seeded Random Test Passed u16: {}", n_u16);
        println!("ChaCha20Rng Seeded Random Test Passed u32: {}", n_u32); 
        println!("ChaCha20Rng Seeded Random Test Passed i32: {}", n_i32);
        println!("ChaCha20Rng Seeded Random Test Passed float: {}", n_f64);
        println!("ChaCha8Rng Seeded Random Test Passed float: {}", n_bool1);
        println!("ChaCha8Rng Seeded Random Test Passed float: {}", n_bool2);
        println!("ChaCha8Rng Seeded Random Test Passed float: {}", n_bool3);
        println!("ChaCha8Rng Seeded Random Test Passed float: {}\n\n", n_bool4);

        /*
        Expected output:
            ChaCha20Rng Seeded Random Test Passed u8: 56
            ChaCha20Rng Seeded Random Test Passed u16: 26232
            ChaCha20Rng Seeded Random Test Passed u32: 1578541150
            ChaCha20Rng Seeded Random Test Passed i32: -1448431532
            ChaCha20Rng Seeded Random Test Passed float: 0.706382052111321
*/
    }
}
