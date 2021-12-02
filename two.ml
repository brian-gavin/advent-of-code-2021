open Base
open Stdio

type command = Forward of int | Down of int | Up of int

let parse_command_ex s =
  let parse = function
    | [ "forward"; v ] -> Forward (Int.of_string v)
    | [ "down"; v ] -> Down (Int.of_string v)
    | [ "up"; v ] -> Up (Int.of_string v)
    | [ s; _ ] -> failwith ("bad command: " ^ s)
    | _ -> failwith "bad command format"
  in
  String.split ~on:' ' s |> parse

let collect () =
  In_channel.fold_lines In_channel.stdin ~init:[] ~f:(fun l s -> s :: l)
  |> List.rev_map ~f:parse_command_ex

type position = { horizontal : int; depth : int; aim : int }

let empty_position = { horizontal = 0; depth = 0; aim = 0 }

let handle_command { horizontal; depth; aim } = function
  | Forward x ->
      let horizontal = horizontal + x in
      let depth = depth + (aim * x) in
      { horizontal; depth; aim }
  | Down x ->
      let aim = aim + x in
      { horizontal; depth; aim }
  | Up x ->
      let aim = aim - x in
      { horizontal; depth; aim }

let part1 () =
  let solution { horizontal; depth; _ } = horizontal * depth in
  collect ()
  |> List.fold ~init:empty_position ~f:handle_command
  |> solution |> Int.to_string |> print_endline
