use text_io::scan;
use text_io::try_scan;

#[derive(Clone)]
struct Claim {
    id: u64,
    top_left: (u32, u32),
    width: u32,
    height: u32,
    did_overlap: bool,
}

impl Claim {
    fn does_overlap(&self, claim: &Claim) -> bool {
        let min_x = self.top_left.0;
        let max_x = self.top_left.0 + self.width - 1;
        let c_min_x = claim.top_left.0;
        let c_max_x = claim.top_left.0 + claim.width - 1;

        if max_x < c_min_x || c_max_x < min_x {
            return false;
        }

        let min_y = self.top_left.1;
        let max_y = self.top_left.1 + self.height - 1;
        let c_min_y = claim.top_left.1;
        let c_max_y = claim.top_left.1 + claim.height - 1;

        if max_y < c_min_y || c_max_y < min_y {
            return false;
        }

        return true;
    }
}

fn main() {
    let mut claims = get_claims();
    let claims_len = claims.len();
    for i in 0..claims_len {
        let mut did_overlap = false;
        for j in (i + 1)..claims_len {
            if claims[i].does_overlap(&claims[j]) {
                claims[i].did_overlap = true;
                claims[j].did_overlap = true;
                did_overlap = true;
                break;
            }
        }
        if !did_overlap && !claims[i].did_overlap {
            println!("id is: {}", claims[i].id);
            return;
        }
    }
}

fn get_claims() -> Vec<Claim> {
    let lines = util::read_file_lines("input.txt");
    let mut claims = Vec::new();

    for line in lines {
        let id: u64;
        let top_left_x: u32;
        let top_left_y: u32;
        let width: u32;
        let height: u32;
        scan!(line.bytes() => "#{} @ {},{}: {}x{}", id, top_left_x, top_left_y, width, height);
        claims.push(Claim {
            id: id,
            top_left: (top_left_x, top_left_y),
            width: width,
            height: height,
            did_overlap: false,
        });
    }
    claims
}
