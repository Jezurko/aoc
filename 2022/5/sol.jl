input = split( readchomp( "input.txt" ), "\n\n" )
function crates()
    parsed = collect.( split( replace( input[begin],
                                       "[" => "", " [" => "", "]" => "" , "    " => "_" ), "\n" ) )
    pop!( parsed )
    reverse.( filter.( x-> x != '_', [ c[:] for c in eachrow( reduce( hcat, parsed ) ) ] ) )
end
moves() = map( x->parse.( Int, x ),
              split.( replace.( split( input[end], "\n"),
                                "move " => "", " from" => "", " to" => "" ), " " ) )

function move_crates()
    c = crates()
    for move in moves()
        for _ in 1:move[1]
            push!( c[move[3]], pop!( c[move[2]] ) )
        end
    end
    return c
end

function move_crates2()
    c = crates()
    for move in moves()
        push!( c[move[3]],
               splice!( c[move[2]],
                       length( c[move[2]] ) - move[1] + 1:length( c[move[2]] ), [] )... )
    end
    return c
end
get_tops( arrs ) = [ arr[end] for arr in arrs ]

println( String( get_tops( move_crates() ) ) )
println( String( get_tops( move_crates2() ) ) )
