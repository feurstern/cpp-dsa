pub fn nth(n: u32) -> u32 {
    let mut primes : Vec<u32> = Vec::new();
    let mut c = 2;

    while primes.len() <= n as usize{
        let is_prime = primes.iter().take_while(|&&p| p * p <= c).all(|&p| c %p!=0);

        if is_prime{
            primes.push(c)
        }

        c+=1;
    }

    primes[n as usize]
}
