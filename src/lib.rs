use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  println!("The Rust function say() received {}", s);
  let r = String::from("hello ");
  return r + s;
}

#[wasm_bindgen]
pub fn jiami(s: &str) -> String {
  println!("jiami收到参数s:{}",s);
  let text = s;
  let mut bytes = Vec::new();

  let mut key: u8 = 0;
  let cipher_text = loop {
      if key == 255 {
          break String::from("加密失败，请适当更换文本内容")
      };
      key = key + 1;
      for c in text.bytes() {
          let (b, _) = (c as u8).overflowing_add(key);
          bytes.push(b);
      };
      
      let end_text = String::from_utf8(bytes.clone());
      let end_text = match end_text{
          Ok(b) => {
              break b
          },
          Err(e) =>{
              e.to_string()
          }, 
      };
      println!("{}key{}", end_text, key);
  };
  
  format!("密文是：\"{}\"，密码是：\"{}\"。", cipher_text, key)
}

#[wasm_bindgen]
pub fn jiemi(s: &str,key: &str) -> String {
  println!("jiemi收到参数s:{},key:{}",s,key);
  let text = s;
  let mut bytes = Vec::new();
  let key = key.parse::<u8>().unwrap();

  for c in text.bytes() {
      let (b, _) = (c as u8).overflowing_sub(key);
      bytes.push(b);
  };
  
  let cipher_text = String::from_utf8(bytes.clone());
  let cipher_text = match cipher_text{
      Ok(b) => b,
      Err(e) =>{
          e.to_string()
      }, 
  };
  format!("解密结果：\"{}\"。", cipher_text)
}



