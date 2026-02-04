pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    coordinates.len() < 3 || {
        let x0 = coordinates[0][0];
        let y0 = coordinates[0][1];
        let dx = coordinates[1][0] - x0;
        let dy = coordinates[1][1] - y0;

        coordinates[2..]
            .iter()
            .all(|c| (c[1] - y0) * dx == dy * (c[0] - x0))
    }
}
