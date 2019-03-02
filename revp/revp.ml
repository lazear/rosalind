(* Take the first `n` items in a list *)
let take n xs = 
	let rec takerec n acc xs = match xs with
		| [] -> List.rev acc
		| hd::tl -> if n = 0 then List.rev acc else takerec (n - 1) (hd :: acc) tl
	in takerec n [] xs

(* Skip the first `n` items in a list *)
let rec skip n xs = match xs with
	| [] -> []
	| x :: xs -> if n = 0 then (x :: xs) else skip (n - 1) xs

(* Drop the last `n` items from a list *)
let drop n xs = 
	let rec inner n acc xs = match xs with
		| [] -> []
		| x :: xs -> if n = 0 then acc else inner (n -1) (x :: acc) xs
	in inner (List.length xs - n) [] xs

(* Filter a list *)
let rec filter p xs = match xs with 
	| [] -> []
	| x :: xs -> p x :: filter p xs

let enumerate xs  = 
	let rec inner n xs = match xs with
		| [] -> []
		| x :: xs -> (n, x) :: inner (n + 1) xs 
	in inner 0 xs




exception InvalidNucleotide of char 

let complement x = match x with
	| 'A' -> 'T'
	| 'C' -> 'G'
	| 'G' -> 'C'
	| 'T' -> 'A'
	| _ -> raise (InvalidNucleotide x)


let revcomp xs = List.rev_map complement xs

let str_to_list s = 
	let rec acc i xs = 
		if i < 0 then xs else acc (i - 1) (s.[i] :: xs) 
	in acc (String.length s - 1) []




let print_list lit xs = List.iter (fun x -> Printf.printf lit x) xs



(* Return the indices of the occurences of item in xs *)
let positions item xs = 
	let rec inner n pos ys = match ys with
		| [] -> pos
		| hd :: tl when hd == item ->  inner (n + 1) (n :: pos) tl
		| hd :: tl -> inner (n + 1) pos tl
	in inner 0 [] xs

(* Find palindromes with length between 4 and 9 in a list *)
let palindrome_search xs =
	let palindrome xs ys = List.for_all (fun x -> x == true) (List.map2 (=) xs ys)
	in let rec inner n = 
		if List.length xs < n then [] else 
		let seq = take n xs in 
		if seq == [] then [] else
		match n with
			| 12 -> palindrome seq (revcomp seq) :: []
			| _ -> palindrome seq (revcomp seq) :: inner (n + 1)
	in inner 4
	

let problem xs = 
	let max = List.length xs in
	let rec inner = function
		| n when n == max -> []
		| n -> palindrome_search (skip n xs) :: inner (n + 1)
	in inner 0

let solve xs =
	let rec inner = function
		| [] -> ()
		| hd::tl -> let indices = (positions true (snd hd)) in 
			(* Head is a tuple of (Str index * bool list) where the bool list represents
			 whether palindrome of length 4 - 12 exists in that slot *)
			let rec print_info = function
				| [] -> inner tl
				| x :: xs -> 
					Printf.printf "%d %d\n" ((fst hd) + 1) (x + 4);
				 	print_info xs
			 in print_info indices
	in inner (enumerate (problem xs))


let read_fasta f = 
	let ic = open_in f in 
	let try_read () = 
		try Some (input_line ic) with End_of_file -> None in 

	let rec loop acc = match try_read () with 
		| Some s -> loop (String.sub s 0 ((String.length s) - 1) :: acc)
		| None -> close_in ic; List.rev acc in 
	loop []

let () = 
	let seq = str_to_list (String.concat "" (skip 1 (read_fasta "rosalind_revp.txt"))) in
	solve seq;