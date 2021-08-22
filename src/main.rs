extern crate image;

use std::convert::TryFrom;

struct Pxhdr {
    version: u8,
    fmt: u8,
}
impl Pxhdr {
    fn pack(&self) -> [u8; 2] {
        [self.version, self.fmt]
    }
}

#[derive(Copy, Clone, Debug)]
struct PxFmt1 {
    x: u16,
    y: u16,
    r: u8,
    g: u8,
    b: u8,
}

impl PxFmt1 {
    fn pack(&self) -> Vec<u8> {
        let x = self.x.to_le_bytes();
        let y = self.y.to_le_bytes();
        vec![x[0], x[1], y[0], y[1], self.r, self.g, self.b]
    }
}

fn main() {
    use std::net::UdpSocket;
    let socket = UdpSocket::bind("[::]:6666").expect("couldn't bind to address");
    let header = Pxhdr { version: 0, fmt: 1 };
    let img = image::io::Reader::open("image.png")
        .unwrap()
        .decode()
        .unwrap();
    let flat_frame: Vec<_> = img
        .as_rgba8()
        .unwrap()
        .enumerate_pixels()
        .map(|(x, y, p)| PxFmt1 {
            x: u16::try_from(x).expect(""),
            y: u16::try_from(y).expect(""),
            r: p[0],
            g: p[1],
            b: p[2],
        })
        .collect();
    /*
    let mut frame = vec![
        vec![
            PxFmt1 {
                x: 0,
                y: 0,
                r: 100,
                g: 255,
                b: 100,
            };
            1080
        ];
        1920
    ];
    for x in 0..frame.len() {
        for y in 0..frame[x].len() {
            frame[x][y].x += u16::try_from(x).expect("");
            frame[x][y].y += u16::try_from(y).expect("");
        }
    }
    let flat_frame = frame.concat();
    */
    /*let buffer = [
        header.pack().to_vec(),
        frame
            .concat()
            .iter()
            .map(|p| p.pack())
            .collect::<Vec<[u8; 7]>>()
            .concat()
            .to_vec(),
    ]
    .concat();*/
    loop {
        let chunk_iter = flat_frame.chunks(160);
        for chunk in chunk_iter {
            let data = chunk
                .iter()
                .map(|p| p.pack())
                .collect::<Vec<Vec<u8>>>()
                .concat();
            let packet = [header.pack().to_vec(), data].concat();
            socket
                .send_to(packet.as_slice(), "[2001:678:9ec:3000::1]:5005")
                .expect("couldn't send data");
        }
    }
}
