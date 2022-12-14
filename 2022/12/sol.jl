input = readchomp( "input.txt" )

map = collect.( split( input, "\n" ) )

function find_char( map, char )
    for (x, row ) ∈ enumerate( map )
        y = findfirst( char, String( row ) )
        if y !== nothing
            return ( x, y )
        end
    end
end

function get_height( map, pos )
    height = map[pos[begin]][pos[end]]
    height = height == 'S' ? 'a' :  height == 'E' ? 'z' : height
    return Int( height )
end

function BFS( map, start )
    fin_pos = find_char( map, 'E' )
    dst = fill( -1, length( map ), length( map[begin] ) )
    dst[start[begin], start[end]] = 0
    queue = [start]
    while !isempty( queue )
        pos = popfirst!( queue )
        for move ∈ [(1, 0), (-1, 0), (0, 1), (0, -1)]
            x = pos[begin] + move[begin]
            y = pos[end] + move[end]
            if !( checkbounds( Bool, map, x) && checkbounds( Bool, map[x], y ) ) ||
                get_height( map, (x, y) ) - get_height( map, pos )> 1
                continue
            end
            if dst[x, y] == -1
                dst[x, y] = dst[pos[begin], pos[end]] + 1
                push!( queue, ( x, y ) )
            end
        end
    end
    return dst[fin_pos[begin], fin_pos[end]] > -1 ? dst[fin_pos[begin], fin_pos[end]] : typemax( Int )
end

function trail( map )
    shortest = BFS( map, find_char( map, 'S' ) )
    println( "part1: ", shortest )
    for x ∈ 1:length( map )
        y = findnext( 'a', String( map[x] ), 1 )
        while( y !== nothing )
            shortest = min( shortest, BFS( map, (x, y) ) )
            y = findnext( 'a', String( map[x] ), y + 1 )
        end
    end
    return shortest
end

println( "part2: ", trail( map ) )
