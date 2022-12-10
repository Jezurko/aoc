input = readchomp( "input.txt" )

struct pos
    x::Int
    y::Int
end

struct move
    dir::String
    len::Int
end

moves = Dict( "L" => pos( -1, 0 ), "R" => pos( 1, 0 ), "U" => pos( 0, 1 ), "D" => pos( 0, -1 ) )

move_head( dir, head ) = pos( head.x + moves[dir].x, head.y + moves[dir].y )

function move_tail( tail, head )
    x_dist = head.x - tail.x
    y_dist = head.y - tail.y
    if abs( x_dist ) > 1 || abs( y_dist ) > 1
        return pos( tail.x + clamp( x_dist, -1, 1 ), tail.y + clamp( y_dist, -1, 1 ) )
    end
    return tail
end

function count_tail_positions( num )
    positions = Set()
    rope = fill( pos(1,1), num )
    push!( positions, rope[end] )
    for line ∈ split( input, "\n" )
        splt = split( line, " " )
        m = move( splt[begin], parse( Int, splt[end] ) )
        for i ∈ 1:m.len
            rope[begin] = move_head( m.dir, rope[begin] )
            for knot ∈ 2:length( rope )
                rope[knot] = move_tail( rope[knot], rope[knot - 1] )
            end
            push!( positions, rope[end] )
        end
    end
    return length( positions )
end

println( count_tail_positions( 2 ) )
println( count_tail_positions( 10 ) )
