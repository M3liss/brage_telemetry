fn main() {
2   let pt = 7;
3   let x = pt;
4   let y = pt; 
}

struct Beacon {
    lon: f32,
    lat: f32,
    distance: f32,
}

struct Beacon2 {
    x: f32,
    y: f32,
    z: f32,
    distance: f32,
}

struct Position {
    x: f32,
    y: f32,
    z: f32,
}

const ITERATIONS: usize = 15;

fn calc_pos (beacons &[Beacon2], current_pos : &mut Position) -> u8 {

    let mut num_useful_nodes = 0;
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_z = 0.0;
    
    // Make an initial guess Q that equals the average of all circle centers Cn
    for beacon in beacons {
        if (beacon.lon != 0.0 && beacon.lat != 0.0) {
            num_useful_nodes += 1;
            sum_x += beacon.x;
            sum_y += beacon.y;
            sum_z += beacon.z;
        }
    }
    
    // We need more than 3 locations for accuracy
    if (num_useful_nodes < 4) {
        return 0;
    }
    
    // Make an initial guess Q that equals the average of all circle centers Cn
    current_pos.x = sum_x / num_useful_nodes as f32;
    current_pos.y = sum_y / num_useful_nodes as f32;
    current_pos.z = sum_z / num_useful_nodes as f32;

    // Repeat for n iterations
    for n in 0..ITERATIONS {
        let step = Position {x:0.0, y:0.0, z:0.0};
    
        // For every circle center Cn and circle radii Rn, find a delta vector ΔQn that follows the formula:
        // ΔQn = normalize(Cn - Q) * (magnitude(Cn - Q) - Rn)
        for beacon in beacons {
            if (beacon.lon != 0.0 && beacon.lat != 0.0) {
                continue;
            }
            let beacon_pos = Position {x: beacon.x, y: beacon.y, z: beacon.z};
            let difference = Position {x: current_pos.x - beacon.x, 
                                        y: current_pos.y - beacon.y, 
                                        z: current_pos.z - beacon.z};
                                        
            // magnitude: √(x2 + y2 + z2)
            let magnitude = (difference.x * difference.x + difference.y * difference.y + difference.z * difference.z).sqrt();
            
            // ΔQn = normalize(Cn - Q) * (magnitude(Cn - Q) - Rn)
            let mut wanted_step = Position {x:0.0, y:0.0, z:0.0};
            wanted_step.x = (difference.x / magnitude) * (magnitude - beacon.distance);
            wanted_step.y = (difference.y / magnitude) * (magnitude - beacon.distance);
            wanted_step.z = (difference.z / magnitude) * (magnitude - beacon.distance);
            
            //Add the average of all the vectors in ΔQn to Q
            step.x += wanted_step.x / num_useful_nodes;
            step.y += wanted_step.y / num_useful_nodes;
            step.z += wanted_step.z / num_useful_nodes;
        }
        current_pos.x += step.x;
        current_pos.y += step.y;
        current_pos.z += step.z;
    }
    
}
