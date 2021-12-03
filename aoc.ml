open Base

let () =
  let usage = "aoc: <problem> <part>" in
  let problem : string ref = ref "" in
  let part : string ref = ref "" in
  let anon =
    let calls = ref 0 in
    let anon s =
      calls := !calls + 1;
      if !calls = 1 then problem := s else part := s
    in
    anon
  in
  Caml.Arg.parse [] anon usage;
  match (!problem, !part) with
  | "1", "1" -> One.part1 ()
  | "1", "2" -> One.part2 ()
  | "2", "1" -> Two.part1 ()
  | "3", "1" -> Three.part1 ()
  | "3", "2" -> Three.part2 ()
  | problem, part ->
      failwith (Printf.sprintf "bad (problem,part): (%s,%s) " problem part)
