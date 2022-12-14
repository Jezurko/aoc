function play(x, y)
    res = mod1( x - y, 3 )
    res != 2 ? res == 1 ? 2 : 1 : 0
end

find(x, y) = y == 0 ? mod1( x, 3 ) - 1 : y == 1 ? x : ( x + 1 ) % 3

function rockpaperscissors()
    strategy = map.( x -> ( Int( x[begin] ) - 64 ) % 23 - 1,
                    split.( split( chomp( read( "input.txt", String ) ), "\n" ), " " ) )
    results = map( x -> x[end] + 1 + 3 * play( x[end], x[begin] ), strategy )
    alt_results = map( x -> x[end] * 3 + find( x[begin], x[end] ) + 1, strategy )
    println( "part1: ", sum( results ) )
    println( "part2: ", sum( alt_results ) )
end

rockpaperscissors()
