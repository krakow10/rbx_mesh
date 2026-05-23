#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum Revision7 {
	#[brw(magic = b"version 7.00")]
	Version700,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Mesh7 {
	pub revision: Revision7,
	#[br(temp)]
	#[bw(ignore)]
	#[brw(magic = b"\n")]
	_newline: (),
	#[br(temp)]
	#[bw(ignore)]
	#[brw(magic = b"COREMESH")]
	_coremesh: (),
	// <- 0x15
	pub unknown1_1: u32, // 2
	// These numbers are nearly the file length
	pub unknown1_2: u32, // 10181
	pub unknown1_3: u32, // 10177
	#[br(temp)]
	#[bw(ignore)]
	#[brw(magic = b"DRACO")]
	_draco: (),
	pub unknown3: [u8; 11],
	#[br(count = 804)]
	pub indices: Vec<u16>, // index into float_triples
	// <- 0x684
	pub unknown4: [u8; 32],
	#[br(count = 408)]
	pub float_triples: Vec<[f32; 3]>,
	// <- 0x19b9
	pub unknown5: [u8; 5],
	#[br(count = 290)]
	pub unknown6: Vec<u8>,
	// <- 0x1ae0
	// kinda gave up here, there may be more subdivisions in unknown7
	#[br(count = 3330)]
	pub unknown7: Vec<u8>,
	// <- 0x27E2
	#[br(temp)]
	#[bw(ignore)]
	#[brw(magic = b"LODS")]
	_lods: (),
	pub unknown8: [u8; 27],
}

#[test]
fn read_mesh7() {
	use binrw::BinReaderExt;
	let data = std::fs::read("meshes/mesh7_127279296594138.bin").unwrap();
	let mut bytes = std::io::Cursor::new(data.as_slice());
	let mesh: Mesh7 = bytes.read_le().unwrap();
	macro_rules! print_first_8_and_last_8 {
		($field:ident) => {
			println!(
				"{}: first = {:?} last = {:?}",
				stringify!($field),
				mesh.$field.get(0..8),
				mesh.$field.get(mesh.$field.len() - 8..)
			);
		};
	}
	println!("unknown1_1 = {:?}", mesh.unknown1_1);
	println!("unknown1_2 = {:?}", mesh.unknown1_2);
	println!("unknown1_3 = {:?}", mesh.unknown1_3);
	print_first_8_and_last_8!(indices);
	println!("unknown4 = {:?}", mesh.unknown4);
	println!("unknown5 = {:?}", mesh.unknown5);
	print_first_8_and_last_8!(unknown6);
	print_first_8_and_last_8!(unknown7);
	println!("unknown8 = {:?}", mesh.unknown8);
	println!("data.len() = {}", data.len());
	assert_eq!(data.len() as u64, bytes.position());
}
