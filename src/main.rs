use std::fs::File;
use std::io::ErrorKind;

fn main() {
    hwresult();
}
fn metexpect(){
    let path = "history.json";
    let f = File::open(path).expect("ERROR OPENING FILE");//unwrap ile ayni islemi yapmaktadir fakat farki burada bir mesaj verebiliriz
}

fn metunwrap(){
    let path = "history.json";
    let f = File::open(path).unwrap();//match ile yaptigimiz hata yakalama islemini daha hazili ve kolay yolla yapilmaktadir.

}

fn hwresult() {
    let path = "history.json";
    let f = File::open(path);

    let f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(file) => file,
                Err(e) => panic!("Error creating file :{e:?}"),
            },
            other => panic!("ERROR OCCURED: {:?}", other)
        },
    };
}