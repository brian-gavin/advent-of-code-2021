open Stdio
open Base

type cell = { n : int; marked : bool }

(*
let debug_print b =
  Array.iter b ~f:(fun r ->
      Array.iter r ~f:(fun { marked; n } ->
          if marked then printf "\027[32m%d\027[0m " n else printf "%d " n);
      Stdio.print_endline "");
  Stdio.print_endline ""
*)

let draw t n =
  Array.map t
    ~f:(Array.map ~f:(fun c -> if n = c.n then { c with marked = true } else c))

let win t =
  let row_wins = Array.for_all ~f:(fun { marked; _ } -> marked) in
  let has_row_win = Array.exists ~f:row_wins in
  if has_row_win t then true else Array.transpose_exn t |> has_row_win

let unmarked =
  let row_unmarked =
    Array.filter_map ~f:(fun { n; marked } ->
        match marked with false -> Some n | true -> None)
  in
  Array.fold ~init:[] ~f:(fun a r ->
      List.append a (row_unmarked r |> Array.to_list))

let play draws boards =
  List.fold_until draws ~init:boards
    ~f:(fun boards n ->
      let boards = List.map boards ~f:(fun b -> draw b n) in
      match List.find boards ~f:win with
      | Some b -> Stop (Some (b, n))
      | None -> Continue boards)
    ~finish:(fun _ -> None)

let to_row s =
  String.split s ~on:' '
  |> List.filter ~f:(fun s -> not (String.equal s ""))
  |> Array.of_list_map ~f:(fun s -> { n = Int.of_string s; marked = false })

let fold_boards () =
  let module Ic = In_channel in
  let folder (boards, board) = function
    | "" -> (board :: boards, [||])
    | s -> (boards, Array.append board [| to_row s |])
  in
  let boards, last = Ic.fold_lines Ic.stdin ~init:([], [||]) ~f:folder in
  last :: boards

let collect () =
  let module Ic = In_channel in
  let draws =
    String.split ~on:',' (Ic.input_line_exn Ic.stdin)
    |> List.map ~f:Int.of_string
  in
  let _skip = Ic.input_line_exn Ic.stdin in
  (draws, fold_boards ())

let sum = List.fold ~init:0 ~f:( + )

let part1 () =
  let draws, boards = collect () in
  match play draws boards with
  | Some (winner, n) ->
      Stdio.print_endline (Int.to_string n);
      sum (unmarked winner) * n |> Int.to_string |> Stdio.print_endline
  | None -> failwith "no winner?"

let part2 () = ()
