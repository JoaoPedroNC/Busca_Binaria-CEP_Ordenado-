use std ::env;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::mem;
use std::path::Path;
//Deve-se sonverter os bytes para string latin1
fn latin1_to_string(s: &[u8] ->
String{
s.iter().map(|&c|c as char).collect()
}

struct Cep{
logradouro:[u8;72],
bairro:[u8;72],
cidade:[u8;72],
uf:[u8;72],
sigla:[u8;2],
cep[u8;8],
lixo:[u8;2],
}
impl Cep{
fn get_cep_value(&self)-> i32{
       latin1_to_string(&self.cep).trim(),
parse::<i32>().unwrap()
        }
  }

  impl fmt::Display for Cep{
  fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
  //Imprimir informações do CEP
       write!(
              f,
              "logradouro: {}\nbairro: {}\ncidade: {}\nuf: {}\nsigla: {}\ncep: {}",
              latin1_to_string(&self.logradouro),
              latin1_to_string(&self.bairro),
              latin1_to_string(&self.cidade),
              latin1_to_string(&self.uf),
              latin1-to-string(&self.sigla),
              self.get_cep_value()
              )
             }
           }
  fn main(){
     let args: Vec<String> = env::args().collect();
     if args.len()!=3{
                       println!(
                                "Nenhum argumento foi passado! \n Exemplo de uso: \n\t$ {}<cep><arquivo_ordenado>",
                                 args.get(0).unwrap()
                                );
                     }
  
  //Verifica se o argumento passado é um número
     let cep: i32=match args.get(1).unwrap().parse(){
                                                     Ok(cep) =>cep,
                                                     Err(_) =>{
                                                               println!(
                                                                        "O CEP deve ser um número inteiro!");
                                                                         return;
                                                               }
                                                     };
        let arquivo=args.get(2).unwrap();
  
        println!(
                 "Buscando o cep{} no arquivo{}", cep, arquivo);
  
        let arquivo= match File::open(Path::new(&arquivo)){
                                                           OK(file)=> file,
                                                           Err(e) => panic!("Erro ao abrir o arquivo: {}", e),
                                                          };
  
       let file_size = match arquivo.metadata(){
             
  
  
  
             
  
  
