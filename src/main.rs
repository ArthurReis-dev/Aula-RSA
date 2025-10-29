use rand::thread_rng;
use num_bigint::{BigInt, RandBigInt, ToBigInt};
use num_traits::{One, Zero};
use std::str;

// Função para calcular o máximo divisor comum
fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    if b == &BigInt::zero() {
        a.clone()
    } else {
        gcd(b, &(a % b))
    }
}

// Função para calcular o inverso modular
fn modinv(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let mut mn = (m.clone(), a.clone());
    let mut xy = (BigInt::zero(), BigInt::one());

    while mn.1 != BigInt::zero() {
        let q = &mn.0 / &mn.1;
        mn = (mn.1.clone(), &mn.0 - &q * &mn.1);
        xy = (xy.1.clone(), &xy.0 - &q * &xy.1);
    }

    if mn.0 != BigInt::one() {
        None
    } else {
        Some((xy.0 % m + m) % m)
    }
}

// Gera um BigInt aleatório entre min e max
fn random_bigint_range(min: &BigInt, max: &BigInt) -> BigInt {
    let mut rng = thread_rng();
    let range = max - min;
    let rand = rng.gen_bigint_below(&range);
    min + rand
}

// Geração de chaves RSA
fn generate_keys(bits: usize) -> (BigInt, BigInt, BigInt) {
    let mut rng = thread_rng();

    let p = rng.gen_bigint(bits).abs();
    let q = rng.gen_bigint(bits).abs();
    let n = &p * &q;
    let phi = (&p - 1) * (&q - 1);

    let mut e = BigInt::from(65537);
    while gcd(&e, &phi) != BigInt::one() {
        e = random_bigint_range(&BigInt::from(2), &phi);
    }

    let d = modinv(&e, &phi).expect("Não foi possível calcular o inverso modular");
    (n, e, d)
}

// Criptografar mensagem
fn encrypt(message: &str, e: &BigInt, n: &BigInt) -> BigInt {
    let m = BigInt::from_bytes_be(num_bigint::Sign::Plus, message.as_bytes());
    m.modpow(e, n)
}

// Descriptografar mensagem
fn decrypt(ciphertext: &BigInt, d: &BigInt, n: &BigInt) -> String {
    let m = ciphertext.modpow(d, n);
    let bytes = m.to_bytes_be().1;
    String::from_utf8(bytes).expect("Erro ao converter para UTF-8")
}

fn main() {
    let bits = 64; // Tamanho dos primos (pequeno para testes)
    let (n, e, d) = generate_keys(bits);

    let message = "Olá, Arthur!";
    let encrypted = encrypt(message, &e, &n);
    let decrypted = decrypt(&encrypted, &d, &n);

    println!("Mensagem original: {}", message);
    println!("Criptografada: {}", encrypted);
    println!("Descriptografada: {}", decrypted);
}
