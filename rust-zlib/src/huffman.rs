use std;
use self::HuffmanNode::{Node, Leaf};
use gz_reader::GzBitReader;
use cvec::Buf;


static CODE_LENGTH_OFFSETS: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];
static EXTRA_LENGTH_ADDEND: [usize; 20] = [
    11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227];
static EXTRA_DIST_ADDEND: [usize; 26] = [
    4, 6, 8, 12, 16, 24, 32, 48, 64, 96, 128, 192, 256, 384, 512, 768, 1024, 1536, 2048,
    3072, 4096, 6144, 8192, 12288, 16384, 24576];



#[derive(Clone, Show)]
struct HuffmanRange {
    end: u32,
    bit_length: u32,
}

impl HuffmanRange {
    fn new() -> HuffmanRange {
        HuffmanRange { end: 0, bit_length: 0 }
    }
}

#[derive(Show)]
struct TreeNode {
    len: usize,
    bits: usize,
    label: usize
}

#[derive(Show)]
enum HuffmanNode {
    Node(Option<Box<HuffmanNode>>, Option<Box<HuffmanNode>>),
    Leaf(u32)
}

impl HuffmanNode {
    fn read(&self, stream: &mut GzBitReader) -> Option<u32> {
        match self {
            &Leaf(v) => Some(v),
            &Node(ref left, ref right) => {
                let target = match try_opt!(stream.next_bit()) {
                    0 => try_ref_opt!(left),
                    1 => try_ref_opt!(right),
                    _ => { panic!("Bit greater than one, no bueno."); }
                };
                target.read(stream)
            }
        }
    }

    fn read_test(&self, bits: usize, len: usize) -> Option<u32> {
        match self {
            &Leaf(v) => {
                assert!(len == 0);
                Some(v)
            },
            &Node(ref left, ref right) => {
                let target = match get_bit(bits, len) {
                    0 => try_ref_opt!(left),
                    1 => try_ref_opt!(right),
                    _ => { panic!("Bit greater than one, no bueno."); }
                };
                target.read_test(bits, len - 1)
            }
        }
    }
}

fn build_huffman_tree(ranges: &Vec<HuffmanRange>)
    -> Option<HuffmanNode>
{
    let max_bit_length: usize = try_opt!(ranges.iter()
                                         .map(|x| x.bit_length)
                                         .max()) as usize;
    let bl_count = count_bitlengths(ranges, max_bit_length);
    let mut next_code = compute_first_codes(&bl_count, max_bit_length);
    let table: Vec<TreeNode> = compute_code_table(&mut next_code, ranges);
    let tree: HuffmanNode = build_tree(&table);
    Some(tree)
}

// determine number of codes of each bit-length
// returns a vector where the index corresponds to (bit_length - 1)
fn count_bitlengths(ranges: &Vec<HuffmanRange>, max_bit_length: usize) -> Vec<u32> {
    // Vec of size max_bit_length + 1, initialized to 0
    let mut bl_count: Vec<u32> = std::iter::repeat(0).take(max_bit_length).collect();

    let mut range_iter = ranges.iter();
    let mut old_range: &HuffmanRange = range_iter.next().unwrap();
    {
        if old_range.bit_length > 0 {
            let count_ref = bl_count.get_mut((old_range.bit_length - 1) as usize).unwrap();
            *count_ref += old_range.end + 1;
        }
    }

    for range in range_iter {
        if range.bit_length > 0 {
            let count_ref = bl_count.get_mut((range.bit_length - 1) as usize).unwrap();
            *count_ref += range.end - old_range.end;
        }
        old_range = range;
    }
    bl_count
}

// Figure out what the first code for each bit-length would be. This is one more than the last code
// of the previous bit length, left-shifted once.
// Returns a vector where the index corresponds to (bit_length - 1)
fn compute_first_codes(bl_count: &Vec<u32>, max_bit_length: usize) -> Vec<u32> {
    let mut ret = Vec::new();
    let mut code: u32 = 0;
    // from the RFC
    for bits in (0 .. bl_count.len()) {
        if bits > 1 {
            code = ( code + bl_count[bits - 1] ) << 1;
        }
        ret.push(if bl_count[bits] > 0 { code } else { 0 });
    }
    ret
}

// Assign codes to each symbol in the each range of a given bitlength
fn compute_code_table(next_code: &mut Vec<u32>, ranges: &Vec<HuffmanRange>)
    -> Vec<TreeNode>
{
    let mut ret = Vec::new();
    let mut active_range: usize = 0;
    let num_entries = ranges.get(ranges.len() - 1).unwrap().end;
    for n in 0 .. num_entries + 1 {
        if n > ranges[active_range].end {
            active_range += 1;
        }
        let mut tree = TreeNode { len: 0, bits: 0, label: 0 };
        let bit_length = ranges[active_range].bit_length as usize;
        if bit_length > 0 {
            tree.len = bit_length;
            tree.bits = next_code[bit_length - 1] as usize;
            tree.label = n as usize;
            *next_code.get_mut(bit_length - 1).unwrap() += 1;
            ret.push(tree);
        }
    }
    ret
}

fn build_tree(code_table: &Vec<TreeNode>) -> HuffmanNode {
    let mut root = Node(None, None);
    for (n, t_node) in code_table.iter().enumerate() {
        let bits = t_node.bits;
        let len = (t_node.len - 1) as isize;
        let label = t_node.label;
        make_tree(&mut root, bits, len, label);
    }
    root
}

fn make_tree(tree: &mut HuffmanNode, bits: usize, len: isize, label: usize) {
    match tree {
        &mut Leaf(_) => {
            panic!("This shouldn't have happened.");
        },
        &mut Node(ref mut left, ref mut right) => {
            match get_bit(bits, len as usize) {
                0 => { make_tree_side(left, bits, len - 1, label); },
                1 => { make_tree_side(right, bits, len - 1, label); },
                _ => { panic!("A bit was greater than 1, this is bad."); }
            }
        }
    }
}

fn make_tree_side(t_side: &mut Option<Box<HuffmanNode>>, bits: usize, len: isize, value: usize) {
    match t_side {
        &mut None => { *t_side = Some(box make_new_tree(bits, len, value)); },
        &mut Some(ref mut t) => { make_tree(&mut **t, bits, len, value); },
    };
}

fn make_new_tree(bits: usize, len: isize, value: usize) -> HuffmanNode {
    if len < 0 {
        Leaf(value as u32)
    } else {
        match get_bit(bits, len as usize) {
            0 => Node(Some(box make_new_tree(bits, len - 1, value)), None),
            1 => Node(None, Some(box make_new_tree(bits, len - 1, value))),
            _ => { panic!("A bit was greater than 1, this is bad."); }
        }
    }
}

// gets 'index' bit of input
fn get_bit(input: usize, len: usize) -> usize {
    if (input & (1 << len)) > 0 { 1 } else { 0 }
}

// Reads a huffman tree from a GzBitReader and returns two trees:
// the first is the literals tree, and the second is the distances tree
fn read_huffman_tree(stream: &mut GzBitReader) -> Option<(HuffmanNode, HuffmanNode)> {
    println!("read tree");
    let hlit = try_opt!(stream.read_bits(5));
    let hdist = try_opt!(stream.read_bits(5));
    let hclen = try_opt!(stream.read_bits(4)); // max of 15
    let mut code_length_ranges = Vec::new();
    let mut code_lengths = [0u32; 19];

    for i in 0 .. (hclen + 4) as usize {
        code_lengths[CODE_LENGTH_OFFSETS[i]] = try_opt!(stream.read_bits(3));
    }

    // make these ranges for the huffman tree routine
    let mut range = HuffmanRange::new();
    for i in (0 .. 19) {
        if i > 0 && code_lengths[i] != code_lengths[i-1] {
            code_length_ranges.push(range.clone());
        }
        range.end = i as u32;
        range.bit_length = code_lengths[i];
    }
    code_length_ranges.push(range.clone());
    let code_lengths_root = try_opt!(build_huffman_tree(&code_length_ranges));

    // now we read the literal/length alphabet, encoded with the huffman tree
    // we just built
    let mut i = 0;
    let mut alphabet: Vec<u32> = Vec::new();
    while i < (hlit + hdist + 258) {
        let code = try_opt!(code_lengths_root.read(stream));
        if code > 15 {
            let mut repeat_length = {
                if code == 16 {
                    try_opt!(stream.read_bits(2)) + 3
                } else if code == 17 {
                    try_opt!(stream.read_bits(3)) + 3
                } else if code == 18 {
                    try_opt!(stream.read_bits(7)) + 11
                } else { panic!("invalid code"); }
            } as i32;
            while repeat_length > 0 {
                if code == 16 {
                    let prev = *try_opt!(alphabet.get((i-1) as usize));
                    alphabet.push(prev);
                } else {
                    alphabet.push(0);
                }
                i += 1;
                repeat_length -= 1;
            }
        } else {
            alphabet.push(code);
            i += 1;
        }
    }

    // now alphabet lenths have been read, turn these into a range declaration and build
    // the final huffman code from it
    let mut literals_ranges = Vec::new();
    for i in 0 .. (hlit + 257) as usize {
        if i > 0 && alphabet[i] != alphabet[i-1] {
            literals_ranges.push(range.clone());
        }
        range.end = i as u32;
        range.bit_length = alphabet[i];
    };
    literals_ranges.push(range.clone());

    let mut distances_ranges = Vec::new();
    let dist_start = hlit + 257;
    for i in dist_start as usize .. (hdist + dist_start + 1) as usize {
        if i > dist_start as usize && alphabet[i] != alphabet[i-1] {
            distances_ranges.push(range.clone());
        }
        range.end = i as u32 - dist_start;
        range.bit_length = alphabet[i];
    }
    distances_ranges.push(range);

    let literals_root = try_opt!(build_huffman_tree(&literals_ranges));
    let distances_root = try_opt!(build_huffman_tree(&distances_ranges));
    Some((literals_root, distances_root))
}

fn build_fixed_huffman_tree() -> Option<HuffmanNode> {
    let ranges = vec![HuffmanRange { end: 143, bit_length: 8},
                      HuffmanRange { end: 255, bit_length: 9},
                      HuffmanRange { end: 279, bit_length: 7},
                      HuffmanRange { end: 287, bit_length: 8}];
    //build_huffman_tree(&ranges)
    Some(Leaf(0u32))
}

fn inflate_huffman_codes(stream: &mut GzBitReader,
                         literals_root: &HuffmanNode,
                         distances_root: Option<&HuffmanNode>,
                         out: &mut Buf)
    -> Option<()>
{
    println!("inflate codes");
    while let Some(code) = literals_root.read(stream) {
        println!("{:?}", stream);
        println!("looping");
        println!("{:?}", literals_root);
        return None;
        assert!(code < 286);
        if code < 256 {
            out.push(code as u8);
        } else if code == 256 { //stop code
            break;
        } else if code > 256 {
            let mut length;
            let mut dist;
            let mut extra_bits;
            if code < 265 {
                length = code - 254;
            } else {
                if code < 285 {
                    //println!("1");
                    extra_bits = try_opt!(stream.read_bits((code - 261) / 4));
                    length = extra_bits + EXTRA_LENGTH_ADDEND[(code - 265) as usize] as u32;
                } else { length = 256 }; // is this necessary?
            }

            // now, the length is followed by the distance back
            match distances_root {
                None => {
                    //println!("2");
                    dist = try_opt!(stream.read_bits(5)); // hardcoded distance
                },
                Some(distance_tree) => {
                    //println!("3");
                    dist = try_opt!(distance_tree.read(stream));
                }
            };

            if dist > 3 {
                //println!("4");
                let extra_dist = try_opt!(stream.read_bits((dist - 2) / 2));
                dist = extra_dist + EXTRA_DIST_ADDEND[(dist - 4) as usize] as u32;
            }
            out.copy_back_pointer(dist as usize, length as usize);
        }
    }
    println!("done");
    Some(())
}

// inflate() is called with a GzBitReader starting at the head of the first block
pub fn inflate(stream: &mut GzBitReader, out: &mut Buf) -> Option<()> {
    println!("inflate called");
    let fixed_tree = try_opt!(build_fixed_huffman_tree());
    let mut last_block = 0;
    while { last_block == 0 } {
        println!("getting block");
        last_block = try_opt!(stream.next_bit());
        let block_format = try_opt!(stream.read_bits(2));
        println!("got block");
        match block_format {
            0x00 => {
                println!("uncompressed block");
                // uncompressed block type, not supported
                return None;
            },
            0x01 => {
                // fixed tree
                try_opt!(inflate_huffman_codes(stream, &fixed_tree, None, out));
            },
            0x02 => {
                // dynamic tree
                let (literals_tree, distances_tree) = try_opt!(read_huffman_tree(stream));
                try_opt!(inflate_huffman_codes(stream, &literals_tree, Some(&distances_tree), out));
            }
            _ => {
                println!("unsupported block");
                // unsupported block type
                return None;
            }
        }
    }
    println!("{}", out.len());
    Some(())
}
