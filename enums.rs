enum IpAddrKind{
    V4(u8,u8,u8,u8),
    V6
}

struct IpAddr{
    kind : IpAddrKind,
    address : String
}

enum Message{
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}

impl Message{
    fn some_function(){
        println!("print");
    }
}

enum Option<T>{
    Some(T),
    None
}

fn main() {
    let localhost = IpAddr{
        kind : IpAddrKind::V6,
        address : String::from("192.168.0.1")
    };

    let localhostv2 = IpAddrKind::V4(192,168,0,1);
}
