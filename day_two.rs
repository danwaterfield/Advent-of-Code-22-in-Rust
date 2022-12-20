fn main() {
   let input = include_str!("../../input/AO2_input.txt");
   let score: i32 = input
      .lines()
      .map(|line| {
         let bytes = line.as_bytes();
         let them = (bytes[0] - b'A') as i32;
         let me = (bytes[2] - b'X') as i32;

         let outcome = ((me - them).rem_euclid(3) + 1) % 3;
         outcome * 3 + me + 1
      })
      .sum();

      println!("Part one: {}", score);


      let score: i32 = input
      .lines()
      .map(|line| {
         let bytes = line.as_bytes();
         let them = (bytes[0] - b'A') as i32;
         let outcome: i32 = (bytes[2] - b'X') as i32;
      
         let me = (them + outcome + 2) % 3;

         outcome * 3 + me + 1
   
      })
   .sum();

println!("Part two: {}", score);

}
