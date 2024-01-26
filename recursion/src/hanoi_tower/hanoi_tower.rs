const NUM_DISKS: usize = 3;

fn move_disks(posts: &mut [[usize; NUM_DISKS]; 3],
              num_to_move: usize, from_post: usize, to_post: usize, temp_post: usize) {
    if num_to_move > 0 {
        move_disks(posts, num_to_move - 1, from_post, temp_post, to_post);
        move_disk(posts, from_post, to_post);
        move_disks(posts, num_to_move - 1, temp_post, to_post, from_post);
    }
}

// Move one disk from from_post to to_post.
fn move_disk(posts: &mut [[usize; NUM_DISKS]; 3], from_post: usize, to_post: usize) {
    draw_posts(posts);
    // Find the first non-empty row in from_post
    let mut fpos = 0;
    while (fpos < NUM_DISKS) && (posts[from_post][fpos] == 0) {
        fpos += 1;
    }

    // Find the last empty row in to_post
    let mut tpos = 0;
    while (tpos < NUM_DISKS) && (posts[to_post][tpos] == 0) {
        tpos += 1;
    }
    tpos -= 1;

    // Move the disk
    posts[to_post][tpos] = posts[from_post][fpos];
    posts[from_post][fpos] = 0;
}

// Draw the posts by showing the size of the disk at each level.
fn draw_posts(posts: &[[usize; NUM_DISKS]; 3]) {
    for i in 0..posts.len() {
        for j in 0..NUM_DISKS {
            print!("{:2}", posts[j][i]);
        }
        println!();
    }
    println!("------");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_disks() {
        // Make three posts with NUM_DISKS entries, all set to 0.
        let mut posts = [[0; NUM_DISKS]; 3];

        // Put the disks on the first post in order, smallest first (on top).
        for i in 0..NUM_DISKS {
            posts[0][i] = i + 1;
        }

        // Move the disks.
        move_disks(&mut posts, NUM_DISKS, 0, 2, 1);
        draw_posts(&posts);
        println!("Ok");
    }
}
