open Base
open Stdio

(* sample word length *)
(* let n = 5 *)

(* input word length *)
let n = 12

let rates =
  List.fold ~init:(0, 0) ~f:(fun (g, e) (zc, oc) ->
      let most_common, least_common = if zc > oc then (0, 1) else (1, 0) in
      ((g lsl 1) lor most_common, (e lsl 1) lor least_common))

let part1 () =
  let counts = Array.init n ~f:(fun _ -> (0, 0)) in
  In_channel.iter_lines In_channel.stdin ~f:(fun s ->
      List.iteri (String.to_list s) ~f:(fun i c ->
          let zeroes, ones = counts.(i) in
          match c with
          | '0' -> counts.(i) <- (zeroes + 1, ones)
          | '1' -> counts.(i) <- (zeroes, ones + 1)
          | _ -> failwith "bad bit"));
  let gamma, epsilon = rates (Array.to_list counts) in
  print_endline (Int.to_string (gamma * epsilon))

let part2 () = ()