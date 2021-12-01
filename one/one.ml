open Base
open Stdio

let collect () =
  In_channel.fold_lines In_channel.stdin ~init:[] ~f:(fun l s -> s :: l)
  |> List.rev_map ~f:Int.of_string

let num_increases = function
  | hd :: tl ->
      List.fold tl ~init:(0, hd) ~f:(fun (cnt, last) n ->
          if n > last then (cnt + 1, n) else (cnt, n))
      |> fst
  | [] -> 0

let part1 () = collect () |> num_increases |> Int.to_string |> print_endline

let windows l =
  let rec f windows = function
    | [] | [ _ ] | [ _; _ ] -> windows
    | l -> f (List.take l 3 :: windows) (List.tl_exn l)
  in
  f [] l |> List.rev

let sum = List.fold ~init:0 ~f:( + )

let part2 () =
  collect () |> windows |> List.map ~f:sum |> num_increases |> Int.to_string
  |> print_endline

let () = part2 ()

(*
let debug_print l =
  let inner l =
    "[" ^ String.concat ~sep:"," (List.map l ~f:Int.to_string) ^ "]"
  in
  print_endline ("[" ^ String.concat ~sep:"\n" (List.map l ~f:inner) ^ "]");
  l
*)
