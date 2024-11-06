fn main() {
   let a = vec![1, 2];
   let b = vec![3, 4];
   let c = vec![5, 6];

   let combined: Vec<_> = a.iter().zip(b.iter()).zip(c.iter()).map(|((x, y), z)| (x, y, z)).collect();

   println!("{:?}", combined); // Output: [(1, 3, 5), (2, 4, 6)]
}