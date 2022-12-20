using Distances

int( x ) = parse( Int, x )

input = readchomp( "input.txt" )

sensors = split( input, "\n" )

function get_row_range( y, scan; lower_bound = typemin( Int ), upper_bound = typemax( Int ), ignore_beacons = true)
    sensor, beacon = split( scan, ":" )
    s_pos = int.( match( r".*x=([-]?\d+), y=([-]?\d+).*", sensor ).captures )
    b_pos = int.( match( r".*x=([-]?\d+), y=([-]?\d+).*", beacon ).captures )
    dst = cityblock( s_pos, b_pos )
    min_x = max( s_pos[1] - dst + abs( s_pos[2] - y ), lower_bound )
    max_x = min( dst - abs( s_pos[2] - y ) + s_pos[1], upper_bound )
    if ( max_x < min_x )
        return (typemax( Int ), typemin( Int ))
    end
    if min_x == b_pos[1] && ignore_beacons
       min_x += 1
    end
    if max_x == b_pos[1] && ignore_beacons
       max_x -= 1
    end

    return (min_x, max_x)
end

function part1()
    ranges = map( s -> get_row_range( 2000000, s ), sensors )
    points = Set()
    for r ∈ ranges
        for x ∈ r[begin] : r[end]
            push!( points, x )
        end
    end
    return length( points )
end

function part2()
    for y ∈ 0:4000000
        ranges = map( s -> get_row_range( y, s; lower_bound = 0, upper_bound = 4000000, ignore_beacons = false ), sensors )
        filter!( x -> x[1] != typemax( Int ) && x[2] != typemin( Int ), ranges )
        sort!( ranges, by = x -> x[1] )
        prev_end = 0
        for range ∈ ranges
            if prev_end + 1 < range[1]
                 return( prev_end + 1, y )
            end
            prev_end = max( range[2], prev_end )
        end
    end
end

@time println( "part1: ", part1() )
@time println( "part2: ", part2() )
