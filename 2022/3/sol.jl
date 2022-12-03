function find_letter( str )
    middle = div( length( str ), 2 )
    for (i, c) in enumerate( str[begin : middle] )
        if ( findnext( c, str, middle + 1 ) != nothing )
            return c
        end
    end
end

function find_tri_letter( strings )
    for c in strings[1]
        if ( occursin( string( c ), strings[2] ) && occursin( string( c ), strings[3] ) )
            return c
        end
    end
end

convert_char( c ) = islowercase( c ) ? Int( c ) - Int( 'a' ) + 1 : Int( c ) - Int( 'A' ) + 27

sum_rucksack() =
    sum( convert_char.( find_letter.( split( chomp( read( "input.txt", String ) ), "\n" ) ) ) )

read_tripples() =
    reshape( split( chomp( read( "input.txt", String ) ), "\n" ), 3, : )

sum_badges() =
    sum( convert_char.( find_tri_letter.( eachcol( read_tripples() ) ) ) )

println( "part1: ", sum_rucksack() )
println( "part2: ", sum_badges() )
