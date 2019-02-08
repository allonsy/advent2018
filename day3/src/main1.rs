use text_io::scan;
use text_io::try_scan;

struct Claim {
    id: u64,
    top_left: (u32, u32),
    width: u32,
    height: u32,
}

impl Claim {
    fn is_in_claim(&self, pt: (u32, u32)) -> bool {
        let (top_left_x, top_left_y) = self.top_left;
        let (x, y) = pt;
        if top_left_x <= x
            && x < top_left_x + self.width
            && top_left_y <= y
            && y < top_left_y + self.height
        {
            return true;
        }
        return false;
    }
}

fn main() {
    let claims = get_claims();
    let (max_x, max_y) = get_bounds(&claims);
    let mut num_overlap = 0;

    for row in 0..max_x {
        for col in 0..max_y {
            let mut has_been_in_claim = false;
            for claim in &claims {
                if claim.is_in_claim((row, col)) {
                    if has_been_in_claim == true {
                        num_overlap += 1;
                        break;
                    } else {
                        has_been_in_claim = true;
                    }
                }
            }
        }
    }

    println!("Number overlapping is: {}", num_overlap);
}

fn get_bounds(claims: &Vec<Claim>) -> (u32, u32) {
    let mut max_x = 0;
    let mut max_y = 0;
    for claim in claims {
        if claim.top_left.0 + claim.width > max_x {
            max_x = claim.top_left.0 + claim.width;
        }
        if claim.top_left.1 + claim.height > max_y {
            max_y = claim.top_left.1 + claim.height;
        }
    }
    (max_x, max_y)
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
        });
    }
    claims
}
