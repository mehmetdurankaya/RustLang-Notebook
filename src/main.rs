use crate::enums::enums;

mod hesap_makinesi;
mod fizz_buzz;
mod enums;

fn main(){
    let topla = hesap_makinesi::topla(10,5);
    let cikar = hesap_makinesi::cikar(10,5);
    let carp = hesap_makinesi::carp(10,5);
    let bol = hesap_makinesi::bol(10,2);
    println!("toplama sonucu: {} çıkarma sonucu {} çarpma sonucu {} bölme sonucu {}" , topla, cikar, carp, bol);

    let fizz_buzz = fizz_buzz::fizz_buzz();

    enums();
}