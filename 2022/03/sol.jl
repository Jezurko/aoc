function split_in_half( string )
    middle = length( string ) ÷ 2
    (string[begin : middle ], string[middle + 1 : end ])
end

get_priority( c ) = islowercase( c ) ? Int( c ) - Int( 'a' ) + 1 : Int( c ) - Int( 'A' ) + 27

score_common_char( strings ) = sum( get_priority, mapreduce( Set, ∩, strings ) )

sum_rucksacks() =
    sum( score_common_char.( split_in_half.( split( chomp( read( "input.txt", String ) ), "\n" ) ) ) )

read_tripples() =
    reshape( split( chomp( read( "input.txt", String ) ), "\n" ), 3, : )

sum_badges() =
    sum( score_common_char.( eachcol( read_tripples() ) ) )

println( "part1: ", sum_rucksacks() )
println( "part2: ", sum_badges() )
