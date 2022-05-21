use rkyv::{Archive, Deserialize, Serialize};

fn main() {
    let req = Test {
        a: u64::MAX,
        e: TestE::Foo { b: [0xf, 0xe, 0xd, 0xc], a: [1, 2, 3, 4] },
    };
    let b = rkyv::to_bytes::<_, 1024>(&req).unwrap();
    let h = hex::encode(b.as_slice());
    println!("HEX BYTES = {}", h);

    let b = hex::decode("ffffffffffffffff000f0e0d0c01020304000000").unwrap();
    let req: Test = rkyv::from_bytes(&b).unwrap();
    println!("{:?}", req);
}

#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
#[archive_attr(derive(bytecheck::CheckBytes))]
struct Test {
    a: u64,
    e: TestE,
}

#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
#[archive_attr(derive(bytecheck::CheckBytes))]
enum TestE {
    Foo { b: [u8; 4], a: [u8; 4] },
    Bar { a: [u8; 4], b: [u8; 4] },
}
